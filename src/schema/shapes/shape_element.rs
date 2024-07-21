// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::constants::blend_mode::BlendMode;
use crate::schema::helpers::visual_object::VisualObject;
use serde::{Deserialize, Serialize};

/// Base class for all elements of `ShapeLayer` and `Group`
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeElement {
    #[serde(flatten)]
    pub visual_object: VisualObject,

    /// Whether the shape is hidden
    #[serde(rename = "hd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,

    /// Index used in expressions
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<f64>,

    /// CSS class used by the SVG renderer
    #[serde(rename = "cl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,

    /// `id` attribute used by the SVG renderer
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,

    /// TODO: This is an unknown property, but it showed up sometimes in test
    /// files.
    #[serde(rename = "ind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
}
