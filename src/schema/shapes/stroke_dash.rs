// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::stroke_dash_type::StrokeDashType;
use crate::schema::helpers::visual_object::VisualObject;
use serde::{Deserialize, Serialize};

/// An item used to described the dashe pattern in a stroked path
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StrokeDashShape {
    #[serde(flatten)]
    visual_object: VisualObject,

    /// Type of the dash
    #[serde(rename = "n")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dash_type: Option<StrokeDashType>,

    /// Length of the dash
    #[serde(rename = "v")]
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<FloatValue>,
}
