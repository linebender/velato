// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::parser::schema::{assets::AnyAsset, helpers::int_boolean::BoolInt, layers::AnyLayer};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::fmt::Display;

/// Top level object, describing the animation
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Animation {
    /// Lottie file version
    #[serde(rename = "v")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    // Name, as seen from editors and the like
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Framerate in frames per second
    #[serde(rename = "fr")]
    pub frame_rate: Number,
    /// "In Point", which frame the animation starts at (usually 0)
    #[serde(rename = "ip")]
    pub in_point: Number,
    /// "Out Point", which frame the animation stops/loops at, which makes this
    /// the duration in frames when `ip` is 0
    #[serde(rename = "op")]
    pub out_point: Number,
    /// Width of the animation
    #[serde(rename = "w")]
    pub width: Number,
    /// Height of the animation
    #[serde(rename = "h")]
    pub height: Number,
    /// Whether the animation has 3D layers
    #[serde(rename = "ddd", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_dimensional: Option<BoolInt>,
    /// List of assets that can be referenced by layers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AnyAsset>>,
    /// List of layers
    #[serde(default)]
    pub layers: Vec<AnyLayer>,
}

impl Animation {
    pub fn from_slice(v: &[u8]) -> Result<Animation, serde_json::Error> {
        serde_json::from_slice(v)
    }

    pub fn from_json(v: serde_json::Value) -> Result<Animation, serde_json::Error> {
        serde_json::from_value(v)
    }

    pub fn to_json(&self) -> serde_json::value::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl std::str::FromStr for Animation {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
