// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::schema::shapes::shape_style::ShapeStyleShape;

use super::base_gradient::BaseGradientShape;
use super::base_stroke::BaseStrokeShape;
use super::graphic_element::GraphicElementShape;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GradientStrokeShape {
    #[serde(flatten)]
    pub shape_style: ShapeStyleShape,

    #[serde(flatten)]
    pub base_stroke: BaseStrokeShape,

    #[serde(flatten)]
    pub gradient: BaseGradientShape,
}
