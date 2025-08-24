// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::shape_element::ShapeElement;
use super::stroke_dash::StrokeDash;
use crate::schema::animated_properties::color_value::ColorValue;
use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::line_cap::LineCap;
use crate::schema::constants::line_join::LineJoin;
use serde::{Deserialize, Serialize};

/// Defines a stroke.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StrokeShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,
    /// Line Cap
    #[serde(rename = "lc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_cap: Option<LineCap>,
    /// Line Join
    #[serde(rename = "lj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_join: Option<LineJoin>,
    /// Miter Limit
    #[serde(rename = "ml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter_limit: Option<f64>,
    /// Animatable alternative to `miter_limit`
    #[serde(rename = "ml2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter_limit_alt: Option<FloatValue>,
    /// Opacity, 100 means fully opaque
    #[serde(rename = "o")]
    pub opacity: FloatValue,
    /// Stroke width
    #[serde(rename = "w")]
    pub stroke_width: FloatValue,
    /// Dashed line definition
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_array: Option<Vec<StrokeDash>>,
    /// Stroke color
    #[serde(rename = "c")]
    pub stroke_color: ColorValue,
}
