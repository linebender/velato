// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/*!
Representations of animated values.
*/

use super::*;

use kurbo::PathEl;

#[derive(Clone, Debug)]
pub enum Position {
    Value(Value<Point>),
    SplitValues((Value<f64>, Value<f64>)),
}

/// Animated affine transformation.
#[derive(Clone, Debug)]
pub struct Transform {
    /// Anchor point.
    pub anchor: Value<Point>,
    /// Translation.
    pub position: Position,
    /// Rotation angle.
    pub rotation: Value<f64>,
    /// Scale factor.
    pub scale: Value<Vec2>,
    /// Skew factor.
    pub skew: Value<f64>,
    /// Skew angle, 0 skews along the x axis, 90 along the y axis.
    pub skew_angle: Value<f64>,
}

impl Transform {
    /// Returns true if the transform is fixed.
    pub fn is_fixed(&self) -> bool {
        self.anchor.is_fixed()
            && match &self.position {
                Position::Value(value) => value.is_fixed(),
                Position::SplitValues((x_value, y_value)) => {
                    x_value.is_fixed() && y_value.is_fixed()
                }
            }
            && self.rotation.is_fixed()
            && self.scale.is_fixed()
            && self.skew.is_fixed()
            && self.skew_angle.is_fixed()
    }

    /// Evaluates the transform at the specified frame.
    pub fn evaluate(&self, frame: f64) -> Affine {
        let anchor = self.anchor.evaluate(frame);
        let position = match &self.position {
            Position::Value(value) => value.evaluate(frame),
            Position::SplitValues((x_value, y_value)) => kurbo::Point {
                x: x_value.evaluate(frame),
                y: y_value.evaluate(frame),
            },
        };
        let rotation = self.rotation.evaluate(frame);
        let scale = self.scale.evaluate(frame);
        let skew = self.skew.evaluate(frame);
        let skew_angle = self.skew_angle.evaluate(frame);
        let skew_matrix = if skew != 0.0 {
            let skew = skew.to_radians();
            let angle = skew_angle.to_radians();
            Affine::rotate(-angle) * Affine::skew(skew.tan(), 0.0) * Affine::rotate(angle)
        } else {
            Affine::IDENTITY
        };
        Affine::translate((position.x, position.y))
            * Affine::rotate(rotation.to_radians())
            * skew_matrix
            * Affine::scale_non_uniform(scale.x / 100.0, scale.y / 100.0)
            * Affine::translate((-anchor.x, -anchor.y))
    }

    /// Converts the animated value to its model representation.
    pub fn into_model(self) -> super::Transform {
        if self.is_fixed() {
            super::Transform::Fixed(self.evaluate(0.0))
        } else {
            super::Transform::Animated(self)
        }
    }
}

/// Animated ellipse.
#[derive(Clone, Debug)]
pub struct Ellipse {
    /// True if the ellipse should be drawn in CCW order.
    pub is_ccw: bool,
    /// Position of the ellipse.
    pub position: Value<Point>,
    /// Size of the ellipse.
    pub size: Value<Size>,
}

impl Ellipse {
    pub fn is_fixed(&self) -> bool {
        self.position.is_fixed() && self.size.is_fixed()
    }

    pub fn evaluate(&self, frame: f64) -> kurbo::Ellipse {
        let position = self.position.evaluate(frame);
        let size = self.size.evaluate(frame);
        let radii = (size.width * 0.5, size.height * 0.5);
        kurbo::Ellipse::new(position, radii, 0.0)
    }
}

/// Animated rounded rectangle.
#[derive(Clone, Debug)]
pub struct Rect {
    /// True if the rect should be drawn in CCW order.
    pub is_ccw: bool,
    /// Position of the rectangle.
    pub position: Value<Point>,
    /// Size of the rectangle.
    pub size: Value<Size>,
    /// Radius of the rectangle corners.
    pub corner_radius: Value<f64>,
}

impl Rect {
    /// Returns true if the rectangle is fixed.
    pub fn is_fixed(&self) -> bool {
        self.position.is_fixed() && self.size.is_fixed() && self.corner_radius.is_fixed()
    }

    /// Evaluates the rectangle at the specified frame.
    pub fn evaluate(&self, frame: f64) -> kurbo::RoundedRect {
        let position = self.position.evaluate(frame);
        let size = self.size.evaluate(frame);
        let position = Point::new(
            position.x - size.width * 0.5,
            position.y - size.height * 0.5,
        );
        let radius = self.corner_radius.evaluate(frame);
        kurbo::RoundedRect::new(
            position.x,
            position.y,
            position.x + size.width,
            position.y + size.height,
            radius,
        )
    }
}

