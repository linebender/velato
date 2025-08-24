// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::shapes::FloatValue;
use serde::{Deserialize, Serialize};

/// Interpolates the shape with its center point and bezier tangents with the
/// opposite direction
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PuckerBloatShape {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,
    /// Amount as a percentage
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<FloatValue>,
}
