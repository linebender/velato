// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// Defines named portions of the composition

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Marker {
    /// Comment
    #[serde(rename = "cm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// Time
    #[serde(rename = "tm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,

    /// Duration
    #[serde(rename = "dr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}
