// Copyright 2026 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{RenderSink, Renderer};
use crate::model::{ImageAsset, fixed};
use crate::{Composition, EvaluationError, FilterEffect, FilterLayerResult};
use kurbo::{Affine, PathEl, Shape};
use std::ops::Range;

const MAX_PATH_ELEMENTS: usize = 65_536;
const MAX_COMMANDS: usize = 4_096;
const MAX_LAYER_NAME_BYTES: usize = 256;

/// Owned animation drawing commands produced by [`Renderer::try_prepare`].
///
/// A prepared scene owns its paths, paints, stroke parameters and layer metadata; it does not
/// borrow the composition or renderer. It represents the supplied frame, transform and alpha.
/// Unlike [`Renderer::append_evaluated`], replay does not sample shape or paint properties.
/// It is neither a persistent frame cache nor prepared GPU resources, and serial preparation
/// and replay are not guaranteed to be faster than direct rendering.
///
/// Callers may schedule preparation themselves, without a platform-specific thread pool.
/// Each concurrent preparation needs independently owned mutable [`Renderer`] state. This API
/// does not make concurrent access to a mutable sink safe. Preserve painter order when replaying
/// independently prepared animations, regardless of the order in which preparation completes.
///
/// See [`Renderer::try_prepare`] for restrictions and [`Self::append`] for sink compatibility.
#[derive(Debug)]
pub struct PreparedScene {
    commands: Vec<Command>,
    elements: Vec<PathEl>,
}

struct RecordingSink {
    scene: PreparedScene,
    tolerance: f64,
    rejected: bool,
}

#[derive(Debug)]
enum Command {
    Layer(peniko::BlendMode, f32, Affine, Range<usize>),
    Clip(Affine, Range<usize>),
    Pop,
    Draw(Option<fixed::Stroke>, Affine, fixed::Brush, Range<usize>),
    Begin(String, usize),
    End,
}

impl Renderer {
    /// Evaluates a frame and records drawing commands for later [`PreparedScene::append`].
    ///
    /// This records the renderer's output; it does not add support for Lottie features ignored
    /// by direct rendering.
    ///
    /// - `Ok(Some(scene))`: all emitted commands were supported and within the limits below.
    /// - `Ok(None)`: unsupported content, a preparation limit, or invalid `tolerance` prevented
    ///   preparation. The partial recording is discarded.
    /// - `Err(error)`: animation evaluation failed, as with [`Self::try_append`].
    ///
    /// No destination sink is touched. The caller owns fallback to direct rendering on `None`;
    /// do not append both successful prepared output and direct output for the same frame.
    /// Fallback still requires a sink capable of handling the animation's images and effects.
    ///
    /// # Restrictions
    ///
    /// Image draws (including image brushes) and filter pushes reject preparation. Restrictions
    /// apply to emitted sink calls, not unused assets or effects that emit no filter at this frame.
    /// Preparation accepts at most:
    ///
    /// - 65,536 path elements in total, including draw, clip and layer paths and repeated geometry.
    /// - 4,096 commands, including layer-group callbacks and layer pushes/pops.
    /// - 256 UTF-8 bytes per layer name (not 256 characters). Names are not truncated.
    ///
    /// Exactly these limits are accepted; exceeding any one rejects the entire recording.
    /// An animation with no visible layers still records the composition clip push/pop.
    ///
    /// These limits do not bound total memory or evaluation time. Evaluation and drawing can
    /// continue after the recorder rejects, including temporary allocations and repeater expansion.
    /// Renderer scratch, paint/stroke payload sizes and the number of outstanding prepared scenes
    /// are not bounded by these limits. Preparation is not a safeguard for untrusted animations.
    ///
    /// # Arguments
    ///
    /// `frame` follows [`Composition::evaluate`], including errors for non-finite time. Evaluation
    /// errors take precedence over rejection. `transform` and `alpha` follow [`Self::try_append`]
    /// without additional validation or clamping; callers should supply finite values.
    ///
    /// `tolerance` must be finite and strictly positive; otherwise this returns `Ok(None)` after
    /// animation evaluation succeeds. It is passed to [`Shape::path_elements`] in each shape's
    /// local coordinates, before its recorded transform is applied. It does not control earlier
    /// geometry evaluation, trimming, sink stroking or rasterization. Currently the renderer emits
    /// rectangles and existing Bézier paths, whose path conversion is exact and tolerance-independent.
    /// See [`PreparedScene::append`] for the limitations of path-based replay.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use kurbo::Affine;
    /// use velato::{Composition, EvaluationError, RenderSink, Renderer};
    ///
    /// fn append_frame(
    ///     renderer: &mut Renderer,
    ///     animation: &Composition,
    ///     frame: f64,
    ///     sink: &mut impl RenderSink,
    /// ) -> Result<(), EvaluationError> {
    ///     let transform = Affine::IDENTITY;
    ///     let alpha = 1.0;
    ///     match renderer.try_prepare(animation, frame, transform, alpha, 0.1)? {
    ///         Some(prepared) => prepared.append(sink),
    ///         None => renderer.try_append(animation, frame, transform, alpha, sink)?,
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn try_prepare(
        &mut self,
        animation: &Composition,
        frame: f64,
        transform: Affine,
        alpha: f64,
        tolerance: f64,
    ) -> Result<Option<PreparedScene>, EvaluationError> {
        let evaluated = animation.evaluate(frame)?;
        if !tolerance.is_finite() || tolerance <= 0.0 {
            return Ok(None);
        }
        let mut sink = RecordingSink {
            scene: PreparedScene {
                commands: Vec::new(),
                elements: Vec::new(),
            },
            tolerance,
            rejected: false,
        };
        self.append_evaluated(&evaluated, transform, alpha, &mut sink);
        Ok((!sink.rejected).then_some(sink.scene))
    }
}

