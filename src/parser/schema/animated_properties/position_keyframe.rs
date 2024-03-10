// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::keyframe::Keyframe;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Position Keyframe
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PositionKeyframe {
    #[serde(flatten)]
    pub keyframe: Keyframe,
    /// In-Tangent for values (e.g., moving position around a curved path).
    #[serde(rename = "ti")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_in_tangent: Option<Vec<Number>>,
    /// Out-Tangent for values (e.g., moving position around a curved path).
    #[serde(rename = "to")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_out_tangent: Option<Vec<Number>>,
}
