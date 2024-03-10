use serde_repr::{Deserialize_repr, Serialize_repr};

/// Layer type
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum LayerType {
    Precomposition = 0,
    SolidColor = 1,
    Image = 2,
    Null = 3,
    Shape = 4,
    Text = 5,
    Audio = 6,
    VideoPlaceholder = 7,
    ImageSequence = 8,
    Video = 9,
    ImagePlaceholder = 10,
    Guide = 11,
    Adjustment = 12,
    Camera = 13,
    Light = 14,
    Data = 15,
}