impl PreparedScene {
    /// Replays the recorded commands, appending them to `sink` in their original order.
    ///
    /// Replay neither clears the sink nor changes this scene. Each call appends another set of
    /// draws, so a scene can be replayed repeatedly or into different compatible sinks. It does
    /// not advance time, resample properties or apply a fresh transform or alpha.
    ///
    /// Layer/clip nesting, blend modes, opacities, transforms, paints, stroke parameters and
    /// layer-group callback order, names and indices are preserved from direct rendering.
    /// Shapes are replayed as path-element slices, not their original concrete types. Sinks that
    /// consume equivalent paths can use replay in place of direct rendering. Primitive-specific
    /// queries such as [`Shape::as_rect`], geometry queries such as bounds or containment, or a
    /// different path-conversion policy need not behave identically for arbitrary shapes.
    /// Replay does not reconvert paths using the sink's tolerance.
    pub fn append(&self, sink: &mut impl RenderSink) {
        self.append_inner(sink, |recorded| recorded);
    }

    /// Replays with an additional instance transform, using `transform * recorded_transform`
    /// for every draw, clip and layer. The preparation transform is not replaced.
    ///
    /// Prepare with [`Affine::IDENTITY`] to keep instance placement entirely in replay.
    /// Paths are not rebuilt, properties are not resampled, and recorded alpha is unchanged.
    /// The sink compatibility and path-tolerance limitations of [`Self::append`] still apply.
    pub fn append_with_transform(&self, sink: &mut impl RenderSink, transform: Affine) {
        if transform == Affine::IDENTITY {
            self.append(sink);
        } else {
            self.append_inner(sink, |recorded| transform * recorded);
        }
    }

    fn append_inner(&self, sink: &mut impl RenderSink, transform: impl Fn(Affine) -> Affine) {
        for command in &self.commands {
            match command {
                Command::Layer(blend, alpha, recorded, range) => sink.push_layer(
                    *blend,
                    *alpha,
                    transform(*recorded),
                    &&self.elements[range.clone()],
                ),
                Command::Clip(recorded, range) => {
                    sink.push_clip_layer(transform(*recorded), &&self.elements[range.clone()])
                }
                Command::Pop => sink.pop_layer(),
                Command::Draw(stroke, recorded, brush, range) => sink.draw(
                    stroke.as_ref(),
                    transform(*recorded),
                    brush,
                    &&self.elements[range.clone()],
                ),
                Command::Begin(name, index) => sink.begin_layer_group(name, *index),
                Command::End => sink.end_layer_group(),
            }
        }
    }
}

impl RecordingSink {
    fn has_capacity(&mut self) -> bool {
        if self.scene.commands.len() >= MAX_COMMANDS {
            self.rejected = true;
        }
        !self.rejected
    }

    fn push(&mut self, command: impl FnOnce() -> Command) {
        if self.has_capacity() {
            self.scene.commands.push(command());
        }
    }

    fn path(&mut self, shape: &impl Shape) -> Option<Range<usize>> {
        if !self.has_capacity() {
            return None;
        }
        let start = self.scene.elements.len();
        for element in shape.path_elements(self.tolerance) {
            if self.scene.elements.len() >= MAX_PATH_ELEMENTS {
                self.rejected = true;
                return None;
            }
            self.scene.elements.push(element);
        }
        Some(start..self.scene.elements.len())
    }
}

impl RenderSink for RecordingSink {
    fn push_layer(
        &mut self,
        blend: impl Into<peniko::BlendMode>,
        alpha: f32,
        transform: Affine,
        shape: &impl Shape,
    ) {
        if let Some(path) = self.path(shape) {
            self.push(|| Command::Layer(blend.into(), alpha, transform, path));
        }
    }

    fn push_clip_layer(&mut self, transform: Affine, shape: &impl Shape) {
        if let Some(path) = self.path(shape) {
            self.push(|| Command::Clip(transform, path));
        }
    }

    fn pop_layer(&mut self) {
        self.push(|| Command::Pop);
    }

    fn draw(
        &mut self,
        stroke: Option<&fixed::Stroke>,
        transform: Affine,
        brush: &fixed::Brush,
        shape: &impl Shape,
    ) {
        if matches!(brush, fixed::Brush::Image(_)) {
            self.rejected = true;
            return;
        }
        if let Some(path) = self.path(shape) {
            self.push(|| Command::Draw(stroke.cloned(), transform, brush.clone(), path));
        }
    }

    fn draw_image(&mut self, _: &ImageAsset, _: Affine, _: f64) {
        self.rejected = true;
    }

    fn push_filter(&mut self, _: Affine, _: &FilterEffect) -> FilterLayerResult {
        self.rejected = true;
        FilterLayerResult::Unsupported
    }

    fn begin_layer_group(&mut self, name: &str, index: usize) {
        if name.len() > MAX_LAYER_NAME_BYTES {
            self.rejected = true;
        }
        self.push(|| Command::Begin(name.to_owned(), index));
    }

    fn end_layer_group(&mut self) {
        self.push(|| Command::End);
    }
}
