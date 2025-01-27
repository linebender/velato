// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod enumerations;
pub mod null;
pub mod precomposition;
pub mod shape;
pub mod solid_color;
pub mod visual;

use self::solid_color::SolidColorLayer;
use self::visual::VisualLayer;
use null::NullLayer;
use precomposition::PrecompositionLayer;
use serde::{Deserialize, Serialize};
use shape::ShapeLayer;

/// There are several layer types, which is specified by the 'ty' attribute. All
/// layers share the properties in `layers::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyLayer {
    /// Renders a Precomposition
    Precomposition(PrecompositionLayer),

    /// Static rectangle filling the canvas with a single color
    SolidColor(SolidColorLayer),

    /// Renders an Image
    // todo Image

    /// No contents, only used for parenting

    /// Has an array of shapes
    Shape(ShapeLayer),
    // Renders Text
    // todo Text
    Null(NullLayer),
    // unimplemented - Audio(AudioLayer),
    // unimplemented - VideoPlaceholder(VideoPlaceholderLayer)
    // unimplemented - Video(VideoLayer)
    // unimplemented - ImagePlaceholder(ImagePlaceholderLayer)
    // unimplemented - Guide(GuideLayer)
    // unimplemented - Adjustment(AdjustmentLayer)
    // unimplemented - Camera(CameraLayer)
    // unimplemented - Light(LightLayer)
    // unimplemented - Data(DataLayer)
}

#[cfg(test)]
mod tests {

    use crate::schema::{
        animated_properties::{
            animated_property::{AnimatedProperty, AnimatedPropertyK},
            multi_dimensional::MultiDimensional,
            position::{Position, PositionValueK},
            value::FloatValue,
        },
        helpers::{
            int_boolean::BoolInt,
            transform::{AnyTransformP, AnyTransformR, Transform},
            visual_object::VisualObject,
        },
        shapes::{ellipse::EllipseShape, group::GroupShape, shape_element::ShapeElement, AnyShape},
    };

    use super::{
        shape::{LayerId, ShapeLayer},
        visual::VisualLayer,
        AnyLayer,
    };
    use once_cell::sync::Lazy;
    use serde_json::json;

    static JSON: Lazy<serde_json::Value> = Lazy::new(|| {
        json!(
            {
                "ddd": 0,
                "ty": 4,
                "ind": 1,
                "st": 0,
                "ip": 0,
                "op": 180,
                "nm": "Ellipse",
                "mn": "{0a36d01c-18e1-48d3-8e8f-cc093b3f24ba}",
                "ks": {
                    "a": {
                        "a": 0,
                        "k": [
                            256,
                            256
                        ]
                    },
                    "p": {
                        "a": 0,
                        "k": [
                            256,
                            256
                        ]
                    },
                    "s": {
                        "a": 0,
                        "k": [
                            100,
                            100
                        ]
                    },
                    "r": {
                        "a": 0,
                        "k": 0
                    },
                    "o": {
                        "a": 0,
                        "k": 100
                    }
                },
                "shapes": [
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
                ]
            }
        )
    });

    #[expect(deprecated, reason = "Uses deprecated attributes")]
    static LAYER: Lazy<AnyLayer> = Lazy::new(|| {
        AnyLayer::Shape(ShapeLayer {
            properties: VisualLayer {
                name: Some("Ellipse".to_string()),
                match_name: Some("{0a36d01c-18e1-48d3-8e8f-cc093b3f24ba}".to_string()),
                three_dimensional: Some(BoolInt::False),
                index: Some(1),
                start_time: 0.0,
                in_point: 0.0,
                out_point: 180.0,
                transform: Transform {
                    anchor_point: Some(Position {
                        property_index: None,
                        animated: Some(BoolInt::False),
                        expression: None,
                        length: None,
                        value: PositionValueK::Static(vec![256.0, 256.0]),
                    }),
                    position: AnyTransformP::Position(Position {
                        property_index: None,
                        animated: Some(BoolInt::False),
                        expression: None,
                        length: None,
                        value: PositionValueK::Static(vec![256.0, 256.0]),
                    }),
                    scale: Some(MultiDimensional {
                        animated_property: AnimatedProperty {
                            animated: Some(BoolInt::False),
                            property_index: None,
                            expression: None,
                            slot_id: None,
                            value: AnimatedPropertyK::Static(vec![100.0, 100.0]),
                        },
                        length: None,
                    }),
                    rotation: Some(AnyTransformR::Rotation(FloatValue {
                        animated_property: AnimatedProperty {
                            animated: Some(BoolInt::False),
                            property_index: None,
                            expression: None,
                            slot_id: None,
                            value: AnimatedPropertyK::Static(0.0),
                        },
                    })),
                    opacity: Some(FloatValue {
                        animated_property: AnimatedProperty {
                            animated: Some(BoolInt::False),
                            property_index: None,
                            expression: None,
                            slot_id: None,
                            value: AnimatedPropertyK::Static(100.0),
                        },
                    }),
                    skew: None,
                    skew_axis: None,
                },
                hidden: None,
                parent_index: None,
                time_stretch: None,
                matte_mode: None,
                matte_target: None,
                masks_properties: None,
                rotate_to_match_anim_pos_path: None,
                matte_layer_index: None,
                has_mask: None,
                motion_blur: None,
                blend_mode: None,
                css_class: None,
                id: None,
                tag_name: None,
                tranform_before_mask_deprecated: None,
                transform_before_mask: None,
            },
            layer_type: LayerId::Shape,
            shapes: vec![AnyShape::Group(GroupShape {
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
            })],
        })
    });

    #[test]
    fn test_deserialize() {
        let actual = serde_json::from_value(JSON.to_owned());

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
