// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::file_asset::FileAsset;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize, Serializer};

/// External image
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(flatten)]
    pub file_asset: FileAsset,
    /// Width of the image
    #[serde(rename = "w")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// Height of the image
    #[serde(rename = "h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
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

// Function signature must match Serde's Serialize trait, so need to suppress Clippy.
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

#[cfg(test)]
mod tests {
    use super::Image;
    use crate::schema::{
        assets::{asset::Asset, file_asset::FileAsset},
        helpers::int_boolean::BoolInt,
    };
    use once_cell::sync::Lazy;
    use serde_json::json;

    static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
        json!(
            {
                "id": "my image",
                "h": 512,
                "w": 512,
                "e": 1,
                "p": "data:image/png;base64,..."
            }
        )
    });
    static IMAGE: Lazy<Image> = Lazy::new(|| Image {
        file_asset: FileAsset {
            asset: Asset {
                id: "my image".to_string(),
                name: None,
            },
            dir: None,
            file_name: "data:image/png;base64,...".to_string(),
            embedded: Some(BoolInt::True),
        },
        height: Some(512.0),
        width: Some(512.0),
        sequence: None,
    });

    #[test]
    fn test_deserialize() {
        let actual = serde_json::from_value(JSON.to_owned());

        match actual {
            Ok(actual) => assert_eq!(*IMAGE, actual),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn test_can_serialize() {
        serde_json::to_value(&*IMAGE).unwrap();
    }
}