/// Animated star or polygon.
#[derive(Clone, Debug)]
pub struct Star {
    pub is_polygon: bool,
    pub direction: f64,
    pub position: Value<Point>,
    pub inner_radius: Value<f64>,
    pub inner_roundness: Value<f64>,
    pub outer_radius: Value<f64>,
    pub outer_roundness: Value<f64>,
    pub rotation: Value<f64>,
    pub points: Value<f64>,
}

impl Star {
    pub fn is_fixed(&self) -> bool {
        self.position.is_fixed()
            && self.inner_radius.is_fixed()
            && self.inner_roundness.is_fixed()
            && self.outer_radius.is_fixed()
            && self.outer_roundness.is_fixed()
            && self.rotation.is_fixed()
            && self.points.is_fixed()
    }

    /// Evaluates the star or polygon at the given frame and emits the
    /// elements to the specified path.
    pub fn evaluate(&self, frame: f64, path: &mut Vec<PathEl>) {
        use std::f64::consts::{FRAC_PI_2, TAU};

        let center = self.position.evaluate(frame);
        let num_points = self.points.evaluate(frame).round() as usize;
        if num_points == 0 {
            return;
        }
        let rotation_deg = self.rotation.evaluate(frame);
        let outer_radius = self.outer_radius.evaluate(frame);
        let outer_roundness = self.outer_roundness.evaluate(frame) / 100.0;

        let is_star = !self.is_polygon;
        let (inner_radius, inner_roundness) = if is_star {
            (
                self.inner_radius.evaluate(frame),
                self.inner_roundness.evaluate(frame) / 100.0,
            )
        } else {
            (outer_radius, 0.0)
        };

        let angle_per_point = TAU / num_points as f64;
        let half_angle = angle_per_point / 2.0;
        let rotation_rad = rotation_deg.to_radians() - FRAC_PI_2;

        let total_vertices = if is_star { num_points * 2 } else { num_points };
        let vertex_angle_step = if is_star { half_angle } else { angle_per_point };

        let mut vertices: Vec<(Point, Point, Point)> = Vec::with_capacity(total_vertices);
        let mut current_angle = rotation_rad;

        for i in 0..total_vertices {
            let (radius, roundness) = if !is_star || i % 2 == 0 {
                (outer_radius, outer_roundness)
            } else {
                (inner_radius, inner_roundness)
            };

            let x = center.x + radius * current_angle.cos();
            let y = center.y + radius * current_angle.sin();

            let (in_pt, out_pt) = if roundness.abs() > 1e-6 {
                let tangent_len = radius * (vertex_angle_step / 2.0).tan() * roundness * 4.0 / 3.0;
                let perp_x = -current_angle.sin();
                let perp_y = current_angle.cos();
                (
                    Point::new(x - tangent_len * perp_x, y - tangent_len * perp_y),
                    Point::new(x + tangent_len * perp_x, y + tangent_len * perp_y),
                )
            } else {
                (Point::new(x, y), Point::new(x, y))
            };

            vertices.push((Point::new(x, y), in_pt, out_pt));
            current_angle += vertex_angle_step;
        }

        if vertices.is_empty() {
            return;
        }

        path.reserve(total_vertices * 3 + 2);
        path.push(PathEl::MoveTo(vertices[0].0));
        for i in 1..vertices.len() {
            let prev_out = vertices[i - 1].2;
            let curr_in = vertices[i].1;
            let curr_pt = vertices[i].0;
            let prev_pt = vertices[i - 1].0;
            if (prev_out.x - prev_pt.x).abs() < 1e-6
                && (prev_out.y - prev_pt.y).abs() < 1e-6
                && (curr_in.x - curr_pt.x).abs() < 1e-6
                && (curr_in.y - curr_pt.y).abs() < 1e-6
            {
                path.push(PathEl::LineTo(curr_pt));
            } else {
                path.push(PathEl::CurveTo(prev_out, curr_in, curr_pt));
            }
        }
        let last = vertices.last().unwrap();
        let first = &vertices[0];
        if (last.2.x - last.0.x).abs() < 1e-6
            && (last.2.y - last.0.y).abs() < 1e-6
            && (first.1.x - first.0.x).abs() < 1e-6
            && (first.1.y - first.0.y).abs() < 1e-6
        {
            path.push(PathEl::ClosePath);
        } else {
            path.push(PathEl::CurveTo(last.2, first.1, first.0));
            path.push(PathEl::ClosePath);
        }
    }

    /// Converts the animated value to its model representation.
    pub fn into_model(self) -> super::Geometry {
        if self.is_fixed() {
            let mut path = Vec::new();
            self.evaluate(0.0, &mut path);
            super::Geometry::Fixed(path)
        } else {
            super::Geometry::Star(self)
        }
    }
}

