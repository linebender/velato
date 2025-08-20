// Copyright 2024 the Velato Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// The following lints are part of the Linebender standard set,
// but resolving them has been deferred for now.
// Feel free to send a PR that solves one or more of these.
#![allow(
    unreachable_pub,
    missing_docs,
    elided_lifetimes_in_paths,
    single_use_lifetimes,
    unused_qualifications,
    clippy::empty_docs,
    clippy::use_self,
    clippy::return_self_not_must_use,
    clippy::cast_possible_truncation,
    clippy::shadow_unrelated,
    clippy::missing_assert_message,
    clippy::missing_errors_doc,
    clippy::exhaustive_enums,
    clippy::todo,
    reason = "Deferred"
)]
#![cfg_attr(
    test,
    allow(
        unused_crate_dependencies,
        reason = "Some dev dependencies are only used in tests"
    )
)]

pub(crate) mod import;
pub(crate) mod runtime;
pub(crate) mod schema;

pub use runtime::{Composition, model};