// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::animated_property::AnimatedProperty;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// An animatable property that holds an array of numbers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MultiDimensional {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<Vec<Number>>,

    /// Number of components in the value arrays.
    /// If present values will be truncated or expanded to match this length
    /// when accessed from expressions.
    #[serde(rename = "l")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<Number>,
}
