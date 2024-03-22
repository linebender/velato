// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Render a Lottie animation to a Vello [`Scene`](https://docs.rs/vello/*/vello/struct.Scene.html).
//!
//! However, this is also intended to be the preferred integration between Vello and [Lottie], so [consider
//! contributing](https://github.com/linebender/velato) if you need a feature which is missing.
//!
//! This crate also re-exports [`vello`], to make handling dependency versions easier
//!
//! ## Usage
//!
//! ```rust
//! // Parse your lottie file
//! let lottie = inclide_str!("../lottie.json");
//! let composition = velato::Composition::from_str(lottie).expect("valid file");
//!
//! // Render to a scene
//! let mut new_scene = vello::Scene::new();
//!
//! // Render to a scene!
//! let mut renderer = velato::Renderer::new();
//! let frame = 0.0; // Arbitrary number chosen. Ensure it's a valid frame!
//! let transform = Affine::IDENTITY;
//! let alpha = 1.0;
//! renderer.render(&composition, frame, transform, alpha, &mut new_scene);
//! ```
//!
//! # Unsupported features
//!
//! Missing features include:
//! - Non-linear easings
//! - Position keyframe (`ti`, `to`) easing
//! - Time remapping (`tm`)
//! - Text
//! - Image embedding
//! - Advanced shapes (stroke dash, zig-zag, etc.)
//! - Advanced effects (motion blur, drop shadows, etc.)
//! - Correct color stop handling
//! - Split rotations
//! - Split positions

pub(crate) mod import;
pub(crate) mod runtime;
pub(crate) mod schema;

// Re-export vello
pub use vello;

pub use runtime::{model, Composition, Renderer};
