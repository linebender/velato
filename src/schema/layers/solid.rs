// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::visual::VisualLayer;
use serde::{Deserialize, Serialize};

/// Has an array of shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SolidLayer {
    /// Visual layer data
    #[serde(flatten)]
    pub visual_layer: VisualLayer,
    #[serde(rename = "sw")]
    pub width: f64,
    #[serde(rename = "sh")]
    pub height: f64,
    /// Color of the layer, unlike most other places, the color is a #rrggbb
    /// hex string
    #[serde(rename = "sc")]
    pub color: String,
}
