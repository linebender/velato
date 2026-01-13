// Copyright 2023 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Triggered when is an issue parsing a lottie file.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Json(serde_json::Error),
}

impl core::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(err) => write!(f, "Error parsing lottie: {err}"),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
