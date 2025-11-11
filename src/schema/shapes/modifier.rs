use serde::{Deserialize, Serialize};

use crate::schema::shapes::graphic_element::GraphicElementShape;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifierShape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,
}
