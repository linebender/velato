use super::shape::Shape;
use crate::parser::schema::animated_properties::shape_property::ShapeProperty;
use serde::{Deserialize, Serialize};

/// Animatable Bezier curve
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PathShape {
    #[serde(flatten)]
    pub shape: Shape,

    /// Bezier path
    #[serde(rename = "ks")]
    pub shape_property: ShapeProperty,
}
