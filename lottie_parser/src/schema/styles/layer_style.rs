// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::helpers::visual_object::VisualObject;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Style Type
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum StyleType {
    Stroke = 0,
    DropShadow = 1,
    InnerShadow = 2,
    OuterGlow = 3,
    InnerGlow = 4,
    BevelEmboss = 5,
    Satin = 6,
    ColorOverlay = 7,
    GradientOverlay = 8,
}

/// Style applied to a layer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayerStyle {
    #[serde(flatten)]
    pub visual_object: VisualObject,
    /// Style Type
    #[serde(rename = "ty")]
    pub style_type: StyleType,
}
