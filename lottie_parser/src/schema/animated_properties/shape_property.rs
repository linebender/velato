// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::helpers::bezier::Bezier;
use crate::schema::helpers::int_boolean::BoolInt;
use serde::de::{self, Deserializer};
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
    /// See [`ShapePropertyK`]
    #[serde(rename = "k")]
    pub value: ShapePropertyK,
}

/// The possible values of "k" in a [`ShapeProperty`].
/// Either a static bezier held, or an animation with keyframes defined.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapePropertyK {
    #[serde(deserialize_with = "deserialize_keyframes")]
    Animated(Vec<ShapeKeyframe>),
    Static(Bezier),
}

/// Custom deserializer ensures that every keyframe has a "s" start, except only the last keyframe which may omit this.
pub fn deserialize_keyframes<'de, D>(deserializer: D) -> Result<Vec<ShapeKeyframe>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the keyframes & handle empty case
    let keyframes = Vec::<ShapeKeyframe>::deserialize(deserializer)?;
    if keyframes.is_empty() {
        return Ok(keyframes);
    }

    // Validate that all keyframes until the last have a start property
    if !keyframes[0..(keyframes.len() - 1)]
        .iter()
        .all(|keyframe| keyframe.start.is_some())
    {
        return Err(de::Error::custom(
            "Animated Shape Keyframe found with missing 's' start property. Only the last keyframe may omit this.",
        ));
    }

    // Early return if last keyframe has a start value.
    if let Some(last_keyframe) = keyframes.last() {
        if last_keyframe.start.is_some() {
            return Ok(keyframes);
        }
    }

    // The last keyframe has no start value - so there must be at least one other keyframe present.
    if keyframes.len() < 2 {
        Err(de::Error::custom(
            "Last Animated Shape Keyframe 's' was omitted where only one keyframe is present.",
        ))
    } else {
        Ok(keyframes)
    }
}
