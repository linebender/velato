// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::builders::{setup_layer_base, setup_precomp_layer, setup_shape_layer};
use super::defaults::{FLOAT_VALUE_ONE_HUNDRED, FLOAT_VALUE_ZERO, MULTIDIM_ONE, POSITION_ZERO};
use crate::import::util::calc_stops;
use crate::parser::schema::animated_properties::keyframe_bezier_handle::{
    KeyframeBezierHandle, KeyframeComponent,
};
use crate::parser::schema::animated_properties::multi_dimensional::MultiDimensional;
use crate::parser::schema::animated_properties::split_vector::SplitVector;
use crate::parser::schema::constants::gradient_type::GradientType;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::parser::{self};
use crate::runtime::model::animated::{self, Position};
use crate::runtime::model::{
    self, Content, Draw, EasingHandle, GroupTransform, Layer, SplineToPath, Time, Tween, Value,
};
use crate::runtime::{self};
use parser::schema;
use vello::kurbo::{Cap, Join, Point, Size, Vec2};
use vello::peniko::{BlendMode, Color, Mix};

pub fn conv_layer(
    source: &parser::schema::layers::AnyLayer,
) -> Option<(Layer, usize, Option<BlendMode>)> {
    let mut layer = Layer::default();

    let params = match source {
        parser::schema::layers::AnyLayer::Null(null_layer) => {
            if let Some(true) = null_layer.properties.hidden {
                return None;
            }

            setup_layer_base(&null_layer.properties, &mut layer)
        }
        parser::schema::layers::AnyLayer::Precomposition(precomp_layer) => {
            if let Some(true) = precomp_layer.properties.hidden {
                return None;
            }

            let params = setup_precomp_layer(precomp_layer, &mut layer);
            let name = precomp_layer.precomp_id.clone();
            let time_remap = precomp_layer.time_remap.as_ref().map(conv_scalar);
            layer.content = Content::Instance { name, time_remap };

            params
        }
        parser::schema::layers::AnyLayer::Shape(shape_layer) => {
            if let Some(true) = shape_layer.properties.hidden {
                return None;
            }

            let params = setup_shape_layer(shape_layer, &mut layer);
            let mut shapes = vec![];
            for shape in &shape_layer.shapes {
                if let Some(shape) = conv_shape(shape) {
                    shapes.push(shape);
                }
            }
            layer.content = Content::Shape(shapes);

            params
        }
        schema::layers::AnyLayer::SolidColor(solid_color_layer) => {
            if let Some(true) = solid_color_layer.properties.hidden {
                return None;
            }

            setup_layer_base(&solid_color_layer.properties, &mut layer)
        }
    };

    let (id, matte_mode) = params;
    Some((layer, id, matte_mode))
}

pub fn conv_transform(
    value: &parser::schema::helpers::transform::Transform,
) -> (runtime::model::Transform, Value<f64>) {
    let rotation_in = match &value.rotation {
        Some(any_trans) => match any_trans {
            parser::schema::helpers::transform::AnyTransformR::Rotation(float_value) => float_value,
            // todo: need to actually handle split rotations
            parser::schema::helpers::transform::AnyTransformR::SplitRotation { .. } => todo!(),
        },
        None => todo!("split rotation"),
    };

    let position = match &value.position {
        schema::helpers::transform::AnyTransformP::Position(position) => {
            Position::Value(conv_pos_point(position))
        }
        schema::helpers::transform::AnyTransformP::SplitPosition(SplitVector { x, y, .. }) => {
            Position::SplitValues((conv_scalar(x), conv_scalar(y)))
        }
    };

    let transform = animated::Transform {
        anchor: conv_pos_point(value.anchor_point.as_ref().unwrap_or(&POSITION_ZERO)),
        position,
        scale: conv_vec2(value.scale.as_ref().unwrap_or(&MULTIDIM_ONE)),
        rotation: conv_scalar(rotation_in),
        skew: conv_scalar(value.skew.as_ref().unwrap_or(&FLOAT_VALUE_ZERO)),
        skew_angle: conv_scalar(value.skew_axis.as_ref().unwrap_or(&FLOAT_VALUE_ZERO)),
    };
    let opacity = conv_scalar(value.opacity.as_ref().unwrap_or(&FLOAT_VALUE_ONE_HUNDRED));
    (transform.to_model(), opacity)
}

