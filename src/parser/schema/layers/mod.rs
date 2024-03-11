// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod enumerations;
pub mod null;
pub mod precomposition;
pub mod shape;
pub mod solid_color;
pub mod visual;

use self::solid_color::SolidColorLayer;
use self::visual::VisualLayer;
use null::NullLayer;
use precomposition::PrecompositionLayer;
use serde::{Deserialize, Serialize};
use shape::ShapeLayer;

/// There are several layer types, which is specified by the 'ty' attribute. All
/// layers share the properties in `layers::common::Properties`.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyLayer {
    /// Renders a Precomposition
    Precomposition(PrecompositionLayer),

    /// Static rectangle filling the canvas with a single color
    SolidColor(SolidColorLayer),

    /// Renders an Image
    // todo Image

    /// No contents, only used for parenting

    /// Has an array of shapes
    Shape(ShapeLayer),
    // Renders Text
    // todo Text
    Null(NullLayer),
    // unimplemented - Audio(AudioLayer),
    // unimplemented - VideoPlaceholder(VideoPlaceholderLayer)
    // unimplemented - Video(VideoLayer)
    // unimplemented - ImagePlaceholder(ImagePlaceholderLayer)
    // unimplemented - Guide(GuideLayer)
    // unimplemented - Adjustment(AdjustmentLayer)
    // unimplemented - Camera(CameraLayer)
    // unimplemented - Light(LightLayer)
    // unimplemented - Data(DataLayer)
}
