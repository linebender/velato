// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::gradient::Gradient;
use super::shape_element::ShapeElement;
use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::fill_rule::FillRule;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientFillShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    /// Gradient data
    #[serde(flatten)]
    pub gradient: Gradient,

    /// Opacity
    #[serde(rename = "o")]
    pub opacity: FloatValue,

    /// Fill Rule
    #[serde(rename = "r")]
    pub fill_rule: Option<FillRule>,
}
