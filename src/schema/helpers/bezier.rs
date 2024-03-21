// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// This represents a cubic bezier path.
/// Note that for interpolation to work correctly all bezier values in a
/// property's keyframe must have the same number of points.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bezier {
    /// Whether the bezier forms a closed loop
    #[serde(rename = "c")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,

    /// Points along the curve
    #[serde(rename = "v")]
    pub vertices: Vec<[f64; 2]>,

    /// Cubic control points, incoming tangent
    #[serde(rename = "i")]
    pub in_tangents: Vec<[f64; 2]>,

    /// Cubic control points, outgoing tangent
    #[serde(rename = "o")]
    pub out_tangents: Vec<[f64; 2]>,
}
