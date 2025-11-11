use serde::{Deserialize, Serialize};

use crate::schema::{
    animated_properties::{position::Position, value::FloatValue},
    shapes::modifier::ModifierShape,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RoundedCornersShape {
    #[serde(flatten)]
    pub modifier: ModifierShape,

    /// The radius of the rounded corners.
    #[serde(rename = "r")]
    pub radius: FloatValue,
}
