// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::keyframe::Keyframe;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// An animatable property that holds an array of numbers.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimatedProperty<StaticType> {
    /// Property Index
    #[serde(rename = "ix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_index: Option<Number>,
    /// Whether the property is animated.
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<BoolInt>,
    /// Expression for the property.
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// One of the ID in the file's slots
    #[serde(rename = "sid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "k")]
    #[serde(bound = "StaticType: Serialize + DeserializeOwned")]
    pub value: AnimatedPropertyK<StaticType>,
}

/// The possible values of "k" in an [`AnimatedProperty`].
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnimatedPropertyK<StaticType> {
    /// Array of keyframes
    AnimatedValue(Vec<Keyframe>),
    /// Static value
    Static(StaticType),
}
