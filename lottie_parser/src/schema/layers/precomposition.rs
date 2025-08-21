// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

use crate::schema::animated_properties::value::FloatValue;

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
    pub width: f64,
    /// Height of the clipping rect
    #[serde(rename = "h")]
    pub height: f64,
    /// Time Remapping
    #[serde(rename = "tm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_remap: Option<FloatValue>,
}

#[cfg(test)]
mod tests {
    use crate::schema::{
        animation::composition::Composition,
        assets::{asset::Asset, precomposition::Precomposition},
        helpers::int_boolean::BoolInt,
    };
    use once_cell::sync::Lazy;
    use serde_json::json;

    static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
        json!(
            {
                "id": "precomp_0",
                "fr": 60,
                "nm": "Example",
                "xt": 0,
                "layers": []
            }
        )
    });
    static PRECOMP: Lazy<Precomposition> = Lazy::new(|| Precomposition {
        asset: Asset {
            id: "precomp_0".to_string(),
            name: Some("Example".to_string()),
        },
        composition: Composition { layers: vec![] },
        frame_rate: Some(60.0),
        extra: Some(BoolInt::False),
    });

    #[test]
    fn test_serde_deserialize() {
        let actual = serde_json::from_value(JSON.to_owned());

        match actual {
            Ok(actual) => assert_eq!(*PRECOMP, actual),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn test_can_serialize() {
        serde_json::to_value(&*PRECOMP).unwrap();
    }
}
