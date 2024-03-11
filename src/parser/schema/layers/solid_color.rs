// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::visual::VisualLayer;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LayerId {
    SolidColor = 1,
}

/// Has an array of shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SolidColorLayer {
    #[serde(flatten)]
    pub properties: VisualLayer,

    /// Layer type, must be 1
    #[serde(rename = "ty")]
    pub layer_type: LayerId,

    /// Color of the layer, unlike most other places, the color is a #rrggbb
    /// hex string
    #[serde(rename = "sc")]
    pub color: String,

    #[serde(rename = "sh")]
    pub height: Number,

    #[serde(rename = "sw")]
    pub width: Number,
}
