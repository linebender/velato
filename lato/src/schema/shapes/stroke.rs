// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::graphic_element::GraphicElementShape;
use super::stroke_dash::StrokeDashShape;
use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::line_cap::LineCap;
use crate::schema::constants::line_join::LineJoin;
use crate::schema::shapes::base_stroke::BaseStrokeShape;
use crate::schema::{
    animated_properties::color_value::ColorValue, shapes::shape_style::ShapeStyleShape,
};
use serde::{Deserialize, Serialize};

/// Defines a stroke.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StrokeShape {
    #[serde(flatten)]
    pub shape_style: ShapeStyleShape,
    #[serde(flatten)]
    pub base_stroke: BaseStrokeShape,

    /// Stroke color
    #[serde(rename = "c")]
    pub stroke_color: ColorValue,
}
