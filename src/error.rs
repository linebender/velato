// Copyright 2023 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// Failure to parse or import a Lottie animation.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Json(serde_json::Error),
    InvalidFrameRate(f64),
}

impl core::error::Error for Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(err) => write!(f, "Error parsing lottie: {err}"),
            Self::InvalidFrameRate(rate) => {
                write!(
                    f,
                    "Invalid animation frame rate {rate}: expected a finite, positive value"
                )
            }
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
