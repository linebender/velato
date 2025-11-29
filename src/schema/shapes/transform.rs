// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::graphic_element::GraphicElementShape;
use crate::schema::helpers::transform::Transform;
use serde::{Deserialize, Serialize};

/// A rectangle, defined by its center point and size.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TransformShape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,
    #[serde(flatten)]
    pub transform: Transform,
}
