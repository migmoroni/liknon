//! Accessible, product-neutral terminal presentation for Liknon CLIs.
//!
//! The crate owns visual profiles, semantic terminal styles, Clap adaptation,
//! and generic help composition. Capability grammar, domain rendering,
//! machine-readable contracts, process behavior, and exit codes remain with
//! each consuming application.

#![forbid(unsafe_code)]

pub mod arguments;
pub mod clap;
pub mod help;
mod palette;
mod presentation;
mod style;
pub mod terminal;

pub use arguments::{
    PaletteArgument, PresentationArgument, RootVisualOptions, VisualOptions, VisualSelection,
};
pub use palette::PaletteProfile;
pub use presentation::{PresentationMetrics, PresentationProfile};
pub use style::{SemanticRole, StyleAttribute, StyleSpec, TerminalColor, Theme};

/// Canonical Liknon mark used by complete root help pages.
pub const LIKNON_ASCII_LOGO: &str = include_str!("../assets/branding/liknon-ascii.txt");
