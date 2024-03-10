use serde_repr::{Deserialize_repr, Serialize_repr};

/// Style at the end of a stoked line
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum LineCap {
    ///
    Butt = 1,
    ///
    Round = 2,
    ///
    Square = 3,
}
