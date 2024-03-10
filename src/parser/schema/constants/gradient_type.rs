use serde_repr::{Deserialize_repr, Serialize_repr};

/// Type of a gradient
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum GradientType {
    Linear = 1,
    Radial = 2,
}
