// Copyright 2023 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use scenes::RobotoText;
use std::collections::VecDeque;
use vello::kurbo::{Affine, PathEl, Rect, Stroke};
use vello::low_level::BumpAllocators;
use vello::peniko::{Brush, Color, Fill};
use vello::{AaConfig, Scene};

const SLIDING_WINDOW_SIZE: usize = 100;

#[derive(Debug)]
pub(crate) struct Snapshot {
    pub fps: f64,
    pub frame_time_ms: f64,
    pub frame_time_min_ms: f64,
    pub frame_time_max_ms: f64,
}

impl Snapshot {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn draw_layer<'a, T>(
        &self,
        scene: &mut Scene,
        text: &mut RobotoText,
        viewport_width: f64,
        viewport_height: f64,
        samples: T,
        bump: Option<BumpAllocators>,
        vsync: bool,
        aa_config: AaConfig,
    ) where
        T: Iterator<Item = &'a u64>,
    {
        let width = (viewport_width * 0.4).clamp(200., 600.);
        let height = width * 0.7;
        let x_offset = viewport_width - width;
        let y_offset = viewport_height - height;
        let offset = Affine::translate((x_offset, y_offset));

        // Draw the background
        scene.fill(
            Fill::NonZero,
            offset,
            &Brush::Solid(Color::from_rgba8(0, 0, 0, 200)),
            None,
            &Rect::new(0., 0., width, height),
        );

        let mut labels = vec![
            format!("Frame Time: {:.2} ms", self.frame_time_ms),
            format!("Frame Time (min): {:.2} ms", self.frame_time_min_ms),
            format!("Frame Time (max): {:.2} ms", self.frame_time_max_ms),
            format!("VSync: {}", if vsync { "on" } else { "off" }),
            format!(
                "AA method: {}",
                match aa_config {
                    AaConfig::Area => "Analytic Area",
                    AaConfig::Msaa16 => "16xMSAA",
                    AaConfig::Msaa8 => "8xMSAA",
                }
            ),
            format!("Resolution: {viewport_width}x{viewport_height}"),
        ];
        if let Some(bump) = &bump {
            if bump.failed >= 1 {
                labels.push("Allocation Failed!".into());
            }
            labels.push(format!("binning: {}", bump.binning));
            labels.push(format!("ptcl: {}", bump.ptcl));
            labels.push(format!("tile: {}", bump.tile));
            labels.push(format!("segments: {}", bump.segments));
            labels.push(format!("blend: {}", bump.blend));
        }

        // height / 2 is dedicated to the text labels and the rest is filled by the bar graph.
        let text_height = height * 0.5 / (1 + labels.len()) as f64;
        let left_margin = width * 0.01;
        let text_size = (text_height * 0.9) as f32;
        for (i, label) in labels.iter().enumerate() {
            text.add(
                scene,
                None,
                text_size,
                Some(&Brush::Solid(Color::WHITE)),
                offset * Affine::translate((left_margin, (i + 1) as f64 * text_height)),
                label,
            );
        }
        text.add(
            scene,
            None,
            text_size,
            Some(&Brush::Solid(Color::WHITE)),
            offset * Affine::translate((width * 0.67, text_height)),
            &format!("FPS: {:.2}", self.fps),
        );

        // Plot the samples with a bar graph
        use PathEl::*;
        let left_padding = width * 0.05; // Left padding for the frame time marker text.
        let graph_max_height = height * 0.5;
        let graph_max_width = width - 2. * left_margin - left_padding;
        let left_margin_padding = left_margin + left_padding;
        let bar_extent = graph_max_width / (SLIDING_WINDOW_SIZE as f64);
        let bar_width = bar_extent * 0.4;
        let bar = [
            MoveTo((0., graph_max_height).into()),
            LineTo((0., 0.).into()),
            LineTo((bar_width, 0.).into()),
            LineTo((bar_width, graph_max_height).into()),
        ];
        // We determine the scale of the graph based on the maximum sampled frame time unless it's
        // greater than 3x the current average. In that case we cap the max scale at 4/3 * the
        // current average (rounded up to the nearest multiple of 5ms). This allows the scale to
        // adapt to the most recent sample set as relying on the maximum alone can make the
        // displayed samples to look too small in the presence of spikes/fluctuation without
        // manually resetting the max sample.
        let display_max = if self.frame_time_max_ms > 3. * self.frame_time_ms {
            round_up((1.33334 * self.frame_time_ms) as usize, 5) as f64
        } else {
            self.frame_time_max_ms
        };
        for (i, sample) in samples.enumerate() {
            let t = offset * Affine::translate((i as f64 * bar_extent, graph_max_height));
            // The height of each sample is based on its ratio to the maximum observed frame time.
            let sample_ms = ((*sample as f64) * 0.001).min(display_max);
            let h = sample_ms / display_max;
            let s = Affine::scale_non_uniform(1., -h);
            #[allow(clippy::match_overlapping_arm)]
            let color = match *sample {
                ..=16_667 => Color::from_rgb8(100, 143, 255),
                ..=33_334 => Color::from_rgb8(255, 176, 0),
                _ => Color::from_rgb8(220, 38, 127),
            };
            scene.fill(
                Fill::NonZero,
                t * Affine::translate((
                    left_margin_padding,
                    (1 + labels.len()) as f64 * text_height,
                )) * s,
                color,
                None,
                &bar,
            );
        }
        // Draw horizontal lines to mark 8.33ms, 16.33ms, and 33.33ms
        let marker = [
            MoveTo((0., graph_max_height).into()),
            LineTo((graph_max_width, graph_max_height).into()),
        ];
        let thresholds = [8.33, 16.66, 33.33];
        let thres_text_height = graph_max_height * 0.05;
        let thres_text_height_2 = thres_text_height * 0.5;
        for t in thresholds.iter().filter(|&&t| t < display_max) {
            let y = t / display_max;
            text.add(
                scene,
                None,
                thres_text_height as f32,
                Some(&Brush::Solid(Color::WHITE)),
                offset
                    * Affine::translate((
                        left_margin,
                        (2. - y) * graph_max_height + thres_text_height_2,
                    )),
                &format!("{t}"),
            );
            scene.stroke(
                &Stroke::new(graph_max_height * 0.01),
                offset * Affine::translate((left_margin_padding, (1. - y) * graph_max_height)),
                Color::WHITE,
                None,
                &marker,
            );
        }
    }
}

