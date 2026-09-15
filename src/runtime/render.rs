// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::model::{
    Content, Draw, Geometry, GroupTransform, ImageAsset, RepeaterComposite, Shape, fixed,
};
use super::{Composition, EvaluatedLayer, EvaluationError, FilterEffect, FilterLayerResult};
use kurbo::{Affine, PathEl, Rect};
use peniko::Mix;
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

    fn push_filter(&mut self, _transform: Affine, _effect: &FilterEffect) -> FilterLayerResult {
        FilterLayerResult::Unsupported
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl kurbo::Shape,
    );

    /// Draws a previously loaded raster image asset.
    fn draw_image(&mut self, image: &ImageAsset, transform: Affine, alpha: f64);

    /// Called before rendering a Lottie layer.
    ///
    /// - `name` is the layer's `nm` field.
    /// - `index` is the layer's position in the layer array.
    fn begin_layer_group(&mut self, _name: &str, _index: usize) {}

    /// Called after rendering a Lottie layer.
    fn end_layer_group(&mut self) {}
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
    ///
    /// Panics on evaluation errors. Use [`Self::try_append`] to handle them explicitly.
    ///
    /// Repeater expansion is unbounded. Callers handling untrusted animations should validate copy
    /// counts and nesting, and benchmark representative workloads to establish their own limits.
    pub fn append(
        &mut self,
        animation: &Composition,
        frame: f64,
        transform: Affine,
        alpha: f64,
        scene: &mut impl RenderSink,
    ) {
        self.try_append(animation, frame, transform, alpha, scene)
            .expect("Lottie hierarchy evaluation failed");
    }

    /// Renders a frame, leaving the sink unchanged if evaluation fails.
    pub fn try_append(
        &mut self,
        animation: &Composition,
        frame: f64,
        transform: Affine,
        alpha: f64,
        scene: &mut impl RenderSink,
    ) -> Result<(), EvaluationError> {
        let evaluated = animation.evaluate(frame)?;
        self.batch.clear();
        let clip = Rect::new(0.0, 0.0, animation.width as _, animation.height as _);
        let clip_bounds = transform.transform_rect_bbox(clip);
        scene.push_clip_layer(transform, &clip);
        for (layer_index, layer) in evaluated.layers.iter().enumerate().rev() {
            if !layer.visible {
                continue;
            }
            self.render_layer(
                animation,
                &evaluated.layers,
                layer,
                layer_index,
                transform,
                alpha,
                &clip_bounds,
                scene,
            );
        }
        scene.pop_layer();
        Ok(())
    }

    #[expect(clippy::too_many_arguments, reason = "Deferred")]
    fn render_layer(
        &mut self,
        animation: &Composition,
        layer_set: &[EvaluatedLayer<'_>],
        evaluated: &EvaluatedLayer<'_>,
        layer_index: usize,
        output_transform: Affine,
        alpha: f64,
        clip_bounds: &Rect,
        scene: &mut impl RenderSink,
    ) {
        if !evaluated.in_range {
            return;
        }
        let layer = evaluated.layer;
        let frame = evaluated.frame;
        scene.begin_layer_group(&layer.name, layer_index);
        let parent_transform = output_transform;
        let transform = output_transform * evaluated.full_transform;
        let full_rect = Rect::new(0.0, 0.0, animation.width as f64, animation.height as f64);
        if let Some((mode, mask_index)) = evaluated.matte {
            scene.push_layer(Mix::Normal, 1.0, parent_transform, &full_rect);
            self.render_layer(
                animation,
                layer_set,
                &layer_set[mask_index],
                mask_index,
                output_transform,
                alpha,
                clip_bounds,
                scene,
            );
            scene.push_layer(mode, 1.0, parent_transform, &full_rect);
        }
        let alpha = alpha * evaluated.opacity;

        // NOTE: important for isolating opacity so it applies to the layer effect output.
        let isolate_opacity = !layer.effects.is_empty() && alpha != 1.0;

        if isolate_opacity {
            scene.push_layer(Mix::Normal, alpha as f32, parent_transform, &full_rect);
        }

        let alpha = if isolate_opacity { 1.0 } else { alpha };
        let mut filter_layers = usize::from(isolate_opacity);

        for effect in layer
            .effects
            .iter()
            .rev()
            .filter_map(|effect| effect.evaluate(frame))
        {
            match scene.push_filter(transform, &effect) {
                FilterLayerResult::Pushed => filter_layers += 1,
                FilterLayerResult::Unsupported => {}
            }
        }
        for mask in &layer.masks {
            mask.geometry.evaluate(frame, &mut self.mask_elements);
            scene.push_clip_layer(transform, &self.mask_elements.as_slice());
            self.mask_elements.clear();
        }
        match &layer.content {
            Content::None => {}
            Content::Instance { .. } => {
                for (index, child) in evaluated.children.iter().enumerate().rev() {
                    if child.layer.is_matte_source || child.layer.hidden {
                        continue;
                    }
                    self.render_layer(
                        animation,
                        &evaluated.children,
                        child,
                        index,
                        output_transform,
                        alpha,
                        clip_bounds,
                        scene,
                    );
                }
            }
            Content::Image { asset_id } => {
                if let Some(image) = animation.images.get(asset_id) {
                    // Lottie requires visuals to stay within authored image bounds.
                    let bounds = image
                        .width
                        .zip(image.height)
                        .filter(|(w, h)| *w > 0.0 && *h > 0.0);
                    if let Some((width, height)) = bounds {
                        scene.push_clip_layer(transform, &Rect::new(0.0, 0.0, width, height));
                    }
                    scene.draw_image(image, transform, alpha);
                    if bounds.is_some() {
                        scene.pop_layer();
                    }
                }
            }
            Content::Shape(shapes) => {
                self.render_shapes(shapes, transform, alpha, frame);
                self.batch.render(scene, clip_bounds);
                self.batch.clear();
            }
        }
        for _ in 0..layer.masks.len() + filter_layers + (evaluated.matte.is_some() as usize * 2) {
            scene.pop_layer();
        }
        scene.end_layer_group();
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
                    self.render_shapes(shapes, group_transform, alpha * group_alpha, frame);
                }
                Shape::Geometry(geometry) => {
                    self.batch.push_geometry(geometry, Affine::IDENTITY, frame);
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
        for geometry in &mut self.batch.geometries[geometry_start..] {
            geometry.transform = transform * geometry.transform;
        }
        for draw in &mut self.batch.draws[draw_start..] {
            draw.transform = transform * draw.transform;
        }
    }
}

