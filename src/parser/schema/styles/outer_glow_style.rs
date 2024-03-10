// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::layer_style::LayerStyle;
use crate::parser::schema::animated_properties::color_value::ColorValue;
use crate::parser::schema::animated_properties::value::FloatValue;
use serde::{Deserialize, Serialize};

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OuterGlowStyle {
    #[serde(flatten)]
    pub layer_style: LayerStyle,
    ///
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<FloatValue>,
    ///
    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorValue>,
    ///
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
    ///
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<FloatValue>,
    ///
    #[serde(rename = "ch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choke_spread: Option<FloatValue>,
    ///
    #[serde(rename = "no")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise: Option<FloatValue>,
    ///
    #[serde(rename = "j")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<FloatValue>,
}