pub(crate) struct Sample {
    pub frame_time_us: u64,
}

pub(crate) struct Stats {
    count: usize,
    sum: u64,
    min: u64,
    max: u64,
    samples: VecDeque<u64>,
}

impl Stats {
    pub(crate) fn new() -> Stats {
        Stats {
            count: 0,
            sum: 0,
            min: u64::MAX,
            max: u64::MIN,
            samples: VecDeque::with_capacity(SLIDING_WINDOW_SIZE),
        }
    }

    pub(crate) fn samples(&self) -> impl Iterator<Item = &u64> {
        self.samples.iter()
    }

    pub(crate) fn snapshot(&self) -> Snapshot {
        let frame_time_ms = (self.sum as f64 / self.count as f64) * 0.001;
        let fps = 1000. / frame_time_ms;
        Snapshot {
            fps,
            frame_time_ms,
            frame_time_min_ms: self.min as f64 * 0.001,
            frame_time_max_ms: self.max as f64 * 0.001,
        }
    }

    pub(crate) fn clear_min_and_max(&mut self) {
        self.min = u64::MAX;
        self.max = u64::MIN;
    }

    pub(crate) fn add_sample(&mut self, sample: Sample) {
        let oldest = if self.count < SLIDING_WINDOW_SIZE {
            self.count += 1;
            None
        } else {
            self.samples.pop_front()
        };
        let micros = sample.frame_time_us;
        self.sum += micros;
        self.samples.push_back(micros);
        if let Some(oldest) = oldest {
            self.sum -= oldest;
        }
        self.min = self.min.min(micros);
        self.max = self.max.max(micros);
    }
}

fn round_up(n: usize, f: usize) -> usize {
    n - 1 - (n - 1) % f + f
}
