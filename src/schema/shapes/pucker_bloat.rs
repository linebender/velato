// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::shapes::{FloatValue, modifier::ModifierShape};
use serde::{Deserialize, Serialize};

/// Interpolates the shape with its center point and bezier tangents with the
/// opposite direction
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PuckerBloatShape {
    #[serde(flatten)]
    pub modifier: ModifierShape,

    /// Amount as a percentage
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<FloatValue>,
}
