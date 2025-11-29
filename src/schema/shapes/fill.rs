// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::graphic_element::GraphicElementShape;
use crate::schema::constants::fill_rule::FillRule;
use crate::schema::shapes::FloatValue;
use crate::schema::{
    animated_properties::color_value::ColorValue, shapes::shape_style::ShapeStyleShape,
};
use serde::{Deserialize, Serialize};

/// Solid fill color
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FillShape {
    #[serde(flatten)]
    pub shape_style: ShapeStyleShape,

    /// Color
    #[serde(rename = "c")]
    pub color: ColorValue,

    /// Fill Rule
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<FillRule>,
}
