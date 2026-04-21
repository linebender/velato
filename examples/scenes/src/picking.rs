// Copyright 2026 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Picking-aware Lottie scene with debug overlays.
//!
//! This example demonstrates how to build layer-aware hit-testing on top of
//! velato using the [`RenderSink`] hooks (`begin_layer_group`/`end_layer_group`).
//!
//! The pipeline has four steps:
//!
//! 1. **Render** the Lottie animation through a custom [`PickingScene`] sink that
//!    intercepts draw calls while forwarding them to vello for normal rendering.
//! 2. **Collect** per-layer axis-aligned bounding boxes (AABBs) from those
//!    intercepted draw calls.
//! 3. **Hit-test** the cursor against the collected AABBs, picking the top-most
//!    layer.
//! 4. **Draw overlays** — highlight the hovered layer, dim all others, and show a
//!    tooltip at the cursor.

use std::{collections::HashSet, sync::Arc, time::Instant};

use kurbo::{Affine, Point, Rect, Shape, Vec2};
use peniko::{BlendMode, Brush, Color, Fill};
use velato::{Composition, RenderSink, Renderer, model::fixed};
use vello::Scene;

use crate::SceneParams;

const DEPTH_BAND: usize = 1000;
const HOVER_FILL: Color = Color::new([1.0, 1.0, 1.0, 0.15]);
const HOVER_STROKE: Color = Color::new([1.0, 1.0, 1.0, 0.9]);
const DIM_FILL: Color = Color::new([0.5, 0.5, 0.5, 0.08]);
const DIM_STROKE: Color = Color::new([0.5, 0.5, 0.5, 0.3]);

pub fn picking_scene() -> impl FnMut(&mut Scene, &mut SceneParams<'_>) {
    let contents = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../assets/google_fonts/Tiger.json"
    ));
    let composition = Arc::new(Composition::from_slice(contents).expect("valid lottie file"));

    let interactive: HashSet<String> = composition.layers.iter().map(|l| l.name.clone()).collect();

    let started = Instant::now();
    let mut renderer = Renderer::new();
    let resolution = Vec2::new(composition.width as f64, composition.height as f64);

    move |scene, params| {
        params.resolution = Some(resolution);

        let frame = ((started.elapsed().as_secs_f64() * composition.frame_rate)
            % (composition.frames.end - composition.frames.start))
            + composition.frames.start;

        // Step 1–2: Render through the picking sink to collect per-frame bounds.
        let mut picking = PickingScene::new(scene, interactive.clone());
        renderer.append(&composition, frame, Affine::IDENTITY, 1.0, &mut picking);
        let hits = picking.take_collected();

        // Step 3: Convert cursor to scene coordinates and pick.
        let scene_cursor = match (params.cursor_position, params.viewport_size) {
            (Some(cursor), Some(viewport)) => Some(cursor_to_scene(
                cursor,
                viewport,
                resolution,
                params.camera_transform,
            )),
            _ => None,
        };
        let hovered = scene_cursor.and_then(|c| pick_layer(&hits, c));

        // Step 4: Draw overlays — highlight hovered layer, dim the rest.
        let thin_stroke = kurbo::Stroke::new(1.0);
        let thick_stroke = kurbo::Stroke::new(3.0);

        for (i, hit) in hits.iter().enumerate() {
            let is_hovered = hovered == Some(i);

            let (fill_color, stroke_color, stroke_width) = if is_hovered {
                (HOVER_FILL, HOVER_STROKE, &thick_stroke)
            } else {
                (DIM_FILL, DIM_STROKE, &thin_stroke)
            };

            scene.fill(Fill::NonZero, Affine::IDENTITY, fill_color, None, &hit.rect);
            scene.stroke(
                stroke_width,
                Affine::IDENTITY,
                stroke_color,
                None,
                &hit.rect,
            );

            let text_color = if is_hovered {
                Color::WHITE
            } else {
                Color::new([1.0, 1.0, 1.0, 0.5])
            };
            let text_size = if is_hovered { 16.0 } else { 12.0 };
            params.text.add(
                scene,
                None,
                text_size,
                Some(&Brush::Solid(text_color)),
                Affine::translate((hit.rect.x0 + 4.0, hit.rect.y0 + text_size as f64 + 2.0)),
                &hit.name,
            );
        }

        // Tooltip: show hovered layer name at cursor (in scene coordinates).
        if let (Some(idx), Some(cursor)) = (hovered, scene_cursor) {
            params.text.add(
                scene,
                None,
                20.0,
                Some(&Brush::Solid(Color::WHITE)),
                Affine::translate((cursor.x + 16.0, cursor.y - 8.0)),
                &format!("-> {}", hits[idx].name),
            );
        }
    }
}

