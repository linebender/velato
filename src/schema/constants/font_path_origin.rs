// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde_repr::{Deserialize_repr, Serialize_repr};

///
#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
pub enum FontPathOrigin {
    Local = 0,
    CssUrl = 1,
    ScriptUrl = 2,
    FontUrl = 3,
}
