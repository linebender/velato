// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::graphic_element::GraphicElementShape;
use crate::schema::animated_properties::position::Position;
use crate::schema::{
    animated_properties::multi_dimensional::MultiDimensional, shapes::shape::Shape,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EllipseShape {
    #[serde(flatten)]
    pub shape: Shape,
    /// Position
    #[serde(rename = "p")]
    pub position: Position,
    /// Size
    #[serde(rename = "s")]
    pub size: MultiDimensional,
}
