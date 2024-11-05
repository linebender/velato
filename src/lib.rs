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
//! let scene = renderer.render(&composition, frame, transform, alpha);
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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub(crate) mod import;
pub(crate) mod runtime;
pub(crate) mod schema;

mod error;
pub use error::Error;

// Re-export vello
pub use vello;

pub use runtime::{model, Composition, Renderer};
