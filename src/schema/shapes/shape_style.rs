// Copyright 2025 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

use crate::schema::{
    animated_properties::value::FloatValue, shapes::graphic_element::GraphicElementShape,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeStyleShape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,

    /// Opacity, 100 means fully opaque
    #[serde(rename = "o")]
    pub opacity: FloatValue,
}
