// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::asset::Asset;
use crate::schema::helpers::int_boolean::BoolInt;
use serde::{Deserialize, Serialize};

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileAsset {
    #[serde(flatten)]
    pub asset: Asset,
    /// Path to the directory containing a file
    #[serde(rename = "u")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    /// Filename or data url
    #[serde(rename = "p")]
    pub file_name: String,
    /// Whether the file is embedded
    #[serde(rename = "e", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded: Option<BoolInt>,
}
