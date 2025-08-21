// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::composite::Composite;

use super::repeater_transform::RepeaterTransformShape;

/// Duplicates previous shapes in a group

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RepeaterShape {
    /// Shape Type
    #[serde(rename = "ty")]
    pub shape_type: String,

    /// Number of copies
    #[serde(rename = "c")]
    pub copies: FloatValue,

    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<FloatValue>,

    /// Stacking order
    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite: Option<Composite>,

    /// Transform applied to each copy
    #[serde(rename = "tr")]
    pub transform: RepeaterTransformShape,
}
