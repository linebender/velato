// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod render;

use crate::import;
use crate::schema::Animation;
use std::collections::HashMap;
use std::ops::Range;

pub mod model;

pub use render::Renderer;

/// Model of a Lottie file.
#[derive(Clone, Default, Debug)]
pub struct Composition {
    /// Frames in which the animation is active.
    pub frames: Range<f64>,
    /// Frames per second.
    pub frame_rate: f64,
    /// Width of the animation.
    pub width: usize,
    /// Height of the animation.
    pub height: usize,
    /// Precomposed layers that may be instanced.
    pub assets: HashMap<String, Vec<model::Layer>>,
    /// Collection of layers.
    pub layers: Vec<model::Layer>,
}

/// Triggered when is an issue parsing a lottie file.
pub type VelatoError = serde_json::Error;

impl Composition {
    /// Creates a new runtime composition from a buffer of Lottie file contents.
    pub fn from_slice(source: impl AsRef<[u8]>) -> Result<Composition, VelatoError> {
        let source = Animation::from_slice(source.as_ref())?;
        let composition = import::conv_animation(source);
        Ok(composition)
    }

    /// Creates a new runtime composition from a json object of Lottie file contents.
    pub fn from_json(v: serde_json::Value) -> Result<Composition, VelatoError> {
        let source = Animation::from_json(v)?;
        let composition = import::conv_animation(source);
        Ok(composition)
    }
}

impl std::str::FromStr for Composition {
    type Err = VelatoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Animation::from_str(s)?;
        let composition = import::conv_animation(source);
        Ok(composition)
    }
}
