use serde::{Deserialize, Serialize};
use serde_json::Number;

/// Defines named portions of the composition

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Marker {
    /// Comment
    #[serde(rename = "cm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Time
    #[serde(rename = "tm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Number>,

    /// Duration
    #[serde(rename = "dr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Number>,
}
