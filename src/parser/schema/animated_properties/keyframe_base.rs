use super::keyframe_bezier_handle::KeyframeBezierHandle;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// A Keyframes specifies the value at a specific time and the interpolation
/// function to reach the next keyframe.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyframeBase {
    /// Time
    #[serde(rename = "t")]
    pub time: Number,
    /// Hold
    #[serde(rename = "h")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold: Option<BoolInt>,
    /// In tangent of the keyframe.
    /// Easing tangent going into the next keyframe.
    #[serde(rename = "i")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_tangent: Option<KeyframeBezierHandle>,
    /// Out tangent of the keyframe.
    /// Easing tangent leaving the current keyframe.
    #[serde(rename = "o")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_tangent: Option<KeyframeBezierHandle>,
}
