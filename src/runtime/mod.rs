mod render;

use std::collections::HashMap;
use std::ops::Range;

/// Re-export vello.
pub use vello;

pub mod model;

pub use render::{RenderSink, Renderer};

/// Model of a Lottie file.
#[derive(Clone, Default, Debug)]
pub struct Composition {
    /// Frames in which the animation is active.
    pub frames: Range<f32>,
    /// Frames per second.
    pub frame_rate: f32,
    /// Width of the animation.
    pub width: u32,
    /// Height of the animation.
    pub height: u32,
    /// Precomposed layers that may be instanced.
    pub assets: HashMap<String, Vec<model::Layer>>,
    /// Collection of layers.
    pub layers: Vec<model::Layer>,
}

impl Composition {
    /// Creates a new composition from the specified buffer containing
    /// the content of a Lottie file.
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Box<dyn std::error::Error>> {
        crate::import::import_composition(bytes)
    }
}
