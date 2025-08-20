// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::animated_properties::shape_property::ShapeProperty;
use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::mask_mode::MaskMode;
use serde::{Deserialize, Serialize};

/// A layer can have an array of masks that clip the contents of the layer to a
/// shape.
///
/// This is similar to mattes, but there are a few differences.
///
/// With mattes, you use a layer to define the clipping area, while with masks
/// you use an animated bezier curve.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Mask {
    /// Name, as seen from editors and the like
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Match name, used in expressions
    #[serde(rename = "mn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_name: Option<String>,

    /// Inverted
    #[serde(rename = "inv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,

    /// Shape
    #[serde(rename = "pt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<ShapeProperty>,

    /// Opacity
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,

    /// Mode
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<MaskMode>,

    /// Expand
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<FloatValue>,
}
