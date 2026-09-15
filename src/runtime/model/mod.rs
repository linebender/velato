// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kurbo::{Affine, PathEl, Point, Shape as _, Size, Vec2};
use peniko::{BlendMode, Color};
use std::ops::Range;

/// Raster image metadata. Loading and decoding belong to [`crate::RenderSink`].
#[derive(Clone, Debug, PartialEq)]
pub struct ImageAsset {
    /// Identifier referenced by an image layer's `refId`.
    pub id: String,
    /// Authored display width in pixels.
    pub width: Option<f64>,
    /// Authored display height in pixels.
    pub height: Option<f64>,
    /// Directory or URL prefix from the asset's `u` field.
    pub directory: Option<String>,
    /// File name or data URL from the asset's `p` field.
    pub file_name: String,
    /// Whether the asset is marked as embedded by its `e` field.
    pub embedded: bool,
}

impl ImageAsset {
    /// Asset location (`u` + `p`), or the unchanged data URL.
    pub fn location(&self) -> String {
        if self.is_data_url() {
            return self.file_name.clone();
        }
        match &self.directory {
            Some(directory) => format!("{directory}{}", self.file_name),
            None => self.file_name.clone(),
        }
    }

    /// Detects data URLs independently of the embedded flag.
    pub fn is_data_url(&self) -> bool {
        self.file_name.starts_with("data:")
    }
}

mod position;
mod spline;
mod value;

pub mod animated;
pub mod fixed;

pub use position::{SpatialKeyframe, SpatialPosition};
pub use value::{Animated, Easing, EasingHandle, Time, Tween, Value, ValueRef};

pub(crate) use spline::SplineToPath;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    expect(
        clippy::large_enum_variant,
        reason = "Deferred. Furthermore, for some reason, only on wasm32, this isn't triggering clippy."
    )
)]
pub enum Transform {
    Fixed(fixed::Transform),
    Animated(animated::Transform),
}
impl Transform {
    pub fn is_fixed(&self) -> bool {
        match self {
            Self::Fixed(_) => true,
            Self::Animated(value) => value.is_fixed(),
        }
    }

    /// Authored components, unavailable for manually constructed matrix-only transforms.
    pub fn components(&self, frame: f64) -> Option<TransformComponents> {
        match self {
            Self::Fixed(_) => None,
            Self::Animated(value) => Some(value.components(frame)),
        }
    }

