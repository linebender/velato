// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::shape_element::ShapeElement;
use crate::parser::schema::constants::shape_direction::ShapeDirection;
use serde::{Deserialize, Serialize};

/// Drawable shape
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Shape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    /// Direction the shape is drawn as, mostly relevant when using trim path
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ShapeDirection>,
}
