// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::keyframe_base::KeyframeBase;
use serde::{Deserialize, Serialize};

/// Keyframes specifies the value at a specific time and the interpolation
/// function to reach the next keyframe.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Keyframe {
    #[serde(flatten)]
    pub base: KeyframeBase,
    /// Value at this keyframe. Note that if the property is a scalar, keyframe
    /// values are still represented as arrays.
    #[serde(rename = "s")]
    pub value: Vec<f64>,
    /// Value at the end of the keyframe. Note that this is deprecated, and you
    /// should use "s" from the next keyframe to get this value.
    #[serde(rename = "e")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value_deprecated: Option<Vec<f64>>,
}
