use serde::{Deserialize, Serialize};

/// Type of a dash item in a stroked line
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StrokeDashType {
    #[serde(rename = "d")]
    Dash,
    #[serde(rename = "g")]
    Gap,
    #[serde(rename = "o")]
    Offset,
}
