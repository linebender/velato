// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::Composition;
use super::model::{Content, Draw, Geometry, GroupTransform, Layer, Shape, fixed};
use kurbo::{
    Affine, BezPath, CubicBez, Line, ParamCurve, ParamCurveArclen, PathEl, PathSeg, Point, QuadBez,
    Rect,
};
use peniko::{Fill, Mix};
use std::mem::swap;
use std::ops::Range;

pub trait RenderSink {
    fn push_layer(
        &mut self,
        blend: impl Into<peniko::BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl kurbo::Shape,
    );

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl kurbo::Shape);

    fn pop_layer(&mut self);

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl kurbo::Shape,
    );
}

/// Renders a composition into a scene.
#[derive(Debug, Default)]
pub struct Renderer {
    batch: Batch,
    mask_elements: Vec<PathEl>,
}

impl Renderer {
    /// Creates a new renderer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Renders and appends the animation at a given frame to the provided scene.
    pub fn append(
        &mut self,
        animation: &Composition,
        frame: f64,
        transform: Affine,
        alpha: f64,
        scene: &mut impl RenderSink,
    ) {
        self.batch.clear();
        scene.push_clip_layer(
            transform,
            &Rect::new(0.0, 0.0, animation.width as _, animation.height as _),
        );
        for layer in animation.layers.iter().rev() {
            if layer.is_mask {
                continue;
            }
            self.render_layer(
                animation,
                &animation.layers,
                layer,
                transform,
                alpha,
                frame,
                scene,
            );
        }
        scene.pop_layer();
    }

    #[expect(clippy::too_many_arguments, reason = "Deferred")]
    fn render_layer(
        &mut self,
        animation: &Composition,
        layer_set: &[Layer],
        layer: &Layer,
        transform: Affine,
        alpha: f64,
        frame: f64,
        scene: &mut impl RenderSink,
    ) {
        if !layer.frames.contains(&frame) {
            return;
        }
        let parent_transform = transform;
        let transform = self.compute_transform(layer_set, layer, parent_transform, frame);
        let full_rect = Rect::new(0.0, 0.0, animation.width as f64, animation.height as f64);
        if let Some((mode, mask_index)) = layer.mask_layer {
            // todo: re-enable masking when it is more understood (and/or if
            // it's currently supported in vello?) Extra layer to
            // isolate blending for the mask
            scene.push_layer(Mix::Normal, 1.0, parent_transform, &full_rect);
            if let Some(mask) = layer_set.get(mask_index) {
                self.render_layer(
                    animation,
                    layer_set,
                    mask,
                    parent_transform,
                    alpha,
                    frame,
                    scene,
                );
            }
            scene.push_layer(mode, 1.0, parent_transform, &full_rect);
        }
        let alpha = alpha * layer.opacity.evaluate(frame) / 100.0;
        for mask in &layer.masks {
            mask.geometry.evaluate(frame, &mut self.mask_elements);
            scene.push_clip_layer(transform, &self.mask_elements.as_slice());
            self.mask_elements.clear();
        }
        match &layer.content {
            Content::None => {}
            Content::Instance {
                name,
                time_remap: _,
            } => {
                // TODO: Use time_remap
                // let frame = time_remap
                //     .as_ref()
                //     .map(|tm| tm.evaluate(frame))
                //     .unwrap_or(frame);
                if let Some(asset_layers) = animation.assets.get(name) {
                    let frame = frame / layer.stretch;
                    let frame_delta = -layer.start_frame / layer.stretch;
                    for asset_layer in asset_layers.iter().rev() {
                        if asset_layer.is_mask {
                            continue;
                        }
                        self.render_layer(
                            animation,
                            asset_layers,
                            asset_layer,
                            transform,
                            alpha,
                            frame + frame_delta,
                            scene,
                        );
                    }
                }
            }
            Content::Shape(shapes) => {
                self.render_shapes(shapes, transform, alpha, frame);
                self.batch.render(scene);
                self.batch.clear();
            }
        }
        for _ in 0..layer.masks.len() + (layer.mask_layer.is_some() as usize * 2) {
            scene.pop_layer();
        }
    }

    fn render_shapes(&mut self, shapes: &[Shape], transform: Affine, alpha: f64, frame: f64) {
        // Keep track of our local top of the geometry stack. Any subsequent
        // draws are bounded by this.
        let geometry_start = self.batch.geometries.len();
        // Also keep track of top of draw stack for repeater evaluation.
        let draw_start = self.batch.draws.len();
        // Top to bottom, collect geometries and draws.
        for shape in shapes {
            match shape {
                Shape::Group(shapes, group_transform) => {
                    let (group_transform, group_alpha) =
                        if let Some(GroupTransform { transform, opacity }) = group_transform {
                            (
                                transform.evaluate(frame).into_owned(),
                                opacity.evaluate(frame) / 100.0,
                            )
                        } else {
                            (Affine::IDENTITY, 1.0)
                        };
                    self.render_shapes(
                        shapes,
                        transform * group_transform,
                        alpha * group_alpha,
                        frame,
                    );
                }
                Shape::Geometry(geometry) => {
                    self.batch.push_geometry(geometry, transform, frame);
                }
                Shape::Draw(draw) => {
                    self.batch.push_draw(draw, alpha, geometry_start, frame);
                }
                Shape::Repeater(repeater) => {
                    let repeater = repeater.evaluate(frame);
                    self.batch
                        .repeat(repeater.as_ref(), geometry_start, draw_start);
                }
                Shape::Trim(trim) => {
                    let trim = trim.evaluate(frame);
                    self.batch.apply_trim(trim.as_ref(), geometry_start);
                }
            }
        }
    }

