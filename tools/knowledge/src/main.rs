use liknon_knowledge_maintenance::{repository_root, verify, write_generated};
use std::env;

fn main() {
    let argument = env::args().nth(1).unwrap_or_else(|| "--check".into());
    let repository = repository_root();
    let result = match argument.as_str() {
        "--check" => verify(&repository),
        "--write" => write_generated(&repository),
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
