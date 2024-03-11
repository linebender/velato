// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use vello::kurbo::{self};
use vello::peniko;

/// Fixed or animated value.
#[derive(Clone, Debug)]
pub enum Value<T: Tweenable> {
    /// Fixed value.
    Fixed(T),
    /// Animated value.
    Animated(Animated<T>),
}

impl<T: Tweenable> Value<T> {
    /// Returns true if the value is fixed.
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    /// Returns the value at a specified frame.
    pub fn evaluate(&self, frame: f32) -> T {
        match self {
            Self::Fixed(fixed) => fixed.clone(),
            Self::Animated(animated) => animated.evaluate(frame),
        }
    }
}

impl<T: Tweenable + Default> Default for Value<T> {
    fn default() -> Self {
        Self::Fixed(T::default())
    }
}

/// Borrowed or owned value.
#[derive(Clone, Debug)]
pub enum ValueRef<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> AsRef<T> for ValueRef<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Borrowed(value) => value,
            Self::Owned(value) => value,
        }
    }
}

impl<'a, T: Clone> ValueRef<'a, T> {
    pub fn to_owned(self) -> T {
        match self {
            Self::Borrowed(value) => value.clone(),
            Self::Owned(value) => value,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Easing {
    pub o: EasingHandle,
    pub i: EasingHandle,
}

impl Easing {
    pub const LERP: Easing = Easing {
        o: EasingHandle { x: 0.0, y: 0.0 },
        i: EasingHandle { x: 1.0, y: 1.0 },
    };
}

#[derive(Copy, Clone, Debug)]
pub struct EasingHandle {
    pub x: f64,
    pub y: f64,
}

/// Time for a particular keyframe, represented as a frame number.
#[derive(Copy, Clone, Debug)]
pub struct Time {
    /// Frame number.
    pub frame: f32,
    /// Easing tangent going into the next keyframe
    pub in_tangent: Option<EasingHandle>,
    /// Easing tangent leaving the current keyframe
    pub out_tangent: Option<EasingHandle>,
    /// Whether it's a hold frame.
    pub hold: bool,
}

impl Time {
    /// Returns the frame indices and interpolation weight for the given frame,
    /// and whether to hold the frame
    pub(crate) fn frames_and_weight(
        times: &[Time],
        frame: f32,
    ) -> Option<([usize; 2], f32, Easing, bool)> {
        if times.is_empty() {
            return None;
        }
        use core::cmp::Ordering::*;
        let ix = match times.binary_search_by(|x| {
            if x.frame < frame {
                Less
            } else if x.frame > frame {
                Greater
            } else {
                Equal
            }
        }) {
            Ok(ix) => ix,
            Err(ix) => ix.saturating_sub(1),
        };
        let ix0 = ix.min(times.len() - 1);
        let ix1 = (ix0 + 1).min(times.len() - 1);

        let t0 = times[ix0];
        let t1 = times[ix1];
        let (t0_ox, t0_oy) = t0.out_tangent.map(|o| (o.x, o.y)).unwrap_or((0.0, 0.0));
        let (t1_ix, t1_iy) = t1.in_tangent.map(|i| (i.x, i.y)).unwrap_or((1.0, 1.0));
        let easing = Easing {
            o: EasingHandle { x: t0_ox, y: t0_oy },
            i: EasingHandle { x: t1_ix, y: t1_iy },
        };
        let hold = t0.hold;
        let t = (frame - t0.frame) / (t1.frame - t0.frame);
        Some(([ix0, ix1], t.clamp(0.0, 1.0), easing, hold))
    }
}

#[derive(Clone, Debug)]
pub struct Animated<T: Tweenable> {
    pub times: Vec<Time>,
    pub values: Vec<T>,
}

impl<T: Tweenable> Animated<T> {
    /// Returns the value at the specified frame.
    pub fn evaluate(&self, frame: f32) -> T {
        self.evaluate_inner(frame).unwrap_or_default()
    }

    fn evaluate_inner(&self, frame: f32) -> Option<T> {
        let ([ix0, ix1], t, easing, hold) = Time::frames_and_weight(&self.times, frame)?;
        let t = if hold { 0f32 } else { t };

        let v1 = self.values.get(ix0)?;
        let v2 = self.values.get(ix1)?;

        Some(v1.ease(v2, t, &easing))
    }
}

/// Something that can be interpolated with an easing function.
pub trait Tweenable: Clone + Default {
    fn ease(&self, other: &Self, t: f32, easing: &Easing) -> Self;
}

impl Tweenable for f64 {
    fn ease(&self, other: &Self, t: f32, _easing: &Easing) -> Self {
        // TODO: We are enforcing linear interpolation for now, but a decent amount of work is done for easings.
        keyframe::ease(keyframe::functions::Linear, *self, *other, t)

        // FIXME: Hopefully we can finish this up one day!
        //keyframe::ease(
        //    keyframe::functions::BezierCurve::from(
        //        keyframe::mint::Vector2::from_slice(&[easing.o.x, easing.o.y]),
        //        keyframe::mint::Vector2::from_slice(&[easing.i.x, easing.i.y]),
        //    ),
        //    *self,
        //    *other,
        //    t,
        //)
    }
}

impl Tweenable for f32 {
    fn ease(&self, other: &Self, t: f32, easing: &Easing) -> Self {
        (*self as f64).ease(&(*other as f64), t, easing) as f32
    }
}

impl Tweenable for kurbo::Point {
    fn ease(&self, other: &Self, t: f32, easing: &Easing) -> Self {
        Self::new(
            self.x.ease(&other.x, t, easing),
            self.y.ease(&other.y, t, easing),
        )
    }
}

impl Tweenable for kurbo::Vec2 {
    fn ease(&self, other: &Self, t: f32, easing: &Easing) -> Self {
        Self::new(
            self.x.ease(&other.x, t, easing),
            self.y.ease(&other.y, t, easing),
        )
    }
}

impl Tweenable for kurbo::Size {
    fn ease(&self, other: &Self, t: f32, easing: &Easing) -> Self {
        Self::new(
            self.width.ease(&other.width, t, easing),
            self.height.ease(&other.height, t, easing),
        )
    }
}

impl Tweenable for peniko::Color {
    fn ease(&self, other: &Self, t: f32, easing: &Easing) -> Self {
        let r = (self.r as f64 / 255.0).ease(&(other.r as f64 / 255.0), t, easing);
        let g = (self.g as f64 / 255.0).ease(&(other.g as f64 / 255.0), t, easing);
        let b = (self.b as f64 / 255.0).ease(&(other.b as f64 / 255.0), t, easing);
        let a = (self.a as f64 / 255.0).ease(&(other.a as f64 / 255.0), t, easing);
        peniko::Color::rgba(r, g, b, a)
    }
}
