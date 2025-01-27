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
    pub copies: usize,
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
    pub fn transform(&self, index: usize) -> Affine {
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
