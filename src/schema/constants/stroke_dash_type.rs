// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// Type of a dash item in a stroked line
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StrokeDashType {
    #[serde(rename = "d")]
    Dash,
    #[serde(rename = "g")]
    Gap,
    #[serde(rename = "o")]
    Offset,
}
