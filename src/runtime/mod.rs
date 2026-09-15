// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod effects;
mod render;

use crate::Error;
use crate::import;
use crate::schema::Animation;
use std::collections::HashMap;
use std::ops::Range;

#[cfg(feature = "vello")]
mod vello;

pub mod model;

pub use effects::{BlurEdgeMode, FilterEffect, FilterLayerResult};
pub use render::{RenderSink, Renderer};

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
    /// Raster image assets, keyed by the identifier used by layer `refId`s.
    pub images: HashMap<String, model::ImageAsset>,
    /// Collection of layers.
    pub layers: Vec<model::Layer>,
}

impl Composition {
    /// Creates a new runtime composition from a buffer of Lottie file contents.
    pub fn from_slice(source: impl AsRef<[u8]>) -> Result<Composition, Error> {
        let source = Animation::from_slice(source.as_ref())?;
        import::conv_animation(source)
    }

    /// Creates a new runtime composition from a json object of Lottie file contents.
    pub fn from_json(v: serde_json::Value) -> Result<Composition, Error> {
        let source = Animation::from_json(v)?;
        import::conv_animation(source)
    }
}

impl std::str::FromStr for Composition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Animation::from_str(s)?;
        import::conv_animation(source)
    }
}
