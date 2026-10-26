//! Repository-only validation for authored Liknon knowledge.

mod contracts;
mod integrity;
mod recipes;
mod release;

use std::path::{Path, PathBuf};

pub use contracts::{Catalog, CompiledSchemas, SchemaKind, SourceRegister};
pub use integrity::{
    canonical_source_assets, tree_digest, validate_catalog_integrity,
    validate_source_asset_inventory,
};
pub use recipes::{extract_recipe_json, extract_recipe_json_text};
pub use release::{
    load_catalog, load_sources, render_sources, validate_catalog_source_values,
    validate_generated_sources, verify, write_generated,
};

/// Repository-relative location of the canonical authored corpus.
pub const CANONICAL_ROOT: &str = "docs/validation/knowledge";

/// Returns the repository root that owns this private workspace package.
pub fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[cfg(test)]
mod tests;
