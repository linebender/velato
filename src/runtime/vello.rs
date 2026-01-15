// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Composition, RenderSink, Renderer, model::fixed};

use kurbo::{Affine, Shape};
use peniko::{BlendMode, Fill};

impl RenderSink for vello::Scene {
    fn push_layer(
        &mut self,
        blend: impl Into<BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl Shape,
    ) {
        self.push_layer(blend, alpha, transform, shape);
    }

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl Shape) {
        self.push_clip_layer(transform, shape);
    }

    fn pop_layer(&mut self) {
        self.pop_layer();
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl Shape,
    ) {
        if let Some(stroke) = stroke {
            self.stroke(stroke, transform, brush, None, shape);
        } else {
            self.fill(Fill::NonZero, transform, brush, None, shape);
        }
    }
}

impl Renderer {
    /// Renders the animation at a given frame to a new scene.
    pub fn render_to_vello_scene(
        &mut self,
        animation: &Composition,
        frame: f64,
        transform: Affine,
        alpha: f64,
    ) -> vello::Scene {
        let mut scene = vello::Scene::new();
        self.append(animation, frame, transform, alpha, &mut scene);
        scene
    }
}
