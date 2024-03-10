// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::parser::schema::layers::AnyLayer;
use serde::{Deserialize, Serialize};

/// Base class for layer holders
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Composition {
    /// An array of Layers
    #[serde(rename = "layers")]
    pub layers: Vec<AnyLayer>,
}
