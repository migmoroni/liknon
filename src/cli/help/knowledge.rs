//! Help text for embedded validation knowledge.

pub(in crate::cli) const ABOUT: &str =
    "Reads validation guidance embedded in this binary without loading configuration";

pub(in crate::cli) const CATALOG_ABOUT: &str =
    "Lists the embedded catalog used for progressive discovery";

pub(in crate::cli) const CATALOG_FORMAT: &str = "Selects the embedded catalog format";

pub(in crate::cli) const CATALOG_FORMAT_LONG: &str = "Selects the embedded catalog format.\n\nHuman output is the default and prints a readable catalog. JSON writes the exact embedded canonical catalog for machine consumption.";

pub(in crate::cli) const SHOW_ABOUT: &str =
    "Writes one canonical Markdown document selected by stable ID";

pub(in crate::cli) const DOCUMENT_ID: &str =
    "Stable document ID returned by the embedded knowledge catalog";
