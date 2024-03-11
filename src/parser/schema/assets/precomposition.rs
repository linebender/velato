// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::asset::Asset;
use crate::parser::schema::animation::composition::Composition;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Asset containing an animation that can be referenced by layers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Precomposition {
    #[serde(flatten)]
    pub asset: Asset,
    #[serde(flatten)]
    pub composition: Composition,
    /// Framerate in frames per second
    #[serde(rename = "fr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<Number>,
    /// Extra composition
    #[serde(rename = "xt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<BoolInt>,
}