/// Animated cubic spline.
#[derive(Clone, Debug)]
pub struct Spline {
    /// True if the spline is closed.
    pub is_closed: bool,
    /// Collection of times.
    pub times: Vec<Time>,
    /// Collection of splines for each time. The splines at each point must have a start, but may also have an
    /// end defined. When the end is not defined, then the next keyframe's start is used.
    pub values: Vec<SplineKeyframeValues>,
}

/// The values of an animated Spline at a single keyframe. When end is None, the next frame's start is used.
#[derive(Clone, Debug)]
pub struct SplineKeyframeValues {
    pub start: Vec<Point>,
    pub end: Option<Vec<Point>>,
}

impl Spline {
    /// Evaluates the spline at the given frame and emits the elements
    /// to the specified path.
    pub fn evaluate(&self, frame: f64, path: &mut Vec<PathEl>) -> bool {
        let Some(([ix0, ix1], t, _easing, hold)) = Time::frames_and_weight(&self.times, frame)
        else {
            return false;
        };
        let t = if hold { 0.0 } else { t };
        let (Some(from), Some(to)) = (self.values.get(ix0), self.values.get(ix1)) else {
            return false;
        };
        let to_slice = {
            if let Some(end) = &from.end {
                end.as_slice()
            } else {
                to.start.as_slice()
            }
        };
        (from.start.as_slice(), to_slice, t).to_path(self.is_closed, path);
        true
    }
}

/// Animated repeater effect.
#[derive(Clone, Debug)]
pub struct Repeater {
    /// Number of times elements should be repeated.
    pub copies: Value<f64>,
    /// Offset applied to each element.
    pub offset: Value<f64>,
    /// Anchor point.
    pub anchor_point: Value<Point>,
    /// Translation.
    pub position: Value<Point>,
    /// Rotation in degrees.
    pub rotation: Value<f64>,
    /// Scale.
    pub scale: Value<Vec2>,
    /// Opacity of the first element.
    pub start_opacity: Value<f64>,
    /// Opacity of the last element.
    pub end_opacity: Value<f64>,
}

impl Repeater {
    /// Returns true if the repeater contains no animated properties.
    pub fn is_fixed(&self) -> bool {
        self.copies.is_fixed()
            && self.offset.is_fixed()
            && self.anchor_point.is_fixed()
            && self.position.is_fixed()
            && self.rotation.is_fixed()
            && self.scale.is_fixed()
            && self.start_opacity.is_fixed()
            && self.end_opacity.is_fixed()
    }

    /// Evaluates the repeater at the specified frame.
    pub fn evaluate(&self, frame: f64) -> fixed::Repeater {
        let copies = self.copies.evaluate(frame).round() as usize;
        let offset = self.offset.evaluate(frame);
        let anchor_point = self.anchor_point.evaluate(frame);
        let position = self.position.evaluate(frame);
        let rotation = self.rotation.evaluate(frame);
        let scale = self.scale.evaluate(frame);
        let start_opacity = self.start_opacity.evaluate(frame);
        let end_opacity = self.end_opacity.evaluate(frame);
        fixed::Repeater {
            copies,
            offset,
            anchor_point,
            position,
            rotation,
            scale,
            start_opacity,
            end_opacity,
        }
    }

    /// Converts the animated value to its model representation.
    pub fn into_model(self) -> super::Repeater {
        if self.is_fixed() {
            super::Repeater::Fixed(self.evaluate(0.0))
        } else {
            super::Repeater::Animated(self)
        }
    }
}

/// Animated stroke properties.
#[derive(Clone, Debug)]
pub struct Stroke {
    /// Width of the stroke.
    pub width: Value<f64>,
    /// Join style.
    pub join: kurbo::Join,
    /// Limit for miter joins.
    pub miter_limit: Option<f64>,
    /// Cap style.
    pub cap: kurbo::Cap,
}

impl Stroke {
    /// Returns true if the stroke is fixed.
    pub fn is_fixed(&self) -> bool {
        self.width.is_fixed()
    }

    /// Evaluates the stroke at the specified frame.
    pub fn evaluate(&self, frame: f64) -> kurbo::Stroke {
        let width = self.width.evaluate(frame);
        let mut stroke = kurbo::Stroke::new(width)
            .with_caps(self.cap)
            .with_join(self.join);
        if let Some(miter_limit) = self.miter_limit {
            stroke.miter_limit = miter_limit;
        }
        stroke
    }

    /// Converts the animated value to its model representation.
    pub fn into_model(self) -> super::Stroke {
        if self.is_fixed() {
            super::Stroke::Fixed(self.evaluate(0.0))
        } else {
            super::Stroke::Animated(self)
        }
    }
}

