// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::runtime::model::{Easing, Tween};

pub fn normalize_to_range(a: f64, b: f64, x: f64) -> f64 {
    if a == b {
        // Avoid division by zero if a and b are the same
        return 0.0;
    }

    // Calculate the normalized value
    (x - a) / (b - a)
}

pub fn calc_stops(value: &[f64], count: usize) -> Vec<[f64; 5]> {
    let mut stops: Vec<[f64; 5]> = Vec::new();
    let mut alpha_stops: Vec<(f64, f64)> = Vec::new();
    for chunk in value.chunks_exact(4) {
        stops.push([chunk[0], chunk[1], chunk[2], chunk[3], 1.0]);
        if stops.len() >= count {
            // there is alpha data at the end of the list, which is a sequence
            // of (offset, alpha) pairs
            for chunk in value.chunks_exact(2).skip(count * 2) {
                let offset = chunk[0];
                let alpha = chunk[1];
                alpha_stops.push((offset, alpha));
            }

            for stop in stops.iter_mut() {
                let mut last: Option<(f64, f64)> = None;
                for &(b, alpha_b) in alpha_stops.iter() {
                    if let Some((a, alpha_a)) = last.take() {
                        let x = stop[0];
                        let t = normalize_to_range(a, b, x);

                        let alpha_interp = alpha_a.tween(&alpha_b, t, &Easing::LERP);
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
