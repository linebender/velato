use serde_repr::{Deserialize_repr, Serialize_repr};

/// Star or Polygon
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum StarType {
    Star = 1,
    Polygon = 2,
}
