// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Composition, RenderSink, Renderer, model::ImageAsset, model::fixed};

use kurbo::{Affine, Shape};
use peniko::{BlendMode, Fill, ImageBrushRef, ImageData};
use std::collections::HashMap;

impl RenderSink for vello::Scene {
    fn push_layer(
        &mut self,
        blend: impl Into<BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl Shape,
    ) {
        self.push_layer(Fill::NonZero, blend, alpha, transform, shape);
    }

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl Shape) {
        self.push_clip_layer(Fill::NonZero, transform, shape);
    }

    fn pop_layer(&mut self) {
        self.pop_layer();
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl Shape,
    ) {
        if let Some(stroke) = stroke {
            self.stroke(stroke, transform, brush, None, shape);
        } else {
            self.fill(Fill::NonZero, transform, brush, None, shape);
        }
    }

    fn draw_image(&mut self, _: &ImageAsset, _: Affine, _: f64) {
        panic!(
            "vello::Scene cannot resolve image assets; pass an image map to Renderer::render_to_vello_scene, or provide a custom RenderSink"
        );
    }
}

struct VelloSceneSink<'a> {
    scene: &'a mut vello::Scene,
    /// Lottie resolve images with refId, a string schema type
    images: &'a HashMap<String, ImageData>,
}

impl RenderSink for VelloSceneSink<'_> {
    fn push_layer(
        &mut self,
        blend: impl Into<BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl Shape,
    ) {
        self.scene
            .push_layer(Fill::NonZero, blend, alpha, transform, shape);
    }

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl Shape) {
        self.scene.push_clip_layer(Fill::NonZero, transform, shape);
    }

    fn pop_layer(&mut self) {
        self.scene.pop_layer();
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl Shape,
    ) {
        if let Some(stroke) = stroke {
            self.scene.stroke(stroke, transform, brush, None, shape);
        } else {
            self.scene
                .fill(Fill::NonZero, transform, brush, None, shape);
        }
    }

    fn draw_image(&mut self, asset: &ImageAsset, mut transform: Affine, alpha: f64) {
        let Some(image) = self.images.get(&asset.id) else {
            return;
        };
        if let Some((width, height)) = asset
            .width
            .zip(asset.height)
            .filter(|(width, height)| *width > 0.0 && *height > 0.0)
        {
            let image_width = f64::from(image.width);
            let image_height = f64::from(image.height);
            if image_width > 0.0 && image_height > 0.0 {
                let scale = (width / image_width).min(height / image_height);
                let offset = (
                    (width - image_width * scale) * 0.5,
                    (height - image_height * scale) * 0.5,
                );
                transform *= Affine::translate(offset) * Affine::scale(scale);
            }
        }
        let alpha = if alpha.is_finite() {
            alpha.max(0.0) as f32
        } else {
            0.0
        };
        self.scene
            .draw_image(ImageBrushRef::from(image).with_alpha(alpha), transform);
    }
}

impl Renderer {
    /// Renders the animation at a given frame to a new scene.
    ///
    /// `images` contains caller-owned, decoded image data keyed by Lottie asset
    /// identifier. Loading and decoding remain the caller's responsibility.
    pub fn render_to_vello_scene(
        &mut self,
        animation: &Composition,
        images: &HashMap<String, ImageData>,
        frame: f64,
        transform: Affine,
        alpha: f64,
    ) -> vello::Scene {
        let mut scene = vello::Scene::new();
        let mut sink = VelloSceneSink {
            scene: &mut scene,
            images,
        };
        self.append(animation, frame, transform, alpha, &mut sink);
        scene
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Content, Layer, Value};
    use peniko::{Blob, ImageAlphaType, ImageFormat};
    use std::sync::Arc;

    #[test]
    fn registered_images_are_encoded_in_vello_scenes() {
        let animation = Composition {
            width: 10,
            height: 10,
            images: [(
                "image".into(),
                ImageAsset {
                    id: "image".into(),
                    width: Some(1.0),
                    height: Some(1.0),
                    directory: None,
                    file_name: "image.png".into(),
                    embedded: false,
                },
            )]
            .into(),
            layers: vec![Layer {
                frames: 0.0..1.0,
                opacity: Value::Fixed(100.0),
                stretch: 1.0,
                content: Content::Image {
                    asset_id: "image".into(),
                },
                ..Layer::default()
            }],
            ..Composition::default()
        };
        let images = [(
            "image".into(),
            ImageData {
                data: Blob::new(Arc::new(vec![255, 0, 0, 255])),
                format: ImageFormat::Rgba8,
                alpha_type: ImageAlphaType::Alpha,
                width: 1,
                height: 1,
            },
        )]
        .into();
        let scene =
            Renderer::new().render_to_vello_scene(&animation, &images, 0.0, Affine::IDENTITY, 1.0);
        assert_eq!(scene.encoding().resources.patches.len(), 1);
    }
}
