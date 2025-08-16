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
//! - Position keyframe (`ti`, `to`) easing
//! - Time remapping (`tm`)
//! - Text
//! - Image embedding
//! - Advanced shapes (stroke dash, zig-zag, etc.)
//! - Advanced effects (motion blur, drop shadows, etc.)
//! - Correct color stop handling
//! - Split rotations
//! - Split positions

// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// The following lints are part of the Linebender standard set,
// but resolving them has been deferred for now.
// Feel free to send a PR that solves one or more of these.
#![allow(
    unreachable_pub,
    missing_docs,
    elided_lifetimes_in_paths,
    single_use_lifetimes,
    unused_qualifications,
    clippy::empty_docs,
    clippy::use_self,
    clippy::return_self_not_must_use,
    clippy::cast_possible_truncation,
    clippy::shadow_unrelated,
    clippy::missing_assert_message,
    clippy::missing_errors_doc,
    clippy::exhaustive_enums,
    clippy::todo,
    reason = "Deferred"
)]
#![cfg_attr(
    test,
    allow(
        unused_crate_dependencies,
        reason = "Some dev dependencies are only used in tests"
    )
)]

pub(crate) mod import;
pub(crate) mod runtime;
pub(crate) mod schema;

mod error;
pub use error::Error;

// Re-export vello
#[cfg(not(feature = "vello"))]
pub use {kurbo, peniko};
#[cfg(feature = "vello")]
pub use {runtime::Renderer, vello};

pub use runtime::{Composition, model};
