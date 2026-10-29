//! Declarative, shell-free validation for heterogeneous workspaces.
//!
//! This crate owns configuration loading, semantic validation, planning,
//! execution, repository evidence, structured reports, and terminal adaptation
//! for the `liknon-validator` executable owned by the `liknon-cli` suite.

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]

mod dag;
mod error;
mod process;
mod repository;

mod cli;
mod reporting;
mod theme;

pub mod config;
pub mod contracts;
pub mod execution;
pub mod initialization;
pub mod planning;

pub use cli::run_cli_with_version;
pub use error::ValidatorError;
