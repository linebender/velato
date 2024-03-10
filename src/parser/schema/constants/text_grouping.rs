use serde_repr::{Deserialize_repr, Serialize_repr};

///
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextGrouping {
    Characters = 1,
    Words = 2,
    Lines = 3,
    All = 4,
}
