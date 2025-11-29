// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::stroke_dash::StrokeDashShape;
use crate::schema::constants::line_cap::LineCap;
use crate::schema::constants::line_join::LineJoin;
use crate::schema::{
    animated_properties::value::FloatValue, shapes::graphic_element::GraphicElementShape,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BaseStrokeShape {
    /// Line Cap
    #[serde(rename = "lc")]
    pub line_cap: Option<LineCap>,

    /// Line Join
    #[serde(rename = "lj")]
    pub line_join: Option<LineJoin>,

    /// Miter Limit
    #[serde(rename = "ml")]
    pub miter_limit: Option<f64>,

    /// Animatable alternative to miter limit
    #[serde(rename = "ml2")]
    pub miter_limit_anim: Option<FloatValue>,

    /// Stroke Width
    #[serde(rename = "w")]
    pub width: FloatValue,

    /// Dashed line definition
    #[serde(rename = "d")]
    pub dashes: Option<Vec<StrokeDashShape>>,
}
