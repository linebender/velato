// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::runtime::model::{Easing, Tweenable};
use serde_json::Number;

pub trait NumberExt {
    fn unwrap_f32(&self) -> f32;
    fn unwrap_f64(&self) -> f64;
    fn unwrap_u32(&self) -> u32;
}

impl NumberExt for serde_json::Number {
    fn unwrap_f32(&self) -> f32 {
        self.as_f64().expect("Could not get float from JSON Number") as f32
    }

    fn unwrap_f64(&self) -> f64 {
        self.as_f64().expect("Could not get float from JSON Number")
    }

    fn unwrap_u32(&self) -> u32 {
        self.as_u64()
            .expect("Could not get unsigned integer from JSON Number") as u32
    }
}

pub fn normalize_to_range(a: f32, b: f32, x: f32) -> f32 {
    if a == b {
        // Avoid division by zero if a and b are the same
        return 0.0;
    }

    // Calculate the normalized value
    (x - a) / (b - a)
}

pub fn calc_stops(value: &[Number], count: usize) -> Vec<[f64; 5]> {
    let mut stops: Vec<[f64; 5]> = Vec::new();
    let mut alpha_stops: Vec<(f32, f64)> = Vec::new();
    for chunk in value.chunks_exact(4) {
        stops.push([
            chunk[0].unwrap_f64(),
            chunk[1].unwrap_f64(),
            chunk[2].unwrap_f64(),
            chunk[3].unwrap_f64(),
            1.0,
        ]);
        if stops.len() >= count {
            // there is alpha data at the end of the list, which is a sequence
            // of (offset, alpha) pairs
            for chunk in value.chunks_exact(2).skip(count * 2) {
                let offset = chunk[0].unwrap_f32();
                let alpha = chunk[1].unwrap_f64();
                alpha_stops.push((offset, alpha));
            }

            for stop in stops.iter_mut() {
                let mut last: Option<(f32, f64)> = None;
                for &(b, alpha_b) in alpha_stops.iter() {
                    if let Some((a, alpha_a)) = last.take() {
                        let x = stop[0] as f32;
                        let t = normalize_to_range(a, b, x);

                        let alpha_interp = alpha_a.ease(&alpha_b, t, &Easing::LERP);
                        let alpha_interp = if (x >= a && x <= b) && (t <= 0.25) && (x <= 0.1) {
                            alpha_a
                        } else {
                            alpha_interp
                        }; // todo: this is a hack to get alpha rendering with a
                           // falloff similar to lottiefiles'

                        let alpha_interp = if (x >= a && x <= b) && (t >= 0.75) && (x >= 0.9) {
                            alpha_b
                        } else {
                            alpha_interp
                        }; // todo: this is a hack to get alpha rendering with a
                           // falloff similar to lottiefiles'

                        stop[4] = stop[4].min(alpha_interp);
                    }
                    last = Some((b, alpha_b));
                }
            }
            break;
        }
    }

    stops
}
