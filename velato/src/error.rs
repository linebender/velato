// Copyright 2023 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use thiserror::Error;

/// Triggered when is an issue parsing a lottie file.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Error parsing lottie: {0}")]
    Json(#[from] serde_json::Error),
}
