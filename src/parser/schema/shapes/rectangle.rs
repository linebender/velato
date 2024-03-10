use crate::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::parser::schema::animated_properties::position::Position;
use crate::parser::schema::animated_properties::value::FloatValue;
use serde::{Deserialize, Serialize};

use super::shape_element::ShapeElement;

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RectangleShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,
    /// Center of the rectangle
    #[serde(rename = "p")]
    pub position: Position,
    /// Size
    #[serde(rename = "s")]
    pub size: MultiDimensional,
    /// Rounded corners radius
    #[serde(rename = "r")]
    pub rounded_corner_radius: FloatValue,
}
