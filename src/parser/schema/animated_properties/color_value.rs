use super::animated_property::AnimatedProperty;
use crate::parser::schema::helpers::color::Color;
use serde::{Deserialize, Serialize};

/// An animatable property that holds a Color.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColorValue {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<Color>,
}
