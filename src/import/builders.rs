// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::converters::{conv_blend_mode, conv_scalar, conv_shape_geometry, conv_transform};
use super::defaults::FLOAT_VALUE_ONE_HUNDRED;
use crate::runtime::model::Layer;
use crate::schema::helpers::int_boolean::BoolInt;
use crate::{runtime, schema};
use peniko::{self, BlendMode, Compose, Mix};

pub struct LayerSetupParams {
    pub layer_index: usize,
    pub matte_mode: Option<BlendMode>,
    pub matte_layer_index: Option<usize>,
}

pub fn setup_precomp_layer(
    source: &schema::layers::precomposition::PrecompositionLayer,
    target: &mut Layer,
) -> LayerSetupParams {
    target.name = source
        .visual_layer
        .layer
        .visual_object
        .name
        .clone()
        .unwrap_or_default();
    target.parent = source.visual_layer.layer.parent_index;
    let (transform, opacity) = conv_transform(&source.visual_layer.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.width = source.width;
    target.height = source.height;
    target.is_mask = source
        .visual_layer
        .matte_target
        .as_ref()
        .is_some_and(|td| *td == BoolInt::True);

    let matte_mode = source
        .visual_layer
        .matte_mode
        .as_ref()
        .map(|mode| match mode {
            schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
            schema::constants::matte_mode::MatteMode::Alpha
            | schema::constants::matte_mode::MatteMode::Luma => Compose::SrcIn.into(),
            schema::constants::matte_mode::MatteMode::InvertedAlpha
            | schema::constants::matte_mode::MatteMode::InvertedLuma => Compose::SrcOut.into(),
        });

    let matte_layer_index = source
        .visual_layer
        .matte_layer_index
        .map(|idx| idx as usize);

    target.blend_mode = conv_blend_mode(
        source
            .visual_layer
            .blend_mode
            .as_ref()
            .unwrap_or(&crate::schema::constants::blend_mode::BlendMode::Normal),
    );
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.stretch = source.visual_layer.layer.time_stretch.unwrap_or(1.0);
    target.frames = source.visual_layer.layer.in_point..source.visual_layer.layer.out_point;
    target.start_frame = source.visual_layer.layer.start_time.unwrap_or(0.0);

    for mask_source in source
        .visual_layer
        .masks_properties
        .as_ref()
        .unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape
            && let Some(geometry) = conv_shape_geometry(shape)
        {
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
            });
        }
    }

    LayerSetupParams {
        layer_index: source.visual_layer.layer.index.unwrap_or(0),
        matte_mode,
        matte_layer_index,
    }
}

pub fn setup_shape_layer(
    source: &schema::layers::shape::ShapeLayer,
    target: &mut Layer,
) -> LayerSetupParams {
    target.name = source
        .visual_layer
        .layer
        .visual_object
        .name
        .clone()
        .unwrap_or_default();
    target.parent = source.visual_layer.layer.parent_index;
    let (transform, opacity) = conv_transform(&source.visual_layer.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.is_mask = source
        .visual_layer
        .matte_target
        .as_ref()
        .is_some_and(|td| *td == BoolInt::True);

    let matte_mode = source
        .visual_layer
        .matte_mode
        .as_ref()
        .map(|mode| match mode {
            schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
            schema::constants::matte_mode::MatteMode::Alpha
            | schema::constants::matte_mode::MatteMode::Luma => Compose::SrcIn.into(),
            schema::constants::matte_mode::MatteMode::InvertedAlpha
            | schema::constants::matte_mode::MatteMode::InvertedLuma => Compose::SrcOut.into(),
        });

    let matte_layer_index = source
        .visual_layer
        .matte_layer_index
        .map(|idx| idx as usize);

    target.blend_mode = conv_blend_mode(
        source
            .visual_layer
            .blend_mode
            .as_ref()
            .unwrap_or(&crate::schema::constants::blend_mode::BlendMode::Normal),
    );
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.stretch = source.visual_layer.layer.time_stretch.unwrap_or(1.0);
    target.frames = source.visual_layer.layer.in_point..source.visual_layer.layer.out_point;
    target.start_frame = source.visual_layer.layer.start_time.unwrap_or(0.0);

    for mask_source in source
        .visual_layer
        .masks_properties
        .as_ref()
        .unwrap_or(&Vec::default())
    {
        if let Some(shape) = &mask_source.shape
            && let Some(geometry) = conv_shape_geometry(shape)
        {
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
            });
        }
    }

    LayerSetupParams {
        layer_index: source.visual_layer.layer.index.unwrap_or(0),
        matte_mode,
        matte_layer_index,
    }
}

pub fn setup_layer_base(
    source: &schema::layers::visual::VisualLayer,
    target: &mut Layer,
) -> LayerSetupParams {
    target.name = source.layer.visual_object.name.clone().unwrap_or_default();
    target.parent = source.layer.parent_index;
    let (transform, opacity) = conv_transform(&source.transform);
    target.transform = transform;
    target.opacity = opacity;
    target.is_mask = source
        .matte_target
        .as_ref()
        .is_some_and(|td| *td == BoolInt::True);

    let matte_mode = source.matte_mode.as_ref().map(|mode| match mode {
        schema::constants::matte_mode::MatteMode::Normal => Mix::Normal.into(),
        schema::constants::matte_mode::MatteMode::Alpha
        | schema::constants::matte_mode::MatteMode::Luma => Compose::SrcIn.into(),
        schema::constants::matte_mode::MatteMode::InvertedAlpha
        | schema::constants::matte_mode::MatteMode::InvertedLuma => Compose::SrcOut.into(),
    });

    let matte_layer_index = source.matte_layer_index.map(|idx| idx as usize);

    target.blend_mode = conv_blend_mode(
        source
            .blend_mode
            .as_ref()
            .unwrap_or(&crate::schema::constants::blend_mode::BlendMode::Normal),
    );
    // TODO: Why do we do this next part?
    if target.blend_mode == Some(peniko::Mix::Normal.into()) {
        target.blend_mode = None;
    }
    target.stretch = source.layer.time_stretch.unwrap_or(1.0);
    target.frames = source.layer.in_point..source.layer.out_point;
    target.start_frame = source.layer.start_time.unwrap_or(0.0);

    for mask_source in source.masks_properties.as_ref().unwrap_or(&Vec::default()) {
        if let Some(shape) = &mask_source.shape
            && let Some(geometry) = conv_shape_geometry(shape)
        {
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
            });
        }
    }

    LayerSetupParams {
        layer_index: source.layer.index.unwrap_or(0),
        matte_mode,
        matte_layer_index,
    }
}
