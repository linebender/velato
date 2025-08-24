// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::layer_style::LayerStyle;
use crate::schema::animated_properties::color_value::ColorValue;
use crate::schema::animated_properties::value::FloatValue;
use serde::{Deserialize, Serialize};

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColorOverlayStyle {
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
    #[serde(rename = "so")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
}
