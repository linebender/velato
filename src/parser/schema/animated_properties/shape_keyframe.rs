use super::keyframe_base::KeyframeBase;
use crate::parser::schema::helpers::bezier::Bezier;
use serde::{Deserialize, Serialize};

/// Keyframe holding Bezier objects
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeKeyframe {
    #[serde(flatten)]
    pub base: KeyframeBase,
    #[serde(rename = "s")]
    pub start: Vec<Bezier>,
}