#[derive(Clone, Debug)]
struct DrawData {
    stroke: Option<fixed::Stroke>,
    brush: fixed::Brush,
    alpha: f64,
    transform: Affine,
    /// Range into `ShapeBatch::geometries`
    geometry: Range<usize>,
    // Only repeaters that copied this style may share its opacity layers with other styles.
    repeater_depth: usize,
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
            transform: Affine::IDENTITY,
            geometry,
            repeater_depth: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct GeometryData {
    /// Range into `ShapeBatch::elements`
    elements: Range<usize>,
    transform: Affine,
    // Sequential trimming follows copy indices independently of paint stacking.
    trim_order: usize,
    // Inherited styles need copy opacity without inheriting the repeated styles' stacking position.
    copies: Vec<CopyLayer>,
}

impl GeometryData {
    fn opacity_copies(&self) -> &[CopyLayer] {
        // Opaque ancestors still distinguish separate instances of translucent descendants.
        let end = self
            .copies
            .iter()
            .rposition(|copy| copy.alpha != 1.0)
            .map_or(0, |index| index + 1);
        &self.copies[..end]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CopyLayer {
    id: usize,
    alpha: f64,
}

#[derive(Debug, Default)]
struct Batch {
    elements: Vec<PathEl>,
    geometries: Vec<GeometryData>,
    draws: Vec<DrawData>,
    repeat_geometries: Vec<GeometryData>,
    repeat_draws: Vec<DrawData>,
    next_copy: usize,
    trim_elements: Vec<PathEl>,
}

impl Batch {
    fn push_geometry(&mut self, geometry: &Geometry, transform: Affine, frame: f64) {
        let start = self.elements.len();
        geometry.evaluate(frame, &mut self.elements);
        let end = self.elements.len();
        self.geometries.push(GeometryData {
            elements: start..end,
            transform,
            trim_order: self.geometries.len(),
            copies: Vec::new(),
        });
    }

    fn push_draw(&mut self, draw: &Draw, alpha: f64, geometry_start: usize, frame: f64) {
        self.draws.push(DrawData::new(
            draw,
            alpha,
            geometry_start..self.geometries.len(),
            frame,
        ));
    }

    fn repeat(&mut self, repeater: &fixed::Repeater, geometry_start: usize, draw_start: usize) {
        if repeater.copies == 0 {
            self.geometries.truncate(geometry_start);
            self.draws.truncate(draw_start);
            return;
        }
        if geometry_start == self.geometries.len() {
            return;
        }

        // First move the relevant ranges of geometries and draws into side
        // buffers
        self.repeat_geometries
            .extend(self.geometries.drain(geometry_start..));
        self.repeat_draws.extend(self.draws.drain(draw_start..));
        let geometry_count = self.repeat_geometries.len();
        let start_alpha = repeater.start_opacity / 100.0;
        let end_alpha = repeater.end_opacity / 100.0;
        let delta_alpha = if repeater.copies > 1 {
            (end_alpha - start_alpha) / (repeater.copies - 1) as f64
        } else {
            0.0
        };
        // Later styles traverse geometry directly, so it must be in paint order too.
        for copy in 0..repeater.copies {
            let i = match repeater.composite {
                RepeaterComposite::Below => repeater.copies - 1 - copy,
                RepeaterComposite::Above => copy,
            };
            let transform = repeater.transform(i);
            let copy_layer = CopyLayer {
                id: self.next_copy,
                alpha: start_alpha + delta_alpha * i as f64,
            };
            self.next_copy += 1;
            for geometry in &self.repeat_geometries {
                let mut geometry = geometry.clone();
                geometry.trim_order += i * geometry_count;
                geometry.transform = transform * geometry.transform;
                geometry.copies.insert(0, copy_layer);
                self.geometries.push(geometry);
            }
        }

        // Draws are consumed in reverse, unlike geometries.
        for copy in (0..repeater.copies).rev() {
            let copy_start = geometry_start + copy * geometry_count;
            let i = match repeater.composite {
                RepeaterComposite::Below => repeater.copies - 1 - copy,
                RepeaterComposite::Above => copy,
            };
            let transform = repeater.transform(i);
            for mut draw in self.repeat_draws.iter().cloned() {
                let start = draw.geometry.start - geometry_start;
                let end = draw.geometry.end - geometry_start;
                draw.geometry = copy_start + start..copy_start + end;
                draw.transform = transform * draw.transform;
                draw.repeater_depth += 1;
                self.draws.push(draw);
            }
        }
        // Clear the side buffers
        self.repeat_geometries.clear();
        self.repeat_draws.clear();
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

        let sequential = trim.mode == super::model::TrimMode::Sequential;
        let paths: Vec<Vec<PathEl>> = self.geometries[geometry_start..]
            .iter()
            .map(|geometry| {
                self.elements[geometry.elements.clone()]
                    .iter()
                    .map(|element| {
                        if sequential {
                            *element
                        } else {
                            geometry.transform * *element
                        }
                    })
                    .collect()
            })
            .collect();
        let mut intervals = vec![(0.0, 0.0); paths.len()];
        if sequential {
            let lengths: Vec<_> = paths.iter().map(|path| super::trim::length(path)).collect();
            let total_length = lengths.iter().sum();
            let mut order: Vec<_> = (0..paths.len()).collect();
            order.sort_unstable_by_key(|index| self.geometries[geometry_start + index].trim_order);
            let mut offset = 0.0;
            for index in order {
                intervals[index] = (offset, total_length);
                offset += lengths[index];
            }
        }
        let ranges: Vec<_> = [Some(first), second].into_iter().flatten().collect();
        for ((geometry, path), interval) in self.geometries[geometry_start..]
            .iter_mut()
            .zip(paths)
            .zip(intervals)
        {
            let new_start = self.trim_elements.len();
            super::trim::trim(
                &path,
                &ranges,
                sequential.then_some(interval),
                &mut self.trim_elements,
            );
            geometry.elements = new_start..self.trim_elements.len();
            if !sequential {
                geometry.transform = Affine::IDENTITY;
            }
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

    fn render(&self, scene: &mut impl RenderSink, clip_bounds: &Rect) {
        let mut active_copies: &[CopyLayer] = &[];
        let mut active_draw = None;
        let mut active_repeater_depth = 0;
        let mut path = Vec::new();

        for (draw_index, draw) in self.draws.iter().enumerate().rev() {
            if draw.alpha <= 0.0 || !draw.transform.is_finite() {
                continue;
            }
            let style_inverse = draw.transform.inverse();
            // A singular style transform collapses both fill and stroke to zero area.
            if !style_inverse.is_finite() {
                continue;
            }
            // Compound paths must be assembled after group transforms and modifiers are resolved.
            for geometries in self.geometries[draw.geometry.clone()]
                .chunk_by(|a, b| a.opacity_copies() == b.opacity_copies())
            {
                let copies = geometries[0].opacity_copies();
                if copies.iter().any(|copy| copy.alpha <= 0.0) {
                    continue;
                }

                let shared = active_copies
                    .iter()
                    .zip(copies)
                    .enumerate()
                    .take_while(|(depth, (active, next))| {
                        active.id == next.id
                            && (active_draw == Some(draw_index)
                                || (*depth < active_repeater_depth && *depth < draw.repeater_depth))
                    })
                    .count();
                for copy in active_copies[shared..].iter().rev() {
                    if copy.alpha != 1.0 {
                        scene.pop_layer();
                    }
                }
                for copy in &copies[shared..] {
                    if copy.alpha != 1.0 {
                        scene.push_layer(
                            Mix::Normal,
                            copy.alpha as _,
                            Affine::IDENTITY,
                            clip_bounds,
                        );
                    }
                }
                active_copies = copies;
                active_draw = Some(draw_index);
                active_repeater_depth = draw.repeater_depth;
                self.render_draw(draw, style_inverse, geometries, &mut path, scene);
            }
        }
        for copy in active_copies.iter().rev() {
            if copy.alpha != 1.0 {
                scene.pop_layer();
            }
        }
    }

    fn render_draw(
        &self,
        draw: &DrawData,
        style_inverse: Affine,
        geometries: &[GeometryData],
        path: &mut Vec<PathEl>,
        scene: &mut impl RenderSink,
    ) {
        let modified_brush = if draw.alpha != 1.0 {
            Some(draw.brush.clone().multiply_alpha(draw.alpha as _))
        } else {
            None
        };
        let brush = modified_brush.as_ref().unwrap_or(&draw.brush);
        let geometry = &geometries[0];
        let path = if geometries.len() == 1 && geometry.transform == draw.transform {
            &self.elements[geometry.elements.clone()]
        } else {
            path.clear();
            for geometry in geometries {
                let elements = &self.elements[geometry.elements.clone()];
                if geometry.transform == draw.transform {
                    path.extend_from_slice(elements);
                } else {
                    let transform = style_inverse * geometry.transform;
                    if transform.is_finite() {
                        path.extend(elements.iter().map(|element| transform * *element));
                    }
                }
            }
            path.as_slice()
        };
        if !path.is_empty() {
            scene.draw(draw.stroke.as_ref(), draw.transform, brush, &path);
        }
    }

    fn clear(&mut self) {
        self.elements.clear();
        self.geometries.clear();
        self.draws.clear();
        self.repeat_geometries.clear();
        self.repeat_draws.clear();
        self.next_copy = 0;
        self.trim_elements.clear();
    }
}
