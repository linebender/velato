// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::layer_style::LayerStyle;
use crate::parser::schema::animated_properties::color_value::ColorValue;
use crate::parser::schema::animated_properties::gradient_colors::GradientColors;
use crate::parser::schema::animated_properties::value::FloatValue;
use crate::parser::schema::constants::gradient_type::GradientType;
use serde::{Deserialize, Serialize};

///
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientOverlayStyle {
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
    #[serde(rename = "gf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<GradientColors>,
    ///
    #[serde(rename = "gs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smoothness: Option<FloatValue>,
    ///
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<FloatValue>,
    ///
    #[serde(rename = "gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_type: Option<GradientType>,
    ///
    #[serde(rename = "re")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<FloatValue>,
    /// Align with layer
    #[serde(rename = "al")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<FloatValue>,
    ///
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<FloatValue>,
    ///
    #[serde(rename = "of")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<FloatValue>,
}
