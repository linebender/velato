// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
Representations of fixed (non-animated) values.
*/

use vello::kurbo::{self, Affine, Point, Vec2};
use vello::peniko;

/// Fixed affine transformation.
pub type Transform = kurbo::Affine;

/// Fixed RGBA color.
pub type Color = peniko::Color;

/// Fixed color stops.
pub type ColorStops = peniko::ColorStops;

/// Fixed brush.
pub type Brush = peniko::Brush;

/// Fixed stroke style.
pub type Stroke = kurbo::Stroke;

/// Fixed repeater effect.
#[derive(Clone, Debug)]
pub struct Repeater {
    /// Number of times to repeat.
    pub copies: u32,
    /// Offset of each subsequent repeated element.
    pub offset: f64,
    /// Anchor point.
    pub anchor_point: Point,
    /// Translation.
    pub position: Point,
    /// Rotation in degrees.
    pub rotation: f64,
    /// Scale.
    pub scale: Vec2,
    /// Opacity of the first element.
    pub start_opacity: f64,
    /// Opacity of the last element.
    pub end_opacity: f64,
}

impl Repeater {
    /// Returns the transform for the given copy index.
    pub fn transform(&self, index: u32) -> Affine {
        let t = self.offset + index as f64;
        Affine::translate((
            t * self.position.x + self.anchor_point.x,
            t * self.position.y + self.anchor_point.y,
        )) * Affine::rotate((t * self.rotation).to_radians())
            * Affine::scale_non_uniform(
                (self.scale.x / 100.0).powf(t),
                (self.scale.y / 100.0).powf(t),
            )
            * Affine::translate((-self.anchor_point.x, -self.anchor_point.y))
    }
}

// TODO: probably move this to peniko. The better option is to add an alpha
// parameter to the draw methods in vello. This is already handled at the
// encoding level.
pub(crate) fn brush_with_alpha(brush: &Brush, alpha: f64) -> Brush {
    if alpha == 1.0 {
        brush.clone()
    } else {
        match brush {
            Brush::Solid(color) => color.with_alpha_factor(alpha as f32).into(),
            Brush::Gradient(gradient) => Brush::Gradient(peniko::Gradient {
                kind: gradient.kind,
                extend: gradient.extend,
                stops: gradient
                    .stops
                    .iter()
                    .map(|stop| stop.with_alpha_factor(alpha as f32))
                    .collect(),
            }),
            _ => unreachable!(),
        }
    }
}
