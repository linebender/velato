// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// File metadata from the lottie authoring tool
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Meta {
    #[serde(rename = "a", default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(rename = "d", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// typically a hex color string
    #[serde(rename = "tc", default, skip_serializing_if = "Option::is_none")]
    pub theme_color: Option<String>,

    /// authoring tool identifier
    #[serde(rename = "g", default, skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,

    /// Authoring tools vary between a single string and an array of strings, so this is left untyped.
    #[serde(rename = "k", default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<serde_json::Value>,
}
