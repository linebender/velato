use super::layer_style::LayerStyle;
use crate::parser::schema::animated_properties::color_value::ColorValue;
use crate::parser::schema::animated_properties::value::FloatValue;
use serde::{Deserialize, Serialize};

/// Style applied to a layer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SatinStyle {
    #[serde(flatten)]
    pub layer_style: LayerStyle,
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<FloatValue>,
    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorValue>,
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<FloatValue>,
    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<FloatValue>,
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<FloatValue>,
    #[serde(rename = "in")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<FloatValue>,
}
