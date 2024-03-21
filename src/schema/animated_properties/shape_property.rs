// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::helpers::bezier::Bezier;
use crate::schema::helpers::int_boolean::BoolInt;
use serde::{Deserialize, Serialize};

use super::shape_keyframe::ShapeKeyframe;

/// An animatable property that holds a Bezier
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeProperty {
    /// The index of the property.
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<f64>,
    /// Whether the property is animated
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<BoolInt>,
    /// The expression for the property.
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    ///
    #[serde(rename = "k")]
    pub value: ShapePropertyK,
}

/// The possible values of "k" in a [`ShapeProperty`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapePropertyK {
    Animated(Vec<ShapeKeyframe>),
    Static(Bezier),
}
