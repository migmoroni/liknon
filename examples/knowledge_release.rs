#[path = "../tools/knowledge.rs"]
mod knowledge;

use std::{env, path::Path};

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--check".into());
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let result = match argument.as_str() {
        "--check" => knowledge::verify(repository),
        "--write" => knowledge::write_generated(repository),
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
