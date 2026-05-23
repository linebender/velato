// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::star_type::StarType;
use crate::schema::{animated_properties::position::Position, shapes::shape::Shape};
use serde::{Deserialize, Serialize};

/// Regular polygon or star.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PolyStarShape {
    #[serde(flatten)]
    pub shape: Shape,
    /// Position
    #[serde(rename = "p")]
    pub position: Position,
    /// Outer Radius
    #[serde(rename = "or")]
    pub outer_radius: FloatValue,
    /// Outer Roundness as a percentage
    #[serde(rename = "os")]
    pub outer_roundness: FloatValue,
    /// Rotation, clockwise in degrees
    #[serde(rename = "r")]
    pub rotation: FloatValue,
    /// Points
    #[serde(rename = "pt")]
    pub points: FloatValue,
    /// Star type, 1 for Star, 2 for Polygon
    #[serde(rename = "sy")]
    pub star_type: StarType,
    /// If sy is 1 (star) you also have attributes defining the inner ends of
    /// the star:
    /// Points
    #[serde(rename = "ir")]
    pub inner_radius: Option<FloatValue>,
    /// Star type, 1 for Star, 2 for Polygon
    #[serde(rename = "is")]
    pub inner_roundness: Option<FloatValue>,
}
