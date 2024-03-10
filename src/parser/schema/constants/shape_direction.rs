use serde_repr::{Deserialize_repr, Serialize_repr};

/// Drawing direction of the shape curve, useful for trim path
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum ShapeDirection {
    /// Usually clockwise
    Normal = 1,
    /// Usually counter clockwise
    Reversed = 3,
}
