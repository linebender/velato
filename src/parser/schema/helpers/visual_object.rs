use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Default, Debug, Clone)]
pub struct VisualObject {
    /// Name, as seen from editors and the like
    #[serde(rename = "nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Match name, used in expressions
    #[serde(rename = "mn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_name: Option<String>,
}
