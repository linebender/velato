use super::shape_element::ShapeElement;
use crate::parser::schema::helpers::transform::Transform;
use serde::{Deserialize, Serialize};

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TransformShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,
    #[serde(flatten)]
    pub transform: Transform,
}