fn cursor_to_scene(cursor: Vec2, viewport: Vec2, resolution: Vec2, camera: Affine) -> Point {
    let scale_factor = (viewport.x / resolution.x).min(viewport.y / resolution.y);
    let full_transform = camera * Affine::scale(scale_factor);
    let inverse = full_transform.inverse();
    inverse * Point::new(cursor.x, cursor.y)
}

fn pick_layer(hits: &[LayerHit], cursor: Point) -> Option<usize> {
    hits.iter()
        .enumerate()
        .filter(|(_, hit)| hit.rect.contains(cursor))
        .max_by_key(|(_, hit)| hit.paint_order)
        .map(|(i, _)| i)
}

/// A [`RenderSink`] wrapper around a vello [`Scene`] that intercepts draw calls
/// to track per-layer bounding boxes while still forwarding everything to vello
/// for normal rendering.
struct PickingScene<'a> {
    scene: &'a mut Scene,
    tracker: LayerBoundsTracker,
}

impl<'a> PickingScene<'a> {
    fn new(scene: &'a mut Scene, interactive_layers: HashSet<String>) -> Self {
        Self {
            scene,
            tracker: LayerBoundsTracker::new(interactive_layers),
        }
    }

    fn take_collected(&mut self) -> Vec<LayerHit> {
        self.tracker.take_collected()
    }
}

impl RenderSink for PickingScene<'_> {
    fn push_layer(
        &mut self,
        blend: impl Into<BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl Shape,
    ) {
        self.scene
            .push_layer(Fill::NonZero, blend, alpha, transform, shape);
    }

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl Shape) {
        self.scene.push_clip_layer(Fill::NonZero, transform, shape);
    }

    fn pop_layer(&mut self) {
        self.scene.pop_layer();
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl Shape,
    ) {
        if self.tracker.is_inside_tracked_layer() {
            let bbox = shape.bounding_box();
            let transformed = transform.transform_rect_bbox(bbox);
            self.tracker.accumulate_bounds(transformed);
        }

        if let Some(stroke) = stroke {
            self.scene.stroke(stroke, transform, brush, None, shape);
        } else {
            self.scene
                .fill(Fill::NonZero, transform, brush, None, shape);
        }
    }

    fn begin_layer_group(&mut self, name: &str, _index: usize) {
        self.tracker.begin_layer_group(name);
    }

    fn end_layer_group(&mut self) {
        self.tracker.end_layer_group();
    }
}

struct LayerHit {
    name: String,
    rect: Rect,
    paint_order: usize,
}

/// State machine that tracks bounding boxes while walking the layer tree.
///
/// The lifecycle for each tracked layer is:
/// `begin_layer` → accumulate draw calls → `end_layer` → push to `collected`.
///
/// Nested sub-layers (precomps) are counted via `nesting` so that only the
/// outermost tracked layer produces a [`LayerHit`].
struct LayerBoundsTracker {
    interactive_layers: HashSet<String>,
    current_layer: Option<String>,
    current_paint_order: usize,
    current_bounds: Option<Rect>,
    nesting: usize,
    /// Monotonic counter — incremented on every `begin_layer_group`.
    counter: usize,
    collected: Vec<LayerHit>,
}

impl LayerBoundsTracker {
    fn new(interactive_layers: HashSet<String>) -> Self {
        Self {
            interactive_layers,
            current_layer: None,
            current_paint_order: 0,
            current_bounds: None,
            nesting: 0,
            counter: 0,
            collected: Vec::new(),
        }
    }

    fn is_inside_tracked_layer(&self) -> bool {
        self.current_layer.is_some()
    }

    fn begin_layer_group(&mut self, name: &str) {
        self.counter += DEPTH_BAND;
        if self.current_layer.is_some() {
            self.nesting += 1;
            return;
        }
        if self.interactive_layers.contains(name) {
            self.current_layer = Some(name.to_string());
            self.current_paint_order = self.counter;
            self.current_bounds = None;
            self.nesting = 0;
        }
    }

    fn accumulate_bounds(&mut self, rect: Rect) {
        if self.current_layer.is_some() {
            self.current_bounds = Some(match self.current_bounds {
                Some(existing) => existing.union(rect),
                None => rect,
            });
        }
    }

    fn end_layer_group(&mut self) {
        if self.current_layer.is_none() {
            return;
        }
        if self.nesting > 0 {
            self.nesting -= 1;
            return;
        }
        if let Some(name) = self.current_layer.take()
            && let Some(rect) = self.current_bounds.take()
        {
            self.collected.push(LayerHit {
                name,
                rect,
                paint_order: self.current_paint_order,
            });
        }
    }

    fn take_collected(&mut self) -> Vec<LayerHit> {
        std::mem::take(&mut self.collected)
    }
}
