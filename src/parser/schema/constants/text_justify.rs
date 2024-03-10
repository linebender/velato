use serde_repr::{Deserialize_repr, Serialize_repr};

/// Text alignment / justification
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextJustify {
    Left = 0,
    Right = 1,
    Center = 2,
    JustifyWithLastLineLeft = 3,
    JustifyWithLastLineRight = 4,
    JustifyWithLastLineCenter = 5,
    JustifyWithLastLineFull = 6,
}
