//! Terminal adaptation and human presentation for the Liknon validation CLI.

#![warn(missing_docs)]
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]

mod cli;
mod reporting;
mod theme;

pub(crate) use liknon_validation::{
    config, contracts, execution, initialization, planning, ValidatorError,
};

pub use cli::run_cli;
