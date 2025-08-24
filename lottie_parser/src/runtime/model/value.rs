// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Fixed or animated value.
#[derive(Clone, Debug)]
pub enum Value<T: Tween> {
    /// Fixed value.
    Fixed(T),
    /// Animated value.
    Animated(Animated<T>),
}

impl<T: Tween + Default> Value<T> {
    /// Returns the value at a specified frame.
    pub fn evaluate(&self, frame: f64) -> T {
        match self {
            Self::Fixed(fixed) => fixed.clone(),
            Self::Animated(animated) => animated.evaluate_or(frame, T::default()),
        }
    }
}

impl<T: Tween> Value<T> {
    /// Returns true if the value is fixed.
    pub fn is_fixed(&self) -> bool {
        matches!(self, Self::Fixed(_))
    }

    /// Returns the value at a specified frame, with a fallback default.
    pub fn evaluate_or(&self, frame: f64, default: T) -> T {
        match self {
            Self::Fixed(fixed) => fixed.clone(),
            Self::Animated(animated) => animated.evaluate_or(frame, default),
        }
    }
}

impl<T: Tween + Default> Default for Value<T> {
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

impl<T> AsRef<T> for ValueRef<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            Self::Borrowed(value) => value,
            Self::Owned(value) => value,
        }
    }
}

impl<T: Clone> ValueRef<'_, T> {
    pub fn into_owned(self) -> T {
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
    pub frame: f64,
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
        frame: f64,
    ) -> Option<([usize; 2], f64, Easing, bool)> {
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
        let (t0_ix, t0_iy) = t0.in_tangent.map(|i| (i.x, i.y)).unwrap_or((1.0, 1.0));
        let easing = Easing {
            o: EasingHandle { x: t0_ox, y: t0_oy },
            i: EasingHandle { x: t0_ix, y: t0_iy },
        };
        let hold = t0.hold;
        let t = (frame - t0.frame) / (t1.frame - t0.frame);
        Some(([ix0, ix1], t.clamp(0.0, 1.0), easing, hold))
    }
}

#[derive(Clone, Debug)]
pub struct Animated<T: Tween> {
    pub times: Vec<Time>,
    pub values: Vec<T>,
}

impl<T: Tween> Animated<T> {
    /// Returns the value at the specified frame.
    pub fn evaluate_or(&self, frame: f64, default: T) -> T {
        self.evaluate_inner(frame).unwrap_or(default)
    }

    fn evaluate_inner(&self, frame: f64) -> Option<T> {
        let ([ix0, ix1], t, easing, hold) = Time::frames_and_weight(&self.times, frame)?;
        let t = if hold { 0.0 } else { t };

        let v1 = self.values.get(ix0)?;
        let v2 = self.values.get(ix1)?;

        Some(v1.tween(v2, t, &easing))
    }
}

/// Something that can be interpolated with an easing function.
pub trait Tween: Clone {
    fn tween(&self, other: &Self, t: f64, easing: &Easing) -> Self;
}

impl Tween for f64 {
    fn tween(&self, other: &Self, t: f64, easing: &Easing) -> Self {
        keyframe::ease(
            keyframe::functions::BezierCurve::from(
                keyframe::mint::Vector2::from_slice(&[easing.o.x, easing.o.y]),
                keyframe::mint::Vector2::from_slice(&[easing.i.x, easing.i.y]),
            ),
            *self,
            *other,
            t,
        )
    }
}

impl Tween for kurbo::Point {
    fn tween(&self, other: &Self, t: f64, easing: &Easing) -> Self {
        Self::new(
            self.x.tween(&other.x, t, easing),
            self.y.tween(&other.y, t, easing),
        )
    }
}

impl Tween for kurbo::Vec2 {
    fn tween(&self, other: &Self, t: f64, easing: &Easing) -> Self {
        Self::new(
            self.x.tween(&other.x, t, easing),
            self.y.tween(&other.y, t, easing),
        )
    }
}

impl Tween for kurbo::Size {
    fn tween(&self, other: &Self, t: f64, easing: &Easing) -> Self {
        Self::new(
            self.width.tween(&other.width, t, easing),
            self.height.tween(&other.height, t, easing),
        )
    }
}

impl Tween for peniko::Color {
    fn tween(&self, other: &Self, t: f64, easing: &Easing) -> Self {
        let [r1, g1, b1, a1] = self.components;
        let [r2, g2, b2, a2] = other.components;
        let r = (r1 as f64).tween(&(r2 as f64), t, easing);
        let g = (g1 as f64).tween(&(g2 as f64), t, easing);
        let b = (b1 as f64).tween(&(b2 as f64), t, easing);
        let a = (a1 as f64).tween(&(a2 as f64), t, easing);
        peniko::Color::new([r as f32, g as f32, b as f32, a as f32])
    }
}
