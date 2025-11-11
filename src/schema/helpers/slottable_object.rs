// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// Object that may have its value replaced with a slot value
#[derive(Deserialize, Serialize, PartialEq, Default, Debug, Clone)]
pub struct SlottableObject {
    /// Identifier to look up the slot
    #[serde(rename = "sid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
