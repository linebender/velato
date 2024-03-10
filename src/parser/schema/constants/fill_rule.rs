// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Rule used to handle multiple shapes rendered with the same fill object
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum FillRule {
    /// Everything is colored (You can think of this as an OR)
    NonZero = 1,
    /// Colored based on intersections and path direction, can be used to
    /// create "holes"
    EvenOdd = 2,
}
