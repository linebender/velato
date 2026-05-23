// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents boolean value… 0 is false, 1 is true.
#[derive(Deserialize_repr, Serialize_repr, PartialEq, Default, Debug, Clone)]
#[repr(u8)]
pub enum BoolInt {
    /// 0 = False
    #[default]
    False = 0,
    /// 1 = True
    True = 1,
}
