// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

use crate::schema::constants::merge_mode::MergeMode;

use super::graphic_element::GraphicElementShape;

/// Boolean operator on shapes
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MergeShape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,

    /// Merge Mode
    #[serde(rename = "mm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_mode: Option<MergeMode>,
}
