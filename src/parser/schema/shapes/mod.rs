// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod ellipse;
pub mod fill;
pub mod gradient_stroke;
pub mod group;
pub mod merge;
pub mod offset_path;
pub mod path;
pub mod polystar;
pub mod pucker_bloat;
pub mod rectangle;
pub mod repeater;
pub mod repeater_transform;
pub mod shape;
pub mod shape_element;
pub mod stroke;
pub mod stroke_dash;
pub mod transform;
pub mod trim;
// todo pub mod stroke_dash;
// todo pub mod shape_list;
// todo pub mod zig_zag;
// todo pub mod no_style;
pub mod base_stroke;
// todo pub mod twist;
// todo pub mod rounded_corners;
pub mod gradient;
pub mod gradient_fill;
// todo pub mod modifier;

use self::fill::FillShape;
use self::gradient_fill::GradientFillShape;
use self::gradient_stroke::GradientStrokeShape;
use self::merge::MergeShape;
use self::offset_path::OffsetPathShape;
use self::path::PathShape;
use self::pucker_bloat::PuckerBloatShape;
use self::rectangle::RectangleShape;
use self::repeater::RepeaterShape;
use self::stroke::StrokeShape;
use self::transform::TransformShape;
use self::trim::TrimShape;
use super::animated_properties::value::FloatValue;
use ellipse::EllipseShape;
use group::GroupShape;
use serde::{Deserialize, Serialize};

/// Lottie considers everything related to vector data as a "shape". All shapes
/// share the properties in `shapes::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "ty")]
#[allow(clippy::large_enum_variant)]
pub enum AnyShape {
    /// A group is a shape that can contain other shapes (including other
    /// groups)
    #[serde(rename = "gr")]
    Group(GroupShape),
    /// A rectangle, defined by its center point and size.
    #[serde(rename = "rc")]
    Rectangle(RectangleShape),
    /// An ellipse, defined by its center point and width and height.
    #[serde(rename = "el")]
    Ellipse(EllipseShape),
    #[serde(rename = "tr")]
    Transform(TransformShape),
    #[serde(rename = "st")]
    Stroke(StrokeShape),
    #[serde(rename = "pb")]
    PuckerBloat(PuckerBloatShape),
    #[serde(rename = "mm")]
    Merge(MergeShape),
    #[serde(rename = "rp")]
    Repeater(RepeaterShape),
    #[serde(rename = "op")]
    OffsetPath(OffsetPathShape),
    #[serde(rename = "fl")]
    Fill(FillShape),
    #[serde(rename = "tm")]
    Trim(TrimShape),
    #[serde(rename = "sh")]
    Path(PathShape),
    #[serde(rename = "gf")]
    GradientFill(GradientFillShape),
    #[serde(rename = "gs")]
    GradientStroke(GradientStrokeShape),
    // TODO: model other shapes
    // todo ZigZag(zig_zag),
    // todo no_style(no_style),
    // todo Twist(twist),
    // todo RoundedCorners(rounded_corners),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ShapeType {
    #[serde(rename = "rc")]
    Rectangle,
    #[serde(rename = "el")]
    Ellipse,
    #[serde(rename = "sr")]
    PolyStar,
    #[serde(rename = "sh")]
    Path,
    #[serde(rename = "fl")]
    Fill,
    #[serde(rename = "st")]
    Stroke,
    #[serde(rename = "gf")]
    GradientFill,
    #[serde(rename = "gs")]
    GradientStroke,
    #[serde(rename = "no")]
    NoStyle,
    #[serde(rename = "gr")]
    Group,
    #[serde(rename = "tr")]
    Transform,
    #[serde(rename = "rp")]
    Repeater,
    #[serde(rename = "tm")]
    Trim,
    #[serde(rename = "rd")]
    RoundedCorners,
    #[serde(rename = "pb")]
    PuckerBloat,
    #[serde(rename = "mm")]
    Merge,
    #[serde(rename = "tw")]
    Twist,
    #[serde(rename = "op")]
    OffsetPath,
    #[serde(rename = "zz")]
    ZigZag,
}
