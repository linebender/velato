use serde_repr::{Deserialize_repr, Serialize_repr};

/// Defines the function used to determine the interpolating factor on a text
/// range selector.
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextShape {
    Square = 1,
    RampUp = 2,
    RampDown = 3,
    Triangle = 4,
    Round = 5,
    Smooth = 6,
}
