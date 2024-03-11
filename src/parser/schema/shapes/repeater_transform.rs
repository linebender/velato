// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::parser::schema::shapes::FloatValue;
use serde::{Deserialize, Serialize};

/// Transform used by a repe…equent repeated object.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RepeaterTransformShape {
    /// Transform used by a repeater, the transform is applied to each
    /// subsequent repeated object.
    #[serde(rename = "so")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_opacity: Option<FloatValue>,

    // Opacity of the last repeated object
    #[serde(rename = "eo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_opacity: Option<FloatValue>,
}
