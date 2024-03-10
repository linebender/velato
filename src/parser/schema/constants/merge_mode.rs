use serde_repr::{Deserialize_repr, Serialize_repr};

/// Boolean operation on shapes
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum MergeMode {
    Normal = 1,
    Add = 2,
    Subtract = 3,
    Intersect = 4,
    ExcludeIntersections = 5,
}