/// Animated linear or radial gradient.
#[derive(Clone, Debug)]
pub struct Gradient {
    /// True if the gradient is radial.
    pub is_radial: bool,
    /// Starting point.
    pub start_point: Value<Point>,
    /// Ending point.
    pub end_point: Value<Point>,
    /// Stop offsets and color values.
    pub stops: super::ColorStops,
}

impl Gradient {
    /// Returns true if the value contains no animated properties.
    pub fn is_fixed(&self) -> bool {
        self.start_point.is_fixed() && self.end_point.is_fixed() && self.stops.is_fixed()
    }

    /// Evaluates the animated value at the given frame.
    pub fn evaluate(&self, frame: f64) -> peniko::Brush {
        let start = self.start_point.evaluate(frame);
        let end = self.end_point.evaluate(frame);
        let stops = self.stops.evaluate(frame).into_owned();
        if self.is_radial {
            let radius = (end.to_vec2() - start.to_vec2()).hypot();
            let mut grad = peniko::Gradient::new_radial(start, radius as f32);
            grad.stops = stops;
            grad.into()
        } else {
            let mut grad = peniko::Gradient::new_linear(start, end);
            grad.stops = stops;
            grad.into()
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorStops {
    pub frames: Vec<Time>,
    pub values: Vec<Vec<f64>>,
    pub count: usize,
}

impl ColorStops {
    pub fn evaluate(&self, frame: f64) -> fixed::ColorStops {
        self.evaluate_inner(frame).unwrap_or_default()
    }

    fn evaluate_inner(&self, frame: f64) -> Option<fixed::ColorStops> {
        let ([ix0, ix1], t, easing, hold) = Time::frames_and_weight(&self.frames, frame)?;

        let v0 = self.values.get(ix0)?;
        let v1 = self.values.get(ix1)?;

        let mut stops: fixed::ColorStops = Default::default();
        for i in 0..self.count {
            let j = i * 5;
            let offset = v0.get(j)?.tween(v1.get(j)?, t, &easing);
            let t = if hold { 0.0 } else { t };
            let r = v0.get(j + 1)?.tween(v1.get(j + 1)?, t, &easing);
            let g = v0.get(j + 2)?.tween(v1.get(j + 2)?, t, &easing);
            let b = v0.get(j + 3)?.tween(v1.get(j + 3)?, t, &easing);
            let a = v0.get(j + 4)?.tween(v1.get(j + 4)?, t, &easing);
            let stop = peniko::ColorStop::from((
                offset as f32,
                peniko::Color::new([r as f32, g as f32, b as f32, a as f32]),
            ));
            stops.push(stop);
        }
        Some(stops)
    }
}

/// Animated brush.
#[derive(Clone, Debug)]
pub enum Brush {
    /// Solid color.
    Solid(Value<Color>),
    /// Gradient color.
    Gradient(Gradient),
}

impl Brush {
    /// Returns true if the value contains no animated properties.
    pub fn is_fixed(&self) -> bool {
        match self {
            Self::Solid(value) => value.is_fixed(),
            Self::Gradient(value) => value.is_fixed(),
        }
    }

    /// Evaluates the animation at the specified time.
    pub fn evaluate(&self, alpha: f64, frame: f64) -> fixed::Brush {
        match self {
            Self::Solid(value) => value
                .evaluate_or(frame, peniko::Color::TRANSPARENT)
                .multiply_alpha(alpha as f32)
                .into(),
            Self::Gradient(value) => value.evaluate(frame),
        }
    }

    /// Converts the animated value to its model representation.
    pub fn into_model(self) -> super::Brush {
        if self.is_fixed() {
            super::Brush::Fixed(self.evaluate(1.0, 0.0))
        } else {
            super::Brush::Animated(self)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trim {
    /// Start of the visible segment (0.0 to 100.0).
    pub start: Value<f64>,
    /// End of the visible segment (0.0 to 100.0).
    pub end: Value<f64>,
    /// Offset rotation in degrees.
    pub offset: Value<f64>,
}

impl Trim {
    /// Returns true if the trim is fixed.
    pub fn is_fixed(&self) -> bool {
        self.start.is_fixed() && self.end.is_fixed() && self.offset.is_fixed()
    }

    /// Evaluates the trim at the specified frame.
    pub fn evaluate(&self, frame: f64) -> fixed::Trim {
        fixed::Trim {
            start: self.start.evaluate(frame),
            end: self.end.evaluate(frame),
            offset: self.offset.evaluate(frame),
        }
    }

    /// Converts the animated value to its model representation.
    pub fn into_model(self) -> super::Trim {
        if self.is_fixed() {
            super::Trim::Fixed(self.evaluate(0.0))
        } else {
            super::Trim::Animated(self)
        }
    }
}
