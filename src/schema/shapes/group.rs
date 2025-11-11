// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::AnyShape;
use super::graphic_element::GraphicElementShape;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GroupShape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,
    /// Number of properties
    #[serde(rename = "np")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_properties: Option<f64>,
    /// Array of shapes
    #[serde(rename = "it")]
    pub shapes: Vec<AnyShape>,
    /// Property Index
    #[serde(rename = "cix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<u8>,
}