pub fn conv_shape_transform(
    value: &parser::schema::shapes::transform::TransformShape,
) -> GroupTransform {
    let rotation_in = match &value.transform.rotation {
        Some(any_trans) => match any_trans {
            parser::schema::helpers::transform::AnyTransformR::Rotation(float_value) => float_value,
            // todo: need to actually handle split rotations
            parser::schema::helpers::transform::AnyTransformR::SplitRotation { .. } => {
                todo!("split rotation")
            }
        },
        None => &FLOAT_VALUE_ZERO,
    };
    let position_in = match &value.transform.position {
        schema::helpers::transform::AnyTransformP::Position(position) => position,
        schema::helpers::transform::AnyTransformP::SplitPosition(_) => {
            // todo: split vectors
            todo!("split position");
        }
    };

    let transform = animated::Transform {
        anchor: conv_pos_point(
            value
                .transform
                .anchor_point
                .as_ref()
                .unwrap_or(&POSITION_ZERO),
        ),
        position: Position::Value(conv_pos_point(position_in)),
        scale: conv_vec2(value.transform.scale.as_ref().unwrap_or(&MULTIDIM_ONE)),
        rotation: conv_scalar(rotation_in),
        skew: conv_scalar(value.transform.skew.as_ref().unwrap_or(&FLOAT_VALUE_ZERO)),
        skew_angle: conv_scalar(
            value
                .transform
                .skew_axis
                .as_ref()
                .unwrap_or(&FLOAT_VALUE_ZERO),
        ),
    };

    let opacity = conv_scalar(
        value
            .transform
            .opacity
            .as_ref()
            .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
    );

    GroupTransform {
        transform: transform.to_model(),
        opacity,
    }
}

pub fn conv_keyframes<'a, T: Tween>(
    keyframes: impl Iterator<Item = &'a schema::animated_properties::keyframe::Keyframe>,
    f: impl Fn(&schema::animated_properties::keyframe::Keyframe) -> T,
) -> Value<T> {
    let mut frames = vec![];
    let mut values = vec![];

    let collect_tangents = |handle: &Option<KeyframeBezierHandle>| {
        let mut handles = vec![];
        let Some(handle) = handle else { return handles };
        match (&handle.x_coordinate, &handle.y_coordinate) {
            (KeyframeComponent::ArrayOfValues(xarr), KeyframeComponent::ArrayOfValues(yarr)) => {
                handles.extend(
                    xarr.iter()
                        .zip(yarr)
                        .map(|(x, y)| EasingHandle { x: *x, y: *y }),
                );
            }
            (KeyframeComponent::ArrayOfValues(xarr), KeyframeComponent::SingleValue(y)) => {
                handles.extend(xarr.iter().map(|x| EasingHandle { x: *x, y: *y }));
            }
            (KeyframeComponent::SingleValue(x), KeyframeComponent::ArrayOfValues(yarr)) => {
                handles.extend(yarr.iter().map(|y| EasingHandle { x: *x, y: *y }));
            }
            (KeyframeComponent::SingleValue(x), KeyframeComponent::SingleValue(y)) => {
                handles.push(EasingHandle { x: *x, y: *y });
            }
        }
        handles
    };

    for keyframe in keyframes {
        let hold = keyframe
            .base
            .hold
            .as_ref()
            .map(|b| b.eq(&BoolInt::True))
            .unwrap_or(false);
        let in_tangents = collect_tangents(&keyframe.base.in_tangent);
        let out_tangents = collect_tangents(&keyframe.base.out_tangent);
        let tangents: Vec<_> = in_tangents.into_iter().zip(out_tangents).collect();
        if tangents.is_empty() {
            frames.push(Time {
                frame: keyframe.base.time,
                in_tangent: None,
                out_tangent: None,
                hold,
            });
            values.push(f(keyframe));
        } else {
            for (in_tangent, out_tangent) in tangents {
                frames.push(Time {
                    frame: keyframe.base.time,
                    in_tangent: Some(in_tangent),
                    out_tangent: Some(out_tangent),
                    hold,
                });
                values.push(f(keyframe));
            }
        }
    }
    Value::Animated(runtime::model::Animated {
        times: frames,
        values,
    })
}

