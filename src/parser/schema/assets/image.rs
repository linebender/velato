// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::file_asset::FileAsset;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Number;

/// External image
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(flatten)]
    pub file_asset: FileAsset,
    /// Width of the image
    #[serde(rename = "w")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Number>,
    /// Height of the image
    #[serde(rename = "h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Number>,
    /// Mark as part of an image sequence if present.
    #[serde(
        rename = "t",
        deserialize_with = "seq_from_str",
        serialize_with = "seq_to_str",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<bool>,
}

pub fn seq_from_str<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    if v == *"seq" {
        Ok(Some(true))
    } else {
        Ok(None)
    }
}

pub fn seq_to_str<S>(v: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        Some(true) => Serializer::serialize_str(serializer, "seq"),
        Some(false) => Serializer::serialize_str(serializer, ""),
        None => unimplemented!("serializer should skip if none"),
    }
}
