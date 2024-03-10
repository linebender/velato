use super::shape_element::ShapeElement;
use crate::parser::schema::animated_properties::color_value::ColorValue;
use crate::parser::schema::constants::fill_rule::FillRule;
use crate::parser::schema::shapes::FloatValue;
use serde::{Deserialize, Serialize};

/// Solid fill color
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FillShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    /// Opacity, 100 means fully opaque
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<FloatValue>,

    /// Color
    #[serde(rename = "c")]
    pub color: ColorValue,

    /// Fill Rule
    #[serde(rename = "r")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<FillRule>,
}