fn conv_keyframe_handle(handle: &KeyframeBezierHandle) -> EasingHandle {
    let KeyframeBezierHandle {
        x_coordinate,
        y_coordinate,
    } = handle;
    let x = match x_coordinate {
        KeyframeComponent::ArrayOfValues(arr) => {
            assert!(arr.len() == 1, "{arr:?}");
            arr[0]
        }
        KeyframeComponent::SingleValue(v) => *v,
    };
    let y = match y_coordinate {
        KeyframeComponent::ArrayOfValues(arr) => {
            assert!(arr.len() == 1);
            arr[0]
        }
        KeyframeComponent::SingleValue(v) => *v,
    };
    EasingHandle { x, y }
}

fn conv_gradient_colors(
    value: &schema::animated_properties::gradient_colors::GradientColors,
) -> runtime::model::ColorStops {
    use schema::animated_properties::animated_property::AnimatedPropertyK::*;

    let count = value.count;
    match &value.colors.animated_property.value {
        Static(value) => runtime::model::ColorStops::Fixed({
            let mut stops = runtime::model::fixed::ColorStops::new();
            let raw = calc_stops(value, count);
            for values in raw {
                stops.push(
                    (
                        values[0] as f32,
                        runtime::model::fixed::Color::rgba(
                            values[1], values[2], values[3], values[4],
                        ),
                    )
                        .into(),
                );
            }
            stops
        }),
        AnimatedValue(animated) => {
            let mut frames = vec![];
            let mut values: Vec<Vec<f64>> = vec![];
            for value in animated {
                let hold = value
                    .base
                    .hold
                    .as_ref()
                    .map(|b| b.eq(&BoolInt::True))
                    .unwrap_or(false);
                frames.push(Time {
                    frame: value.base.time,
                    in_tangent: value.base.in_tangent.as_ref().map(conv_keyframe_handle),
                    out_tangent: value.base.out_tangent.as_ref().map(conv_keyframe_handle),
                    hold,
                });

                let stops = calc_stops(&value.value, count)
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();

                values.push(stops);
            }
            runtime::model::ColorStops::Animated(animated::ColorStops {
                frames,
                values,
                count,
            })
        }
    }
}

