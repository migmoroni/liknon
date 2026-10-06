#![allow(dead_code)]

mod contracts;
mod integrity;
mod recipes;
mod release;

use std::{env, path::Path};

// This file is both an example entrypoint and the module imported by integration tests.
// Re-exports form the small testing surface and are not all consumed by the binary itself.
#[allow(unused_imports)]
pub use contracts::{Catalog, CompiledSchemas, SchemaKind, SourceRegister};
#[allow(unused_imports)]
pub use integrity::{
    embedded_source_assets, tree_digest, validate_catalog_integrity,
    validate_embedded_asset_inventory,
};
#[allow(unused_imports)]
pub use recipes::{extract_recipe_json, extract_recipe_json_text};
#[allow(unused_imports)]
pub use release::{
    load_catalog, load_sources, render_sources, validate_catalog_source_values,
    validate_generated_sources, verify, write_generated,
};

pub const CANONICAL_ROOT: &str = "docs/validation/knowledge";

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--check".into());
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let result = match argument.as_str() {
        "--check" => verify(repository),
        "--write" => write_generated(repository),
        _ => Err(format!(
            "unknown argument {argument}; use --check or --write"
        )),
    };
    match result {
        Ok(digest) => println!("knowledge sha256-tree-v1 {digest}"),
        Err(error) => {
            eprintln!("knowledge release check failed: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests;
