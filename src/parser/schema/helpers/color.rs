// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde_json::Number;

/// Colors are represented as arrays with values between 0 and 1 for the RGB
/// components. for example:
/// [1, 0, 0]
/// [1, 0.5, 0]
/// Note sometimes you might find color values with 4 components (the 4th being
/// alpha) but most player ignore the last component.
pub type Color = Vec<Number>;