fn conv_draw(value: &schema::shapes::AnyShape) -> Option<runtime::model::Draw> {
    use schema::constants::line_cap::LineCap;
    use schema::constants::line_join::LineJoin;
    use schema::shapes::AnyShape;

    match value {
        AnyShape::Fill(value) => {
            let color = conv_color(&value.color);
            let brush = animated::Brush::Solid(color).to_model();
            let opacity = conv_scalar(value.opacity.as_ref().unwrap_or(&FLOAT_VALUE_ONE_HUNDRED));
            Some(runtime::model::Draw {
                stroke: None,
                brush,
                opacity,
            })
        }
        AnyShape::Stroke(value) => {
            let stroke = animated::Stroke {
                width: conv_scalar(&value.stroke_width),
                join: match value.line_join.as_ref().unwrap_or(&LineJoin::Bevel) {
                    LineJoin::Bevel => Join::Bevel,
                    LineJoin::Round => Join::Round,
                    LineJoin::Miter => Join::Miter,
                },
                miter_limit: value.miter_limit,
                cap: match value.line_cap.as_ref().unwrap_or(&LineCap::Butt) {
                    LineCap::Butt => Cap::Butt,
                    LineCap::Round => Cap::Round,
                    LineCap::Square => Cap::Square,
                },
            };
            let color = conv_color(&value.stroke_color);
            let brush = animated::Brush::Solid(color).to_model();
            let opacity = conv_scalar(&value.opacity);
            Some(runtime::model::Draw {
                stroke: Some(stroke.to_model()),
                brush,
                opacity,
            })
        }
        AnyShape::GradientFill(value) => {
            let is_radial = matches!(value.gradient.gradient_type, Some(GradientType::Radial));
            let start_point = conv_multi_point(&value.gradient.start_point);
            let end_point = conv_multi_point(&value.gradient.end_point);
            let gradient = animated::Gradient {
                is_radial,
                start_point,
                end_point,
                stops: conv_gradient_colors(&value.gradient.colors),
            };
            let brush = animated::Brush::Gradient(gradient).to_model();
            Some(Draw {
                stroke: None,
                brush,
                opacity: Value::Fixed(100.0),
            })
        }
        AnyShape::GradientStroke(value) => {
            let stroke = animated::Stroke {
                width: conv_scalar(&value.base_stroke.width),
                join: match value
                    .base_stroke
                    .line_join
                    .as_ref()
                    .unwrap_or(&LineJoin::Round)
                {
                    LineJoin::Bevel => Join::Bevel,
                    LineJoin::Round => Join::Round,
                    LineJoin::Miter => Join::Miter,
                },
                miter_limit: value.base_stroke.miter_limit,
                cap: match value
                    .base_stroke
                    .line_cap
                    .as_ref()
                    .unwrap_or(&LineCap::Round)
                {
                    LineCap::Butt => Cap::Butt,
                    LineCap::Round => Cap::Round,
                    LineCap::Square => Cap::Square,
                },
            };
            let is_radial = matches!(
                value
                    .gradient
                    .gradient_type
                    .as_ref()
                    .unwrap_or(&GradientType::Linear),
                GradientType::Radial
            );
            let start_point = conv_multi_point(&value.gradient.start_point);
            let end_point = conv_multi_point(&value.gradient.end_point);
            let gradient = animated::Gradient {
                is_radial,
                start_point,
                end_point,
                stops: conv_gradient_colors(&value.gradient.colors),
            };
            let brush = animated::Brush::Gradient(gradient).to_model();
            Some(Draw {
                stroke: Some(stroke.to_model()),
                brush,
                opacity: Value::Fixed(100.0),
            })
        }
        _ => None,
    }
}

fn conv_shape(value: &parser::schema::shapes::AnyShape) -> Option<crate::runtime::model::Shape> {
    if let Some(draw) = conv_draw(value) {
        return Some(crate::runtime::model::Shape::Draw(draw));
    } else if let Some(geometry) = conv_geometry(value) {
        return Some(crate::runtime::model::Shape::Geometry(geometry));
    }

    match value {
        schema::shapes::AnyShape::Group(value) => {
            let mut shapes = vec![];
            let mut group_transform = None;
            for item in &value.shapes {
                match item {
                    schema::shapes::AnyShape::Transform(transform) => {
                        group_transform = Some(conv_shape_transform(transform));
                    }
                    _ => {
                        if let Some(shape) = conv_shape(item) {
                            shapes.push(shape);
                        }
                    }
                }
            }
            if !shapes.is_empty() {
                Some(crate::runtime::model::Shape::Group(shapes, group_transform))
            } else {
                None
            }
        }
        // todo: implement repeater shape
        // shapes::Shape::Repeater(value) => {
        //     let repeater = animated::Repeater {
        //         copies: conv_scalar(&value.copies),
        //         offset: conv_scalar(&value.offset),
        //         anchor_point: conv_point(&value.transform.anchor_point),
        //         position: conv_point(&value.transform.position),
        //         rotation: conv_scalar(&value.transform.rotation),
        //         scale: conv_vec2(&value.transform.scale),
        //         start_opacity: conv_scalar(&value.transform.start_opacity),
        //         end_opacity: conv_scalar(&value.transform.end_opacity),
        //     };
        //     Some(Shape::Repeater(repeater.to_model()))
        // }
        _ => None,
    }
}

