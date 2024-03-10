// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::value::FloatValue;
use serde::{Deserialize, Serialize};

/// An animatable property that is split into individually animated components.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SplitVector {
    /// Flag that is true for multidimensionals with individually animated
    /// components.
    #[serde(rename = "s")]
    pub split: bool,

    /// X component.
    #[serde(rename = "x")]
    pub x: FloatValue,

    /// Y component.
    #[serde(rename = "y")]
    pub y: FloatValue,

    /// Z component.
    #[serde(rename = "z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<FloatValue>,
}
