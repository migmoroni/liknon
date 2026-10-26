//! Terminal-independent validation engine used by Liknon applications.
//!
//! This crate owns configuration loading, semantic validation, planning,
//! shell-free execution, repository evidence, initialization, and structured
//! reports. It is an internal implementation boundary, not the future public
//! `liknon` integration package.

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]

mod dag;
mod error;
mod process;
mod repository;

pub mod config;
pub mod contracts;
pub mod execution;
pub mod initialization;
pub mod planning;

pub use error::ValidatorError;
