// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![recursion_limit = "512"]
use once_cell::sync::Lazy;
use serde_json::{json, Number};
use velato::parser::schema::animated_properties::animated_property::{
    AnimatedProperty, AnimatedPropertyK,
};
use velato::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use velato::parser::schema::animated_properties::position::{Position, PositionValueK};
use velato::parser::schema::helpers::int_boolean::BoolInt;
use velato::parser::schema::helpers::visual_object::VisualObject;
use velato::parser::schema::shapes::ellipse::EllipseShape;
use velato::parser::schema::shapes::group::GroupShape;
use velato::parser::schema::shapes::shape_element::ShapeElement;
use velato::parser::schema::shapes::AnyShape;

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
                value: PositionValueK::Static(vec![
                    Number::from_f64(303.9044776119403).unwrap(),
                    Number::from_f64(324.9671641791045).unwrap(),
                ]),
            },
            size: MultiDimensional {
                animated_property: AnimatedProperty {
                    animated: Some(BoolInt::False),
                    property_index: None,
                    expression: None,
                    slot_id: None,
                    value: AnimatedPropertyK::Static(vec![
                        Number::from_f64(205.46865671641788).unwrap(),
                        Number::from_f64(204.6089552238806).unwrap(),
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
fn test_serialize() {
    let actual = serde_json::to_value(&*LAYER).unwrap();

    assert_eq!(*JSON, actual)
}