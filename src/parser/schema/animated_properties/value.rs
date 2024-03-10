// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::animated_property::AnimatedProperty;
use serde::{Deserialize, Serialize};
use serde_json::Number;

/// aka Value, in the Schema.
///
/// An animatable property that holds a float.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FloatValue {
    #[serde(flatten)]
    pub animated_property: AnimatedProperty<Number>,
}