    pub fn evaluate(&self, frame: f64) -> ValueRef<'_, fixed::Transform> {
        match self {
            Self::Fixed(value) => ValueRef::Borrowed(value),
            Self::Animated(value) => ValueRef::Owned(value.evaluate(frame)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Stroke {
    Fixed(fixed::Stroke),
    Animated(animated::Stroke),
}
impl Stroke {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }
    pub fn evaluate(&self, frame: f64) -> ValueRef<'_, fixed::Stroke> {
        match self {
            Self::Fixed(value) => ValueRef::Borrowed(value),
            Self::Animated(value) => ValueRef::Owned(value.evaluate(frame)),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    expect(
        clippy::large_enum_variant,
        reason = "Deferred. Furthermore, for some reason, only on wasm32, this isn't triggering clippy."
    )
)]
pub enum Repeater {
    Fixed(fixed::Repeater),
    Animated(animated::Repeater),
}

/// How repeater copies are stacked relative to the original shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RepeaterComposite {
    /// Later copies are painted below earlier copies.
    #[default]
    Below,
    /// Later copies are painted above earlier copies.
    Above,
}

impl Repeater {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }
    pub fn evaluate(&self, frame: f64) -> ValueRef<'_, fixed::Repeater> {
        match self {
            Self::Fixed(value) => ValueRef::Borrowed(value),
            Self::Animated(value) => ValueRef::Owned(value.evaluate(frame)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ColorStops {
    Fixed(fixed::ColorStops),
    Animated(animated::ColorStops),
}
impl ColorStops {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }
    pub fn evaluate(&self, frame: f64) -> ValueRef<'_, fixed::ColorStops> {
        match self {
            Self::Fixed(value) => ValueRef::Borrowed(value),
            Self::Animated(value) => ValueRef::Owned(value.evaluate(frame)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Brush {
    Fixed(fixed::Brush),
    Animated(animated::Brush),
}

impl Brush {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    pub fn evaluate(&self, alpha: f64, frame: f64) -> ValueRef<'_, fixed::Brush> {
        match self {
            Self::Fixed(value) => {
                if alpha == 1.0 {
                    ValueRef::Borrowed(value)
                } else {
                    ValueRef::Owned(value.to_owned().multiply_alpha(alpha as _))
                }
            }
            Self::Animated(value) => ValueRef::Owned(value.evaluate(alpha, frame)),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::Fixed(Affine::IDENTITY)
    }
}

#[derive(Clone, Debug)]
pub enum Geometry {
    Fixed(Vec<PathEl>),
    Rect(animated::Rect),
    Ellipse(animated::Ellipse),
    Spline(animated::Spline),
    Star(animated::Star),
}

impl Geometry {
    pub fn evaluate(&self, frame: f64, path: &mut Vec<PathEl>) {
        match self {
            Self::Fixed(value) => {
                path.extend_from_slice(value);
            }
            Self::Rect(value) => {
                value.evaluate(frame, path);
            }
            Self::Ellipse(value) => {
                value.evaluate(frame, path);
            }
            Self::Spline(value) => {
                value.evaluate(frame, path);
            }
            Self::Star(value) => {
                value.evaluate(frame, path);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Draw {
    /// Parameters for a stroked draw operation.
    pub stroke: Option<Stroke>,
    /// Brush for the draw operation.
    pub brush: Brush,
    /// Opacity of the draw operation.
    pub opacity: Value<f64>,
}

/// Elements of a shape layer.
#[derive(Clone, Debug)]
pub enum Shape {
    /// Group of shapes with an optional transform.
    Group(Vec<Shape>, Option<GroupTransform>),
    /// Geometry element.
    Geometry(Geometry),
    /// Fill or stroke element.
    Draw(Draw),
    /// Repeater element.
    Repeater(Repeater),
    /// Trim element.
    Trim(Trim),
}

/// Evaluated authored transform values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransformComponents {
    pub anchor: Point,
    pub position: Point,
    pub scale: Vec2,
    pub rotation: f64,
    pub skew: f64,
    pub skew_angle: f64,
}

impl TransformComponents {
    pub fn matrix(self) -> Affine {
        let skew = if self.skew != 0.0 {
            let angle = self.skew_angle.to_radians();
            Affine::rotate(-angle)
                * Affine::skew(-self.skew.to_radians().tan(), 0.0)
                * Affine::rotate(angle)
        } else {
            Affine::IDENTITY
        };

        Affine::translate(self.position.to_vec2())
            * Affine::rotate(self.rotation.to_radians())
            * skew
            * Affine::scale_non_uniform(self.scale.x / 100.0, self.scale.y / 100.0)
            * Affine::translate(-self.anchor.to_vec2())
    }
}

/// Transform and opacity for a shape group.
#[derive(Clone, Debug)]
pub struct GroupTransform {
    pub transform: Transform,
    pub opacity: Value<f64>,
}

/// A Lottie layer effect (`ef`) with animatable parameters.
///
/// Effect behavior follows LottieFiles' ThorVG-powered player and editor.
#[derive(Clone, Debug)]
pub enum LayerEffect {
    /// Lottie Gaussian Blur effect (`ty: 29`). Blurs source color and alpha together.
    GaussianBlur {
        blurriness: Value<f64>,
        dimensions: Value<f64>,
        wrap: Value<f64>,
    },
    /// Lottie Drop Shadow effect (`ty: 25`).
    /// Blurs source alpha and scales it by `opacity` (0–255), then draws the source over it.
    /// Ignores alpha in `color`.
    DropShadow {
        color: Value<peniko::Color>,
        opacity: Value<f64>,
        angle: Value<f64>,
        distance: Value<f64>,
        softness: Value<f64>,
    },
    /// Lottie Fill effect (`ty: 21`).
    /// Replaces source RGB; output alpha is source alpha times `opacity` (0–1).
    /// Ignores alpha in `color`.
    Fill {
        color: Value<peniko::Color>,
        opacity: Value<f64>,
    },
    /// Lottie Tint effect (`ty: 20`).
    /// Changes source RGB while keeping source alpha. Ignores alpha in `black` and `white`.
    Tint {
        black: Value<peniko::Color>,
        white: Value<peniko::Color>,
        amount: Value<f64>,
    },
}

#[derive(Clone, Debug)]
pub struct UnsupportedEffect {
    pub name: String,
    pub effect_type: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayerReference {
    /// Runtime index in the containing composition's layer array.
    Resolved(usize),
    /// Authored Lottie `ind` for which import found no matching layer.
    Unresolved(usize),
}

/// Layer in an animation.
#[derive(Clone, Debug, Default)]
pub struct Layer {
    /// Excluded from normal rendering, but still usable as a matte source or parent.
    pub hidden: bool,
    pub effects: Vec<LayerEffect>,
    /// Effects Velato could not import because their type is unsupported or their parameters are invalid.
    pub unsupported_effects: Vec<UnsupportedEffect>,
    /// Name of the layer.
    pub name: String,
    /// Transform parent in the same composition; `None` means no authored parent.
    pub parent: Option<LayerReference>,
    /// Transform for the entire layer.
    pub transform: Transform,
    /// Opacity for the entire layer.
    pub opacity: Value<f64>,
    /// Width of the layer.
    pub width: f64,
    /// Height of the layer.
    pub height: f64,
    /// Blend mode for the layer.
    pub blend_mode: Option<peniko::BlendMode>,
    /// Range of frames in which the layer is active.
    pub frames: Range<f64>,
    /// Frame time stretch factor.
    pub stretch: f64,
    /// Starting frame for the layer (only applied to instances).
    pub start_frame: f64,
    /// List of masks applied to the content.
    pub masks: Vec<Mask>,
    /// True if the layer is used as a matte source.
    pub is_matte_source: bool,
    /// Matte blend mode and source layer.
    pub matte: Option<(BlendMode, LayerReference)>,
    /// Content of the layer.
    pub content: Content,
}

/// Matte layer mode.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Matte {
    #[default]
    Normal,
    // TODO: Use these
    // Alpha,
    // InvertAlpha,
    // Luma,
    // InvertLuma,
}

/// Mask for a layer.
#[derive(Clone, Debug)]
pub struct Mask {
    /// Blend mode for the mask.
    pub mode: peniko::BlendMode,
    /// Geometry that defines the shape of the mask.
    pub geometry: Geometry,
    /// Opacity of the mask.
    pub opacity: Value<f64>,
}

/// Content of a layer.
#[derive(Clone, Default, Debug)]
pub enum Content {
    /// Empty layer.
    #[default]
    None,
    /// Asset instance with the specified name and time remapping.
    Instance {
        name: String,
        time_remap: Option<Value<f64>>,
    },
    /// Raster image asset referenced by its Lottie `refId`.
    Image { asset_id: String },
    /// Collection of shapes.
    Shape(Vec<Shape>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TrimMode {
    /// Apply the range independently to each contour (`m: 1`).
    #[default]
    Parallel,
    /// Apply the range across combined contour lengths (`m: 2`).
    Sequential,
}

#[derive(Clone, Debug)]
pub enum Trim {
    Fixed(fixed::Trim),
    Animated(animated::Trim),
}

impl Trim {
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    pub fn evaluate(&self, frame: f64) -> ValueRef<'_, fixed::Trim> {
        match self {
            Self::Fixed(value) => ValueRef::Borrowed(value),
            Self::Animated(value) => ValueRef::Owned(value.evaluate(frame)),
        }
    }
}