    /// Computes the transform for a single layer. This currently chases the
    /// full transform chain each time. If it becomes a bottleneck, we can
    /// implement caching.
    fn compute_transform(
        &self,
        layer_set: &[Layer],
        layer: &Layer,
        global_transform: Affine,
        frame: f64,
    ) -> Affine {
        let mut transform = layer.transform.evaluate(frame).into_owned();
        let mut parent_index = layer.parent;
        let mut count = 0_usize;
        while let Some(index) = parent_index {
            // We don't check for cycles at import time, so this heuristic
            // prevents infinite loops.
            if count >= layer_set.len() {
                break;
            }
            if let Some(parent) = layer_set.get(index) {
                parent_index = parent.parent;
                transform = parent.transform.evaluate(frame).into_owned() * transform;
                count += 1;
            } else {
                break;
            }
        }
        global_transform * transform
    }
}

#[derive(Clone, Debug)]
struct DrawData {
    stroke: Option<fixed::Stroke>,
    brush: fixed::Brush,
    alpha: f64,
    /// Range into `ShapeBatch::geometries`
    geometry: Range<usize>,
}

impl DrawData {
    fn new(draw: &Draw, alpha: f64, geometry: Range<usize>, frame: f64) -> Self {
        Self {
            stroke: draw
                .stroke
                .as_ref()
                .map(|stroke| stroke.evaluate(frame).into_owned()),
            brush: draw.brush.evaluate(1.0, frame).into_owned(),
            alpha: alpha * draw.opacity.evaluate(frame) / 100.0,
            geometry,
        }
    }
}

#[derive(Clone, Debug)]
struct GeometryData {
    /// Range into `ShapeBatch::elements`
    elements: Range<usize>,
    transform: Affine,
}

#[derive(Debug, Default)]
struct Batch {
    elements: Vec<PathEl>,
    geometries: Vec<GeometryData>,
    draws: Vec<DrawData>,
    repeat_geometries: Vec<GeometryData>,
    repeat_draws: Vec<DrawData>,
    /// Length of geometries at time of most recent draw. This is
    /// used to prevent merging into already used geometries.
    drawn_geometry: usize,
    trim_elements: Vec<PathEl>,
}

impl Batch {
    fn push_geometry(&mut self, geometry: &Geometry, transform: Affine, frame: f64) {
        // Merge with the previous geometry if possible. There are two
        // conditions:
        // 1. The previous geometry has not yet been referenced by a draw
        // 2. The geometries have the same transform
        if self.drawn_geometry < self.geometries.len()
            && self.geometries.last().map(|last| last.transform) == Some(transform)
        {
            geometry.evaluate(frame, &mut self.elements);
            self.geometries.last_mut().unwrap().elements.end = self.elements.len();
        } else {
            let start = self.elements.len();
            geometry.evaluate(frame, &mut self.elements);
            let end = self.elements.len();
            self.geometries.push(GeometryData {
                elements: start..end,
                transform,
            });
        }
    }

    fn push_draw(&mut self, draw: &Draw, alpha: f64, geometry_start: usize, frame: f64) {
        self.draws.push(DrawData::new(
            draw,
            alpha,
            geometry_start..self.geometries.len(),
            frame,
        ));
        self.drawn_geometry = self.geometries.len();
    }

    fn repeat(&mut self, repeater: &fixed::Repeater, geometry_start: usize, draw_start: usize) {
        // First move the relevant ranges of geometries and draws into side
        // buffers
        self.repeat_geometries
            .extend(self.geometries.drain(geometry_start..));
        self.repeat_draws.extend(self.draws.drain(draw_start..));
        // Next, repeat the geometries and apply the offset transform
        for geometry in self.repeat_geometries.iter() {
            for i in 0..repeater.copies {
                let transform = repeater.transform(i);
                let mut geometry = geometry.clone();
                geometry.transform *= transform;
                self.geometries.push(geometry);
            }
        }
        // Finally, repeat the draws, taking into account opacity and the
        // modified newly repeated geometry ranges
        let start_alpha = repeater.start_opacity / 100.0;
        let end_alpha = repeater.end_opacity / 100.0;
        let delta_alpha = if repeater.copies > 1 {
            // See note in Skottie: AE does not cover the full opacity range
            (end_alpha - start_alpha) / repeater.copies as f64
        } else {
            0.0
        };
        for i in 0..repeater.copies {
            let alpha = start_alpha + delta_alpha * i as f64;
            if alpha <= 0.0 {
                continue;
            }
            for mut draw in self.repeat_draws.iter().cloned() {
                draw.alpha *= alpha;
                let count = draw.geometry.end - draw.geometry.start;
                draw.geometry.start =
                    geometry_start + (draw.geometry.start - geometry_start) * repeater.copies;
                draw.geometry.end = draw.geometry.start + count * repeater.copies;
                self.draws.push(draw);
            }
        }
        // Clear the side buffers
        self.repeat_geometries.clear();
        self.repeat_draws.clear();
        // Prevent merging until new geometries are pushed
        self.drawn_geometry = self.geometries.len();
    }

