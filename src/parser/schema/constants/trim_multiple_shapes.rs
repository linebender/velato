use serde_repr::{Deserialize_repr, Serialize_repr};

/// How to handle multiple shapes in trim path
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TrimMultipleShapes {
    ///
    Individually = 1,
    ///
    Simultaneously = 2,
}
