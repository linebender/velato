use serde_repr::{Deserialize_repr, Serialize_repr};

/// Text capitalization
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextCaps {
    Regular = 0,
    AllCaps = 1,
    SmallCaps = 2,
}
