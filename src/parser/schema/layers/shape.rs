// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shapes - https://lottiefiles.github.io/lottie-docs/shapes/

use super::visual::VisualLayer;
use crate::parser::schema::shapes::AnyShape;
use serde::{Deserialize, Serialize};

#[derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum LayerId {
    Shape = 4,
}

/// Has an array of shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeLayer {
    #[serde(flatten)]
    pub properties: VisualLayer,

    /// Layer type, must be 4
    #[serde(rename = "ty")]
    pub layer_type: LayerId,

    /// Has an array of shapes
    #[serde(rename = "shapes")]
    pub shapes: Vec<AnyShape>,
}
