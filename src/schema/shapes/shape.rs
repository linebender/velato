// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::graphic_element::GraphicElementShape;
use crate::schema::constants::shape_direction::ShapeDirection;
use serde::{Deserialize, Serialize};

/// Drawable shape
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Shape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,

    /// Direction the shape is drawn as, mostly relevant when using trim path
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ShapeDirection>,
}
