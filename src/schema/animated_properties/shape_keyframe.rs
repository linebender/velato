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
    #[serde(rename = "s")]
    pub start: Vec<Bezier>,
}
