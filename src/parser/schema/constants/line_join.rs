use serde_repr::{Deserialize_repr, Serialize_repr};

/// Style at a sharp corner of a stoked line
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum LineJoin {
    Miter = 1,
    Round = 2,
    Bevel = 3,
}
