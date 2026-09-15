// Copyright 2026 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{Easing, Time, Tween};
use kurbo::{
    CubicBez, ParamCurve, ParamCurveArclen, ParamCurveDeriv, ParamCurveExtrema, Point, Vec2,
};

#[derive(Clone, Debug)]
pub struct SpatialKeyframe {
    pub time: Time,
    pub value: Point,
    /// Offset from this keyframe's position.
    pub out_tangent: Option<Vec2>,
    /// Offset from the next position, stored on this keyframe.
    pub in_tangent: Option<Vec2>,
}

#[derive(Clone, Debug)]
struct MotionPath {
    curve: CubicBez,
    arc_length: f64,
    accuracy: f64,
}

#[derive(Clone, Debug)]
pub struct SpatialPosition {
    keyframes: Vec<SpatialKeyframe>,
    paths: Vec<Option<MotionPath>>,
}

impl SpatialPosition {
    const ACCURACY: f64 = 0.01;
    const RELATIVE_ACCURACY: f64 = 1e-5;

    fn is_linear(curve: &CubicBez) -> bool {
        let chord = curve.p3 - curve.p0;
        let out = curve.p1 - curve.p0;
        let incoming = curve.p3 - curve.p2;
        if chord == Vec2::ZERO {
            return out == Vec2::ZERO && incoming == Vec2::ZERO;
        }
        if !curve.is_finite()
            || !chord.is_finite()
            || chord.cross(out) != 0.0
            || chord.cross(incoming) != 0.0
        {
            return false;
        }
        let derivative = curve.deriv();
        if !derivative.is_finite() {
            return false;
        }
        let bounds = derivative.bounding_box();
        let (direction, min, max) = if chord.x.abs() >= chord.y.abs() {
            (chord.x, bounds.x0, bounds.x1)
        } else {
            (chord.y, bounds.y0, bounds.y1)
        };
        if direction > 0.0 {
            min >= 0.0
        } else {
            max <= 0.0
        }
    }

    pub fn new(keyframes: Vec<SpatialKeyframe>) -> Self {
        let paths = keyframes
            .windows(2)
            .map(|pair| {
                let start = &pair[0];
                let end = &pair[1];
                let out_tangent = start.out_tangent.unwrap_or(Vec2::ZERO);
                let in_tangent = start.in_tangent.unwrap_or(Vec2::ZERO);
                if start.time.hold {
                    return None;
                }
                let curve = CubicBez::new(
                    start.value,
                    start.value + out_tangent,
                    end.value + in_tangent,
                    end.value,
                );
                if Self::is_linear(&curve) {
                    return None;
                }
                let control_length = (curve.p1 - curve.p0).hypot()
                    + (curve.p2 - curve.p1).hypot()
                    + (curve.p3 - curve.p2).hypot();
                let accuracy = Self::ACCURACY
                    .min(control_length * Self::RELATIVE_ACCURACY)
                    .max(f64::MIN_POSITIVE);
                let arc_length = if curve.is_finite() {
                    curve.arclen(accuracy)
                } else {
                    f64::NAN
                };
                Some(MotionPath {
                    curve,
                    arc_length,
                    accuracy,
                })
            })
            .collect();
        Self { keyframes, paths }
    }

    pub fn evaluate(&self, frame: f64) -> Point {
        let index = self
            .keyframes
            .partition_point(|k| k.time.frame <= frame)
            .saturating_sub(1);
        let Some(start) = self.keyframes.get(index) else {
            return Point::ORIGIN;
        };
        let Some(end) = self.keyframes.get(index + 1) else {
            return start.value;
        };
        if frame <= start.time.frame || start.time.hold || end.time.frame <= start.time.frame {
            return start.value;
        }
        let t = ((frame - start.time.frame) / (end.time.frame - start.time.frame)).clamp(0.0, 1.0);
        let easing = Easing {
            o: start.time.out_tangent.unwrap_or(Easing::LERP.o),
            i: start.time.in_tangent.unwrap_or(Easing::LERP.i),
        };
        let progress = 0.0_f64.tween(&1.0, t, &easing);
        let Some(path) = &self.paths[index] else {
            return start.value.lerp(end.value, progress);
        };
        if !path.arc_length.is_finite() || !progress.is_finite() {
            return Point::new(f64::NAN, f64::NAN);
        }
        if path.arc_length == 0.0 || progress <= 0.0 {
            return start.value;
        }
        if progress >= 1.0 {
            return end.value;
        }
        let parameter = path
            .curve
            .inv_arclen(progress * path.arc_length, path.accuracy);
        path.curve.eval(parameter)
    }
}
