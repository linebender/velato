// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::VisualLayer;
use serde::{Deserialize, Serialize};

#[derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LayerId {
    Null = 3,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NullLayer {
    #[serde(flatten)]
    pub properties: VisualLayer,

    /// Layer type, must be 3
    #[serde(rename = "ty")]
    pub layer_type: LayerId,
}
