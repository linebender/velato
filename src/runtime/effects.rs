// Copyright 2026 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::model::LayerEffect;
use kurbo::Vec2;
use peniko::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub enum FilterLayerResult {
    Pushed,
    Unsupported,
}

/// How Gaussian blur samples outside the source bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlurEdgeMode {
    /// Extend the edge samples beyond the source bounds.
    Duplicate,
    /// Tile the source outside its bounds.
    Wrap,
}

/// A [`LayerEffect`] evaluated for rendering a single frame.
///
/// We implement these like LottieFiles' ThorVG-powered player for now as their editor is commonly
/// used to author dotlottie animations.
///
/// This can ofcourse change in the future.
#[derive(Clone, Debug)]
pub enum FilterEffect {
    /// Blurs source color and alpha together.
    GaussianBlur {
        std_deviation: Vec2,
        edge_mode: BlurEdgeMode,
    },
    /// Blurs source alpha and scales it by shadow opacity, then draws the source over it.
    ///
    /// The effect color's authored alpha is ignored.
    DropShadow {
        color: Color,
        offset: Vec2,
        std_deviation: f64,
    },
    /// Replaces source RGB; output alpha is source alpha times effect opacity.
    ///
    /// The effect color's authored alpha is ignored.
    Fill { color: Color },
    /// Changes source RGB while keeping source alpha.
    ///
    /// Effect color alpha is ignored.
    Tint {
        black: Color,
        white: Color,
        amount: f64,
    },
}

impl LayerEffect {
    pub fn evaluate(&self, frame: f64) -> Option<FilterEffect> {
        let nonnegative = |value: f64| {
            if value.is_finite() {
                value.max(0.0)
            } else {
                0.0
            }
        };
        Some(match self {
            Self::GaussianBlur {
                blurriness,
                dimensions,
                wrap,
            } => {
                // Match ThorVG's Lottie blurriness-to-sigma conversion.
                let sigma = nonnegative(blurriness.evaluate(frame)) * 0.3;
                if sigma == 0.0 {
                    return None;
                }
                let dimensions = dimensions.evaluate(frame) as u32;
                FilterEffect::GaussianBlur {
                    std_deviation: Vec2::new(
                        if dimensions == 3 { 0.0 } else { sigma },
                        if dimensions == 2 { 0.0 } else { sigma },
                    ),
                    edge_mode: if wrap.evaluate(frame) == 1.0 {
                        BlurEdgeMode::Wrap
                    } else {
                        BlurEdgeMode::Duplicate
                    },
                }
            }
            Self::DropShadow {
                color,
                opacity,
                angle,
                distance,
                softness,
            } => {
                let opacity = (nonnegative(opacity.evaluate(frame)) / 255.0).min(1.0);
                if opacity == 0.0 {
                    return None;
                }
                let angle = angle.evaluate(frame).to_radians();
                let distance = distance.evaluate(frame);
                if !angle.is_finite() || !distance.is_finite() {
                    return None;
                }
                FilterEffect::DropShadow {
                    color: color
                        .evaluate_or(frame, Color::TRANSPARENT)
                        .with_alpha(opacity as f32),
                    offset: Vec2::new(angle.sin() * distance, -angle.cos() * distance),
                    // ThorVG uses the same conversion for Gaussian Blur and Drop Shadow.
                    std_deviation: nonnegative(softness.evaluate(frame)) * 0.3,
                }
            }
            Self::Fill { color, opacity } => FilterEffect::Fill {
                color: color
                    .evaluate_or(frame, Color::TRANSPARENT)
                    .with_alpha(nonnegative(opacity.evaluate(frame)).min(1.0) as f32),
            },
            Self::Tint {
                black,
                white,
                amount,
            } => {
                let amount = (nonnegative(amount.evaluate(frame)) / 100.0).min(1.0);
                if amount == 0.0 {
                    return None;
                }
                FilterEffect::Tint {
                    black: black.evaluate_or(frame, Color::BLACK).with_alpha(1.0),
                    white: white.evaluate_or(frame, Color::WHITE).with_alpha(1.0),
                    amount,
                }
            }
        })
    }
}
