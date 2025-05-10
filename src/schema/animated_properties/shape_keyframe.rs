// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::keyframe_base::KeyframeBase;
use crate::schema::helpers::bezier::Bezier;
use serde::{Deserialize, Serialize};

/// Keyframe holding Bezier objects
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ShapeKeyframe {
    #[serde(flatten)]
    pub base: KeyframeBase,
    /// Starting point of this keyframe. This is required for all keyframes, except the last keyframe.
    /// The last keyframe may omit this value, in which case the previous keyframe's "e" is used (or "s")
    /// if not present.
    #[serde(rename = "s")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Vec<Bezier>>,
    /// Ending point of this keyframe. Each keyframe may store it's own end, to allow for non-continuous animations.
    /// This means the ending point of one frame is not guaranteed to match the starting point of the next.
    /// This is always optional, and never required.
    #[serde(rename = "e")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Vec<Bezier>>,
}
