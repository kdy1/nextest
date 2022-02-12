// Copyright (c) The nextest Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This crate contains deserializers for machine-readable output generated by
//! [cargo-nextest](https://nexte.st).
//!
//! Implemented so far:
//! * ✅ Listing tests with [`TestListSummary`]
//! * ✅ Semantic exit codes with [`NextestExitCode`]
//!
//! # Examples
//!
//! Get the list of tests in the repository:
//!
//! ```rust,no_run
//! // This example requires `cargo nextest` to be installed.
//!
//! use nextest_metadata::ListCommand;
//!
//! let command = ListCommand::new();
//! let test_list = command.exec().unwrap();
//!
//! // The result is a TestListSummary.
//! println!("{:?}", test_list);
//! ```
//!
//! # Minimum supported Rust version (MSRV)
//!
//! The minimum supported Rust version is **Rust 1.54.**
//!
//! While this crate is a pre-release (0.x.x) it may have its MSRV bumped in a patch release.
//! Once a crate has reached 1.x, any MSRV bump will be accompanied with a new minor version.
//!
#![warn(missing_docs)]

mod errors;
mod exit_codes;
mod test_list;

pub use errors::*;
pub use exit_codes::*;
pub use test_list::*;
