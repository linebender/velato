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
pub struct Layer {
    /// Visual object dat   K
    #[serde(flatten)]
    pub visual_object: VisualObject,
    /// Whether the layer is 3D. Lottie doesn't actually support 3D stuff so
    /// this should always be 0
    #[serde(rename = "ddd", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_dimensional: Option<BoolInt>,
    /// Whether the layer is hidden
    #[serde(rename = "hd", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Layer type
    #[serde(rename = "ty")]
    pub layer_type: u8,
    /// Layer index for parenting
    #[serde(rename = "ind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    /// Parent index for parenting
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_index: Option<usize>,
    /// Time Stretch
    #[serde(rename = "sr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_stretch: Option<f64>,
    /// Frame when the layer becomes visible
    #[serde(rename = "ip")]
    pub in_point: f64,
    /// Frame when the layer becomes invisible
    #[serde(rename = "op")]
    pub out_point: f64,
    /// Start Time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "st")]
    pub start_time: Option<f64>,
}
