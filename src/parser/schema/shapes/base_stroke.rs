// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::stroke_dash::StrokeDash;
use crate::parser::schema::animated_properties::value::FloatValue;
use crate::parser::schema::constants::line_cap::LineCap;
use crate::parser::schema::constants::line_join::LineJoin;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BaseStroke {
    /// Line Cap
    #[serde(rename = "lc")]
    pub line_cap: Option<LineCap>,

    /// Line Join
    #[serde(rename = "lj")]
    pub line_join: Option<LineJoin>,

    /// Miter Limit
    #[serde(rename = "ml")]
    pub miter_limit: Option<Number>,

    /// Animatable alternative to miter limit
    #[serde(rename = "ml2")]
    pub miter_limit_anim: Option<FloatValue>,

    /// Opacity
    #[serde(rename = "o")]
    pub opacity: FloatValue,

    /// Stroke Width
    #[serde(rename = "w")]
    pub width: FloatValue,

    /// Dashed line definition
    #[serde(rename = "d")]
    pub dashes: Option<Vec<StrokeDash>>,
}
