// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::animated_properties::position::Position;
use crate::schema::animated_properties::value::FloatValue;
use crate::schema::{
    animated_properties::multi_dimensional::MultiDimensional, shapes::shape::Shape,
};
use serde::{Deserialize, Serialize};

use super::graphic_element::GraphicElementShape;

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RectangleShape {
    #[serde(flatten)]
    pub shape: Shape,
    /// Center of the rectangle
    #[serde(rename = "p")]
    pub position: Position,
    /// Size
    #[serde(rename = "s")]
    pub size: MultiDimensional,
    /// Rounded corners radius
    #[serde(rename = "r")]
    pub rounded_corner_radius: FloatValue,
}
