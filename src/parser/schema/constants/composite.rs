use serde_repr::{Deserialize_repr, Serialize_repr};

/// How to stack copies in a repeater
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum Composite {
    Above = 1,
    Below = 2,
}