fn conv_geometry(value: &schema::shapes::AnyShape) -> Option<crate::runtime::model::Geometry> {
    use schema::shapes::AnyShape;
    match value {
        AnyShape::Ellipse(value) => {
            let ellipse = animated::Ellipse {
                is_ccw: false, /* todo: lottie schema does not have a field
                                * for this (anymore?) */
                position: conv_pos_point(&value.position),
                size: conv_size(&value.size),
            };
            Some(crate::runtime::model::Geometry::Ellipse(ellipse))
        }
        AnyShape::Rectangle(value) => {
            let rect = animated::Rect {
                is_ccw: false, /* todo: lottie schema does not have a field
                                * for this (anymore?) */
                position: conv_pos_point(&value.position),
                size: conv_size(&value.size),
                corner_radius: conv_scalar(&value.rounded_corner_radius),
            };
            Some(crate::runtime::model::Geometry::Rect(rect))
        }
        AnyShape::Path(value) => conv_shape_geometry(&value.shape_property),
        // todo: generic shape
        _ => None,
    }
}

pub fn conv_shape_geometry(
    value: &schema::animated_properties::shape_property::ShapeProperty,
) -> Option<runtime::model::Geometry> {
    use schema::animated_properties::shape_property::ShapePropertyK::*;
    let mut is_closed = false;
    match &value.value {
        Static(value) => {
            let (points, is_closed) = conv_spline(value);
            let mut path = vec![];
            points.as_slice().to_path(is_closed, &mut path);
            Some(runtime::model::Geometry::Fixed(path))
        }
        Animated(animated) => {
            let mut frames = vec![];
            let mut values = vec![];
            for value in animated {
                let hold = value
                    .base
                    .hold
                    .as_ref()
                    .map(|b| b.eq(&BoolInt::True))
                    .unwrap_or(false);
                frames.push(Time {
                    frame: value.base.time,
                    in_tangent: value.base.in_tangent.as_ref().map(conv_keyframe_handle),
                    out_tangent: value.base.out_tangent.as_ref().map(conv_keyframe_handle),
                    hold,
                });
                let (points, is_frame_closed) = conv_spline(value.start.first()?);
                values.push(points);
                is_closed |= is_frame_closed;
            }
            Some(runtime::model::Geometry::Spline(animated::Spline {
                is_closed,
                times: frames,
                values,
            }))
        }
    }
}

pub fn conv_spline(value: &schema::helpers::bezier::Bezier) -> (Vec<Point>, bool) {
    use core::iter::repeat;
    let mut points = Vec::with_capacity(value.vertices.len() * 3);
    let is_closed = value.closed.unwrap_or(false);

    for ((v, i), o) in value
        .vertices
        .iter()
        .zip(value.in_tangents.iter().chain(repeat(&[0.0, 0.0])))
        .zip(value.out_tangents.iter().chain(repeat(&[0.0, 0.0])))
    {
        points.push((v[0], v[1]).into());
        points.push((i[0], i[1]).into());
        points.push((o[0], o[1]).into());
    }
    (points, is_closed)
}

pub fn conv_blend_mode(
    value: &crate::parser::schema::constants::blend_mode::BlendMode,
) -> Option<BlendMode> {
    use crate::parser::schema::constants::blend_mode::BlendMode::*;

    Some(match value {
        Normal => return None,
        Multiply => BlendMode::from(Mix::Multiply),
        Screen => BlendMode::from(Mix::Screen),
        Overlay => BlendMode::from(Mix::Overlay),
        Darken => BlendMode::from(Mix::Darken),
        Lighten => BlendMode::from(Mix::Lighten),
        ColorDodge => BlendMode::from(Mix::ColorDodge),
        ColorBurn => BlendMode::from(Mix::ColorBurn),
        HardLight => BlendMode::from(Mix::HardLight),
        SoftLight => BlendMode::from(Mix::SoftLight),
        Difference => BlendMode::from(Mix::Difference),
        Exclusion => BlendMode::from(Mix::Exclusion),
        Hue => BlendMode::from(Mix::Hue),
        Saturation => BlendMode::from(Mix::Saturation),
        Color => BlendMode::from(Mix::Color),
        Luminosity => BlendMode::from(Mix::Luminosity),
        Add => unimplemented!(),
        HardMix => unimplemented!(),
    })
}

