// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Render a Lottie animation to a Vello [`Scene`](crate::vello::Scene).
//!
//! This currently lacks support for a [number of important](crate#unsupported-features) SVG features.
//!
//! This is also intended to be the preferred integration between Vello and [usvg](https://crates.io/crates/usvg),
//! so [consider contributing](https://github.com/linebender/vello_svg) if you need a feature which is missing.
//!
//! This crate also re-exports [`vello`], so you can easily use the specific version that is compatible with Velato.
//!
//! ## Usage
//!
//! ```no_run
//! # use std::str::FromStr;
//! use velato::vello;
//!
//! // Parse your lottie file
//! let lottie = include_str!("../examples/assets/google_fonts/Tiger.json");
//! let composition = velato::Composition::from_str(lottie).expect("valid file");
//!
//! // Render to a scene
//! let mut new_scene = vello::Scene::new();
//!
//! // Render to a scene!
//! let mut renderer = velato::Renderer::new();
//! let frame = 0.0; // Arbitrary number chosen. Ensure it's a valid frame!
//! let transform = vello::kurbo::Affine::IDENTITY;
//! let alpha = 1.0;
//! let scene = renderer.render_to_vello_scene(&composition, frame, transform, alpha);
//! ```
//!
//! # Unsupported features
//!
//! Missing features include:
//! - Position keyframe (`ti`, `to`) easing
//! - Time remapping (`tm`)
//! - Text
//! - Image embedding
//! - Advanced shapes (stroke dash, zig-zag, etc.)
//! - Advanced effects (motion blur, drop shadows, etc.)
//! - Correct color stop handling
//! - Split rotations
//! - Split positions

// LINEBENDER LINT SET - lib.rs - v4
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    test,
    allow(
        unused_crate_dependencies,
        reason = "Some dev dependencies are only used in tests"
    )
)]

pub use lato::*;
pub use vello;

use kurbo::{Affine, Shape};
use lato::model::fixed;
use peniko::{BlendMode, Fill};

struct VelatoSceneSink<'a>(pub &'a mut vello::Scene);

impl RenderSink for VelatoSceneSink<'_> {
    fn push_layer(
        &mut self,
        blend: impl Into<BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl Shape,
    ) {
        self.0
            .push_layer(Fill::NonZero, blend, alpha, transform, shape);
    }

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl Shape) {
        self.0.push_clip_layer(Fill::NonZero, transform, shape);
    }

    fn pop_layer(&mut self) {
        self.0.pop_layer();
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl Shape,
    ) {
        if let Some(stroke) = stroke {
            self.0.stroke(stroke, transform, brush, None, shape);
        } else {
            self.0.fill(Fill::NonZero, transform, brush, None, shape);
        }
    }
}

/// Renders a [`lato::Composition`] into a [`vello::Scene`].
#[derive(Debug, Default)]
pub struct Renderer(lato::Renderer);

impl Renderer {
    /// Create a new renderer.
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
        scene: &mut vello::Scene,
    ) {
        let mut scene = VelatoSceneSink(scene);
        self.0
            .append(animation, frame, transform, alpha, &mut scene);
    }

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
