// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::parser::schema::animated_properties::value::FloatValue;

use super::visual::VisualLayer;

#[derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LayerId {
    Precomposition = 0,
}

/// Renders a Precomposition
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrecompositionLayer {
    #[serde(flatten)]
    pub properties: VisualLayer,
    /// Layer type, must be 0
    #[serde(rename = "ty")]
    pub layer_type: LayerId,
    /// ID of the precomp as specified in the assets
    #[serde(rename = "refId")]
    pub precomp_id: String,
    /// Width of the clipping rect
    #[serde(rename = "w")]
    pub width: Number,
    /// Height of the clipping rect
    #[serde(rename = "h")]
    pub height: Number,
    /// Time Remapping
    #[serde(rename = "tm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remap: Option<FloatValue>,
}
