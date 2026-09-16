// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde_repr::{Deserialize_repr, Serialize_repr};

/// How to handle multiple shapes in trim path
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum TrimMultipleShapes {
    /// Apply the range independently to each contour.
    Parallel = 1,
    /// Apply the range across combined contour lengths.
    Sequential = 2,
}
