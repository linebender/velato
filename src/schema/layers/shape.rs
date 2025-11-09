// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shapes - <https://lottiefiles.github.io/lottie-docs/shapes/>

use super::visual::VisualLayer;
use crate::schema::shapes::AnyShape;
use serde::{Deserialize, Serialize};

/// Has an array of shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeLayer {
    /// Visual layer data
    #[serde(flatten)]
    pub visual_layer: crate::schema::layers::visual::VisualLayer,
    /// Has an array of shapes
    #[serde(rename = "shapes")]
    pub shapes: Vec<AnyShape>,
}
