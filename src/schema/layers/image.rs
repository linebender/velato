// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

use crate::schema::{
    animated_properties::value::FloatValue,
    helpers::{int_boolean::BoolInt, visual_object::VisualObject},
};

use super::visual::VisualLayer;

/// Renders a Precomposition
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ImageLayer {
    /// Visual layer data
    #[serde(flatten)]
    pub visual_layer: VisualLayer,
    /// ID of the image asset as specified in the assets
    #[serde(rename = "refId")]
    pub ref_id: String,
}
