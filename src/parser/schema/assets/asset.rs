// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Asset {
    /// Unique identifier used by layers when referencing this asset
    #[serde(rename = "id")]
    pub id: String,
    /// Human readable name
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
