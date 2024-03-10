use super::base_stroke::BaseStroke;
use super::gradient::Gradient;
use super::shape_element::ShapeElement;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientStrokeShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    #[serde(flatten)]
    pub base_stroke: BaseStroke,

    #[serde(flatten)]
    pub gradient: Gradient,
}
