// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod asset;
pub mod file_asset;
pub mod image;
pub mod precomposition;

use self::image::Image;
use self::precomposition::Precomposition;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyAsset {
    Image(Image),
    Precomposition(Precomposition),
    // unimplemented - Sound(Sound),
    // unimplemented - DataSource(DataSource),
}

impl<'de> Deserialize<'de> for AnyAsset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let value = serde_json::Value::deserialize(deserializer)?;
        let id = value
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("<unknown>")
            .to_owned();

        // Lottie assets have no single discriminator field
        if value.get("layers").is_some() {
            serde_json::from_value(value)
                .map(AnyAsset::Precomposition)
                .map_err(|e| D::Error::custom(format!("asset id={id:?} (Precomposition): {e}")))
        } else if value.get("t").and_then(|v| v.as_u64()) == Some(3) {
            Err(D::Error::custom(format!(
                "asset id={id:?}: DataSource asset not yet supported"
            )))
        } else if value.get("w").is_some() || value.get("h").is_some() {
            serde_json::from_value(value)
                .map(AnyAsset::Image)
                .map_err(|e| D::Error::custom(format!("asset id={id:?} (Image): {e}")))
        } else {
            Err(D::Error::custom(format!(
                "asset id={id:?}: Sound asset not yet supported"
            )))
        }
    }
}
