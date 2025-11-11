use serde::{Deserialize, Serialize};

use crate::schema::{
    animated_properties::{position::Position, value::FloatValue},
    shapes::modifier::ModifierShape,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TwistShape {
    #[serde(flatten)]
    pub modifier: ModifierShape,

    /// The angle of the twist in degrees.
    #[serde(rename = "a")]
    pub angle: FloatValue,

    /// The position of the center of the twist effect.
    #[serde(rename = "p")]
    pub position: Position,
}
