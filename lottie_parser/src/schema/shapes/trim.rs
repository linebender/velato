// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::animated_properties::value::FloatValue;
use crate::schema::constants::trim_multiple_shapes::TrimMultipleShapes;
use serde::{Deserialize, Serialize};

use super::shape_element::ShapeElement;

/// Trims shapes into a segment.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TrimShape {
    #[serde(flatten)]
    pub shape_element: ShapeElement,

    /// Segment start.
    #[serde(rename = "s")]
    pub start: FloatValue,

    /// Segment end.
    #[serde(rename = "e")]
    pub end: FloatValue,

    /// Offset.
    #[serde(rename = "o")]
    pub offset: FloatValue,

    /// How to treat multiple copies.
    #[serde(rename = "m")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<TrimMultipleShapes>,
}
