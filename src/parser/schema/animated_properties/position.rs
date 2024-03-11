// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::position_keyframe::PositionKeyframe;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// An animatable property to represent a position in space
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Position {
    /// The index of the property.
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>,
    /// Whether the property is animated
    #[serde(rename = "a")]
    pub animated: Option<BoolInt>,
    /// The expression for the property.
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Number of components in the value arrays.
    /// If present, values will be truncated or expanded to match this length
    /// when accessed from expressions.
    #[serde(rename = "l")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Number>,
    /// The value variant (Animated or Static).
    #[serde(rename = "k")]
    pub value: PositionValueK,
}

/// The possible values of "k" in a [`Position`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PositionValueK {
    Animated(Vec<PositionKeyframe>),
    Static(Vec<Number>),
}
