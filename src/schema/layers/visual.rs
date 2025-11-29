// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::constants::matte_mode::MatteMode;
use crate::schema::helpers::int_boolean::BoolInt;
use crate::schema::helpers::mask::Mask;
use crate::schema::helpers::transform::Transform;
use crate::schema::{constants::blend_mode::BlendMode, layers::layer::Layer};
use serde::{Deserialize, Serialize};

/// Common properties between layers
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VisualLayer {
    /// layer data
    #[serde(flatten)]
    pub layer: Layer,

    /// Matte mode
    #[serde(rename = "tt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matte_mode: Option<MatteMode>,
    /// Matte target
    #[serde(rename = "td", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matte_target: Option<BoolInt>,
    /// Masks for the layer
    #[serde(rename = "masksProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masks_properties: Option<Vec<Mask>>,

    // TODO:
    /// Effects for the layer
    //#[serde(rename = "ef")]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub effects: Option<Vec<()>>,
    /// Layer styles
    //#[serde(rename = "sy")]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub styles: Option<Vec<()>>,
    /// Layer transform
    #[serde(rename = "ks")]
    pub transform: Transform,
    /// If 1, The layer will rotate itself to match its animated position path
    #[serde(rename = "ao", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_to_match_anim_pos_path: Option<BoolInt>,
    /// Index of the layer used as matte, if omitted assume the layer above the
    /// current one
    #[serde(rename = "tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matte_layer_index: Option<f64>,
    /// Whether the layer has masks applied
    #[serde(rename = "hasMask", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_mask: Option<bool>,
    /// Whether motion blur is enabled for the layer
    #[serde(rename = "mb", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_blur: Option<bool>,
    /// Blend Mode
    #[serde(rename = "bm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    /// CSS class used by the SVG renderer
    #[serde(rename = "cl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css_class: Option<String>,
    /// id attribute used by the SVG renderer
    #[serde(rename = "ln")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Tag name used by the SVG renderer
    #[serde(rename = "tg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    /// This is deprecated in favour of `transform_before_mask`
    #[deprecated(note = "please use `transform_before_mask` instead")]
    #[serde(rename = "cp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tranform_before_mask_deprecated: Option<String>,
    /// Marks that transforms should be applied before masks
    #[serde(rename = "ct", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_before_mask: Option<BoolInt>,
}
