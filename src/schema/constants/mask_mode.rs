// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// How masks interact with each other. See <https://helpx.adobe.com/after-effects/using/alpha-channels-masks-mattes.html>.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub enum MaskMode {
    #[serde(rename = "n")]
    None,
    #[serde(rename = "a")]
    Add,
    #[serde(rename = "s")]
    Subtract,
    #[serde(rename = "i")]
    #[default]
    Intersect,
    #[serde(rename = "l")]
    Lighten,
    #[serde(rename = "d")]
    Darken,
    #[serde(rename = "f")]
    Difference,
}
