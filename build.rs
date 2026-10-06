use std::{
    collections::BTreeSet,
    env,
    fmt::Write as _,
    fs,
    path::{Component, Path, PathBuf},
};

const KNOWLEDGE_ROOT: &str = "docs/validation/knowledge";

fn main() {
    if let Err(error) = generate_registry() {
        panic!("cannot embed validation knowledge: {error}");
    }
}

fn generate_registry() -> Result<(), String> {
    let repository = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").ok_or("CARGO_MANIFEST_DIR is unavailable")?,
    );
    let root = repository.join(KNOWLEDGE_ROOT);
    let metadata = fs::symlink_metadata(&root)
        .map_err(|error| format!("cannot inspect {}: {error}", root.display()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!("{} must be a real directory", root.display()));
    }

    println!("cargo:rerun-if-changed={}", root.display());
    let mut files = Vec::new();
    collect_files(&root, Path::new(""), &mut files)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut paths = BTreeSet::new();
    let mut generated = String::from("const EMBEDDED_ASSETS: &[EmbeddedAsset] = &[\n");
    for (relative, source) in files {
        if !paths.insert(relative.clone()) {
            return Err(format!("duplicate normalized knowledge path {relative}"));
        }
        let bytes = fs::read(&source)
            .map_err(|error| format!("cannot read {}: {error}", source.display()))?;
        std::str::from_utf8(&bytes)
            .map_err(|error| format!("{} is not UTF-8: {error}", source.display()))?;
        println!("cargo:rerun-if-changed={}", source.display());
        writeln!(
            generated,
            "    EmbeddedAsset {{ path: {relative:?}, bytes: include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{KNOWLEDGE_ROOT}/{relative}\")) }},"
        )
        .map_err(|error| error.to_string())?;
    }
    generated.push_str("];\n");

    let output = PathBuf::from(env::var_os("OUT_DIR").ok_or("OUT_DIR is unavailable")?)
        .join("knowledge_assets.rs");
    fs::write(&output, generated)
        .map_err(|error| format!("cannot write {}: {error}", output.display()))
}

fn collect_files(
    root: &Path,
    relative: &Path,
    files: &mut Vec<(String, PathBuf)>,
) -> Result<(), String> {
    let directory = root.join(relative);
    let mut entries = fs::read_dir(&directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let child = relative.join(entry.file_name());
        let kind = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", entry.path().display()))?;
        if kind.is_symlink() {
            return Err(format!(
                "knowledge cannot contain symlink {}",
                child.display()
            ));
        }
        if kind.is_dir() {
            collect_files(root, &child, files)?;
            continue;
        }
        if !kind.is_file() {
            return Err(format!("unsupported knowledge entry {}", child.display()));
        }
        let normalized = normalize_relative(&child)?;
        files.push((normalized, entry.path()));
    }
    Ok(())
}

fn normalize_relative(path: &Path) -> Result<String, String> {
    let mut parts = Vec::new();
    for component in path.components() {
        let Component::Normal(value) = component else {
            return Err(format!("unsafe knowledge path {}", path.display()));
        };
        let value = value
            .to_str()
            .ok_or_else(|| format!("non-UTF-8 knowledge path {}", path.display()))?;
        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(format!("unsupported knowledge path {}", path.display()));
        }
        parts.push(value);
    }
    if parts.is_empty() {
        return Err("knowledge path cannot be empty".into());
    }
    Ok(parts.join("/"))
}