    fn apply_trim(&mut self, trim: &fixed::Trim, geometry_start: usize) {
        let Some((first, second)) = trim.normalized() else {
            for geometry in &mut self.geometries[geometry_start..] {
                geometry.elements = 0..0;
            }
            return;
        };

        if first.0 <= 1e-9 && first.1 >= 1.0 - 1e-9 && second.is_none() {
            return;
        }

        for geometry in &self.geometries[..geometry_start] {
            self.trim_elements
                .extend(self.elements[geometry.elements.clone()].iter().cloned());
        }

        const ACCURACY: f64 = 0.1;

        for geometry in &mut self.geometries[geometry_start..] {
            let path: BezPath = self.elements[geometry.elements.clone()]
                .iter()
                .cloned()
                .collect();
            let segs: Vec<PathSeg> = path.segments().collect();
            let total_length: f64 = segs.iter().map(|s| s.arclen(ACCURACY)).sum();

            if total_length <= 0.0 || segs.is_empty() {
                let pos = self.trim_elements.len();
                geometry.elements = pos..pos;
                continue;
            }

            let trim_ranges = [Some(first), second];
            let mut trimmed = BezPath::new();
            for range in trim_ranges.into_iter().flatten() {
                let (norm_start, norm_end) = range;
                let start_len = norm_start * total_length;
                let end_len = norm_end * total_length;
                let mut current_len = 0.0;
                let mut need_move = true;

                for seg in &segs {
                    let seg_len = seg.arclen(ACCURACY);
                    let seg_end = current_len + seg_len;

                    if seg_end < start_len - 1e-9 {
                        current_len = seg_end;
                        continue;
                    }
                    if current_len > end_len + 1e-9 {
                        break;
                    }

                    let t_start = if current_len < start_len {
                        seg.inv_arclen(start_len - current_len, ACCURACY)
                            .clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    let t_end = if seg_end > end_len {
                        seg.inv_arclen(end_len - current_len, ACCURACY)
                            .clamp(0.0, 1.0)
                    } else {
                        1.0
                    };

                    if t_end > t_start + 1e-9 {
                        let sub = seg.subsegment(t_start..t_end);
                        if need_move {
                            trimmed.move_to(sub.start());
                            need_move = false;
                        }
                        match sub {
                            PathSeg::Line(l) => trimmed.line_to(l.p1),
                            PathSeg::Quad(q) => trimmed.quad_to(q.p1, q.p2),
                            PathSeg::Cubic(c) => trimmed.curve_to(c.p1, c.p2, c.p3),
                        }
                    }
                    current_len = seg_end;
                }
            }

            let new_start = self.trim_elements.len();
            self.trim_elements
                .extend(trimmed.elements().iter().cloned());
            geometry.elements = new_start..self.trim_elements.len();
        }

        let mut offset = 0;
        for geometry in &mut self.geometries[..geometry_start] {
            let len = geometry.elements.len();
            geometry.elements = offset..offset + len;
            offset += len;
        }

        swap(&mut self.elements, &mut self.trim_elements);
        self.trim_elements.clear();
    }

    fn render(&self, scene: &mut impl RenderSink) {
        // Process all draws in reverse
        for draw in self.draws.iter().rev() {
            // Some nastiness to avoid cloning the brush if unnecessary
            let modified_brush = if draw.alpha != 1.0 {
                Some(draw.brush.clone().multiply_alpha(draw.alpha as _))
            } else {
                None
            };
            let brush = modified_brush.as_ref().unwrap_or(&draw.brush);
            for geometry in self.geometries[draw.geometry.clone()].iter() {
                let path = &self.elements[geometry.elements.clone()];
                let transform = geometry.transform;
                scene.draw(draw.stroke.as_ref(), transform, brush, &path);
            }
        }
    }

    fn clear(&mut self) {
        self.elements.clear();
        self.geometries.clear();
        self.draws.clear();
        self.repeat_geometries.clear();
        self.repeat_draws.clear();
        self.drawn_geometry = 0;
        self.trim_elements.clear();
    }
}
