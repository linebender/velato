// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod asset;
pub mod file_asset;
pub mod image;
pub mod precomposition;

use self::image::Image;
use self::precomposition::Precomposition;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum AnyAsset {
    Image(Image),
    Precomposition(Precomposition),
    // unimplemented - Sound(Sound),
    // unimplemented - DataSource(DataSource),
}
