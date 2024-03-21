// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::multi_dimensional::MultiDimensional;
use serde::{Deserialize, Serialize};

/// Represents colors and offsets in a gradient.
///
/// Colors are represented as a flat list interleaving offsets and color
/// components. There are two possible layouts:
/// - Without alpha, the colors are a sequence of offset, r, g, b
/// - With alpha, same as above but at the end of the list there is a sequence
///   of offset, alpha
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientColors {
    /// Colors in the gradient.
    #[serde(rename = "k")]
    pub colors: MultiDimensional,
    /// Number of colors in k.
    #[serde(rename = "p")]
    pub count: usize,
}
