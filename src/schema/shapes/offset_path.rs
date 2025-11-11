// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::shapes::FloatValue;
use crate::schema::{constants::line_join::LineJoin, shapes::modifier::ModifierShape};
use serde::{Deserialize, Serialize};

/// Interpolates the shape with its center point and bezier tangents with the
/// opposite direction
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OffsetPathShape {
    #[serde(flatten)]
    pub modifier: ModifierShape,

    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<FloatValue>,

    #[serde(rename = "lj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_join: Option<LineJoin>, // default  2

    #[serde(rename = "ml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miter_limit: Option<FloatValue>,
}
