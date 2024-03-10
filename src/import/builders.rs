// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::converters::{conv_blend_mode, conv_scalar, conv_shape_geometry, conv_transform};
use super::defaults::FLOAT_VALUE_ONE_HUNDRED;
use super::NumberExt;
use crate::parser::schema::helpers::int_boolean::BoolInt;
use crate::runtime::model::Layer;
use crate::{parser, runtime};
use parser::schema;
use vello::peniko::{self, BlendMode, Compose, Mix};

pub fn setup_precomp_layer(
    source: &parser::schema::layers::precomposition::PrecompositionLayer,
    target: &mut Layer,
) -> (usize, Option<BlendMode>) {
    target.name = source.properties.name.clone().unwrap_or_default();
    target.parent = source
        .properties
        .parent_index
        .as_ref()
        .map(|i| i.unwrap_u32() as usize);
    let (transform, opacity) = conv_transform(&source.properties.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.width = source.width.unwrap_u32();
    target.height = source.height.unwrap_u32();
    target.is_mask = source
        .properties
        .matte_target
        .as_ref()
        .map_or(false, |td| *td == BoolInt::True);

    let matte_mode = source
        .properties
        .matte_mode
        .as_ref()
        .map(|mode| match mode {
            schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
            schema::constants::matte_mode::MatteMode::Alpha
            | schema::constants::matte_mode::MatteMode::Luma => Compose::SrcIn.into(),
            schema::constants::matte_mode::MatteMode::InvertedAlpha
            | schema::constants::matte_mode::MatteMode::InvertedLuma => Compose::SrcOut.into(),
        });

    target.blend_mode = conv_blend_mode(
        source
            .properties
            .blend_mode
            .as_ref()
            .unwrap_or(&crate::parser::schema::constants::blend_mode::BlendMode::Normal),
    );
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.stretch = source
        .properties
        .time_stretch
        .as_ref()
        .map_or(1.0, |sr| sr.unwrap_f32());
    target.frames =
        source.properties.in_point.unwrap_f32()..source.properties.out_point.unwrap_f32();
    target.start_frame = source.properties.start_time.unwrap_f32();

    for mask_source in source
        .properties
        .masks_properties
        .as_ref()
        .unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape {
            if let Some(geometry) = conv_shape_geometry(shape) {
                let mode = peniko::BlendMode::default();
                let opacity = conv_scalar(
                    mask_source
                        .opacity
                        .as_ref()
                        .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
                );
                target.masks.push(runtime::model::Mask {
                    mode,
                    geometry,
                    opacity,
                })
            }
        }
    }

    (
        source
            .properties
            .index
            .as_ref()
            .map_or(0, |ind| ind.unwrap_u32()) as usize,
        matte_mode,
    )
}

pub fn setup_shape_layer(
    source: &parser::schema::layers::shape::ShapeLayer,
    target: &mut Layer,
) -> (usize, Option<BlendMode>) {
    target.name = source.properties.name.clone().unwrap_or_default();
    target.parent = source
        .properties
        .parent_index
        .as_ref()
        .map(|i| i.unwrap_u32() as usize);
    let (transform, opacity) = conv_transform(&source.properties.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.is_mask = source
        .properties
        .matte_target
        .as_ref()
        .map_or(false, |td| *td == BoolInt::True);

    let matte_mode = source
        .properties
        .matte_mode
        .as_ref()
        .map(|mode| match mode {
            schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
            schema::constants::matte_mode::MatteMode::Alpha
            | schema::constants::matte_mode::MatteMode::Luma => Compose::SrcIn.into(),
            schema::constants::matte_mode::MatteMode::InvertedAlpha
            | schema::constants::matte_mode::MatteMode::InvertedLuma => Compose::SrcOut.into(),
        });

    target.blend_mode = conv_blend_mode(
        source
            .properties
            .blend_mode
            .as_ref()
            .unwrap_or(&crate::parser::schema::constants::blend_mode::BlendMode::Normal),
    );
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.stretch = source
        .properties
        .time_stretch
        .as_ref()
        .map_or(1.0, |sr| sr.unwrap_f32());
    target.frames =
        source.properties.in_point.unwrap_f32()..source.properties.out_point.unwrap_f32();
    target.start_frame = source.properties.start_time.unwrap_f32();

    for mask_source in source
        .properties
        .masks_properties
        .as_ref()
        .unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape {
            if let Some(geometry) = conv_shape_geometry(shape) {
                let mode = peniko::BlendMode::default();
                let opacity = conv_scalar(
                    mask_source
                        .opacity
                        .as_ref()
                        .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
                );
                target.masks.push(runtime::model::Mask {
                    mode,
                    geometry,
                    opacity,
                })
            }
        }
    }

    (
        source
            .properties
            .index
            .as_ref()
            .map_or(0, |ind| ind.unwrap_u32()) as usize,
        matte_mode,
    )
}

pub fn setup_layer_base(
    source: &parser::schema::layers::visual::VisualLayer,
    target: &mut Layer,
) -> (usize, Option<BlendMode>) {
    target.name = source.name.clone().unwrap_or_default();
    target.parent = source
        .parent_index
        .as_ref()
        .map(|i| i.unwrap_u32() as usize);
    let (transform, opacity) = conv_transform(&source.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.is_mask = source
        .matte_target
        .as_ref()
        .map_or(false, |td| *td == BoolInt::True);

    let matte_mode = source.matte_mode.as_ref().map(|mode| match mode {
        schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
        schema::constants::matte_mode::MatteMode::Alpha
        | schema::constants::matte_mode::MatteMode::Luma => Compose::SrcIn.into(),
        schema::constants::matte_mode::MatteMode::InvertedAlpha
        | schema::constants::matte_mode::MatteMode::InvertedLuma => Compose::SrcOut.into(),
    });

    target.blend_mode = conv_blend_mode(
        source
            .blend_mode
            .as_ref()
            .unwrap_or(&crate::parser::schema::constants::blend_mode::BlendMode::Normal),
    );
    // TODO: Why do we do this next part?
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.stretch = source
        .time_stretch
        .as_ref()
        .map_or(1.0, |sr| sr.unwrap_f32());
    target.frames = source.in_point.unwrap_f32()..source.out_point.unwrap_f32();
    target.start_frame = source.start_time.unwrap_f32();

    for mask_source in source.masks_properties.as_ref().unwrap_or(&Vec::default()) {
        if let Some(shape) = &mask_source.shape {
            if let Some(geometry) = conv_shape_geometry(shape) {
                let mode = peniko::BlendMode::default();
                let opacity = conv_scalar(
                    mask_source
                        .opacity
                        .as_ref()
                        .unwrap_or(&FLOAT_VALUE_ONE_HUNDRED),
                );
                target.masks.push(runtime::model::Mask {
                    mode,
                    geometry,
                    opacity,
                })
            }
        }
    }

    (
        source.index.as_ref().map_or(0, |ind| ind.unwrap_u32()) as usize,
        matte_mode,
    )
}