pub fn conv_scalar(
    float_value: &parser::schema::animated_properties::value::FloatValue,
) -> Value<f64> {
    use crate::parser::schema::animated_properties::animated_property::AnimatedPropertyK::*;
    match &float_value.animated_property.value {
        Static(number) => Value::Fixed(*number),
        AnimatedValue(keyframes) => {
            let mut frames = vec![];
            let mut values = vec![];
            for keyframe in keyframes {
                let start_time = keyframe.base.time;
                let data = keyframe.value[0];
                let hold = keyframe
                    .base
                    .hold
                    .as_ref()
                    .map(|b| b.eq(&BoolInt::True))
                    .unwrap_or(false);
                frames.push(crate::runtime::model::Time {
                    frame: start_time,
                    in_tangent: keyframe.base.in_tangent.as_ref().map(conv_keyframe_handle),
                    out_tangent: keyframe.base.out_tangent.as_ref().map(conv_keyframe_handle),
                    hold,
                });
                values.push(data);
                // todo: end_value deprecated but should we still push it if it
                // exists?
            }
            Value::Animated(model::Animated {
                times: frames,
                values,
            })
        }
    }
}

pub fn conv_multi<T: Tween>(
    multidimensional: &parser::schema::animated_properties::multi_dimensional::MultiDimensional,
    f: impl Fn(&Vec<f64>) -> T,
) -> Value<T> {
    use crate::parser::schema::animated_properties::animated_property::AnimatedPropertyK::*;

    match &multidimensional.animated_property.value {
        Static(components) => Value::Fixed(f(components)),
        AnimatedValue(keyframes) => conv_keyframes(keyframes.iter(), |k| f(&k.value)),
    }
}

pub fn conv_multi_color<T: Tween>(
    color: &parser::schema::animated_properties::color_value::ColorValue,
    f: impl Fn(&Vec<f64>) -> T,
) -> Value<T> {
    use crate::parser::schema::animated_properties::animated_property::AnimatedPropertyK::*;

    match &color.animated_property.value {
        Static(components) => Value::Fixed(f(components)),
        AnimatedValue(keyframes) => conv_keyframes(keyframes.iter(), |k| f(&k.value)),
    }
}

pub fn conv_pos<T: Tween>(
    position: &parser::schema::animated_properties::position::Position,
    f: impl Fn(&Vec<f64>) -> T,
) -> Value<T> {
    use crate::parser::schema::animated_properties::position::PositionValueK::*;

    match &position.value {
        Static(components) => Value::Fixed(f(components)),
        Animated(pos_keyframes) => {
            // TODO: Are we using PositionKeyframes here how we're supposed to?
            // there are in_tangents and out_tangents in addition to the keyframes.
            conv_keyframes(pos_keyframes.iter().map(|pk| &pk.keyframe), |k| f(&k.value))
        }
    }
}

#[allow(clippy::get_first)]
pub fn conv_pos_point(value: &schema::animated_properties::position::Position) -> Value<Point> {
    conv_pos(value, |x| {
        Point::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_multi_point(
    value: &schema::animated_properties::multi_dimensional::MultiDimensional,
) -> Value<Point> {
    conv_multi(value, |x| {
        Point::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_color(value: &schema::animated_properties::color_value::ColorValue) -> Value<Color> {
    conv_multi_color(value, |x| {
        Color::rgb(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
            x.get(2).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_vec2(value: &MultiDimensional) -> Value<Vec2> {
    conv_multi(value, |x| {
        Vec2::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}

#[allow(clippy::get_first)]
pub fn conv_size(value: &MultiDimensional) -> Value<Size> {
    conv_multi(value, |x| {
        Size::new(
            x.get(0).copied().unwrap_or(0.0),
            x.get(1).copied().unwrap_or(0.0),
        )
    })
}
