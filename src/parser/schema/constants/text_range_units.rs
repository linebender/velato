use serde_repr::{Deserialize_repr, Serialize_repr};

/// Unit type for a text selector
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextRangeUnits {
    Percent = 1,
    Index = 2,
}
