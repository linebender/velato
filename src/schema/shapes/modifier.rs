// Copyright 2025 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

use crate::schema::shapes::graphic_element::GraphicElementShape;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifierShape {
    #[serde(flatten)]
    pub graphic_element: GraphicElementShape,
}
