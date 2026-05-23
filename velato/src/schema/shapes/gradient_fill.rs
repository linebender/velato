// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::base_gradient::BaseGradientShape;
use crate::schema::constants::fill_rule::FillRule;
use crate::schema::{animated_properties::value::FloatValue, shapes::shape_style::ShapeStyleShape};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientFillShape {
    #[serde(flatten)]
    pub shape_style: ShapeStyleShape,

    /// Gradient data
    #[serde(flatten)]
    pub gradient: BaseGradientShape,

    /// Fill Rule
    #[serde(rename = "r")]
    pub fill_rule: Option<FillRule>,
}
