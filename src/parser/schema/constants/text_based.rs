use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TextedBased {
    Characters = 1,
    CharacterExcludingSpaces = 2,
    Words = 3,
    Lines = 4,
}
