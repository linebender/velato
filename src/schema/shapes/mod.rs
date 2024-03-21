// Copyright 2024 the Velato Authors
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

#[cfg(test)]
mod tests {
    use super::{ellipse::EllipseShape, group::GroupShape, shape_element::ShapeElement, AnyShape};
    use crate::schema::{
        animated_properties::{
            animated_property::{AnimatedProperty, AnimatedPropertyK},
            multi_dimensional::MultiDimensional,
            position::{Position, PositionValueK},
        },
        helpers::{int_boolean::BoolInt, visual_object::VisualObject},
    };
    use once_cell::sync::Lazy;
    use serde_json::json;

    static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
        json!(
            {
                "ty": "gr",
                "nm": "Group",
                "mn": "{f1becc2a-49f0-4f0c-918f-bdffe4c6870f}",
                "it": [
                    {
                        "ty": "el",
                        "nm": "Ellipse",
                        "mn": "{2aabac6e-1dd8-41b0-b60b-baf75ccb6318}",
                        "p": {
                            "a": 0,
                            "k": [
                                303.9044776119403,
                                324.9671641791045
                            ]
                        },
                        "s": {
                            "a": 0,
                            "k": [
                                205.46865671641788,
                                204.6089552238806
                            ]
                        }
                    }
                ]
            }
        )
    });

    static LAYER: Lazy<AnyShape> = Lazy::new(|| {
        AnyShape::Group(GroupShape {
            shape_element: ShapeElement {
                visual_object: VisualObject {
                    name: Some("Group".to_string()),
                    match_name: Some("{f1becc2a-49f0-4f0c-918f-bdffe4c6870f}".to_string()),
                },
                index: None,
                hidden: None,
                blend_mode: None,
                property_index: None,
                css_class: None,
                xml_id: None,
            },
            num_properties: None,
            property_index: None,
            shapes: vec![AnyShape::Ellipse(EllipseShape {
                shape_element: ShapeElement {
                    visual_object: VisualObject {
                        name: Some("Ellipse".to_string()),
                        match_name: Some("{2aabac6e-1dd8-41b0-b60b-baf75ccb6318}".to_string()),
                    },
                    index: None,
                    hidden: None,
                    blend_mode: None,
                    property_index: None,
                    css_class: None,
                    xml_id: None,
                },
                position: Position {
                    property_index: None,
                    animated: Some(BoolInt::False),
                    expression: None,
                    length: None,
                    value: PositionValueK::Static(vec![303.9044776119403, 324.9671641791045]),
                },
                size: MultiDimensional {
                    animated_property: AnimatedProperty {
                        animated: Some(BoolInt::False),
                        property_index: None,
                        expression: None,
                        slot_id: None,
                        value: AnimatedPropertyK::Static(vec![
                            205.46865671641788,
                            204.6089552238806,
                        ]),
                    },
                    length: None,
                },
            })],
        })
    });

    #[test]
    fn test_deserialize() {
        let actual: Result<AnyShape, serde_json::Error> = serde_json::from_value(JSON.to_owned());

        match actual {
            Ok(actual) => assert_eq!(*LAYER, actual),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn test_can_serialize() {
        serde_json::to_value(&*LAYER).unwrap();
    }
}
