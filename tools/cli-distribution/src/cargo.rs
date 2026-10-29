use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use semver::Version;
use serde::Deserialize;

use crate::{
    error::{contract, Error, Result},
    hash::sha256,
    io::safe_relative_path,
    model::{ProductDescriptor, ReleaseIdentity, SCHEMA_VERSION},
};

#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    version: String,
    manifest_path: PathBuf,
    repository: Option<String>,
    publish: Option<Vec<String>>,
    targets: Vec<CargoTarget>,
}

#[derive(Debug, Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
}

pub(crate) fn inspect(
    descriptor: &ProductDescriptor,
    workspace: &Path,
    release: bool,
) -> Result<ReleaseIdentity> {
    descriptor.validate()?;
    let metadata = cargo_metadata(workspace)?;
    let package = metadata
        .packages
        .iter()
        .find(|package| package.name == descriptor.product.cargo_package)
        .ok_or_else(|| {
            contract(format!(
                "Cargo package {} was not found",
                descriptor.product.cargo_package
            ))
        })?;
    validate_package(package, descriptor, workspace)?;
    let version = Version::parse(&package.version)
        .map_err(|error| contract(format!("invalid package version: {error}")))?;
    if !version.pre.is_empty()
        || !version.build.is_empty()
        || version.to_string() != package.version
    {
        return Err(contract(
            "package version must use canonical MAJOR.MINOR.PATCH form",
        ));
    }
    let commit = command_text(workspace, "git", &["rev-parse", "HEAD"])?;
    if commit.len() != 40 || !commit.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(contract("Git HEAD did not resolve to a full commit ID"));
    }
    let tag = format!("{}{}", descriptor.product.tag_prefix, package.version);
    if release {
        validate_release_tag(workspace, &tag, &commit)?;
        if !command_text(workspace, "git", &["status", "--porcelain"])?.is_empty() {
            return Err(contract("release inspection requires a clean worktree"));
        }
    }
    let lock_path = workspace.join("Cargo.lock");
    let (_, cargo_lock_sha256) = sha256(&lock_path)?;
    let mut binary_targets = package
        .targets
        .iter()
        .filter(|target| target.kind.iter().any(|kind| kind == "bin"))
        .map(|target| target.name.clone())
        .collect::<Vec<_>>();
    binary_targets.sort();
    Ok(ReleaseIdentity {
        schema_version: SCHEMA_VERSION,
        product: descriptor.product.id.clone(),
        package: descriptor.product.cargo_package.clone(),
        package_path: descriptor.product.cargo_manifest.clone(),
        version: package.version.clone(),
        tag,
        commit,
        cargo_lock_sha256,
        binary_targets,
    })
}

fn cargo_metadata(workspace: &Path) -> Result<CargoMetadata> {
    let manifest = workspace.join("Cargo.toml");
    let manifest = manifest
        .to_str()
        .ok_or_else(|| contract("workspace manifest path is not valid UTF-8"))?;
    let output = run(
        workspace,
        "cargo",
        &[
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--no-deps",
            "--manifest-path",
            manifest,
        ],
    )?;
    serde_json::from_slice(&output.stdout).map_err(|source| Error::Json {
        path: PathBuf::from("cargo metadata stdout"),
        source,
    })
}

fn validate_package(
    package: &CargoPackage,
    descriptor: &ProductDescriptor,
    workspace: &Path,
) -> Result<()> {
    let relative_manifest =
        safe_relative_path(&descriptor.product.cargo_manifest, "product.cargoManifest")?;
    let expected_manifest = workspace
        .join(relative_manifest)
        .canonicalize()
        .map_err(|source| Error::Read {
            path: workspace.join(&descriptor.product.cargo_manifest),
            source,
        })?;
    let actual_manifest = package
        .manifest_path
        .canonicalize()
        .map_err(|source| Error::Read {
            path: package.manifest_path.clone(),
            source,
        })?;
    if expected_manifest != actual_manifest {
        return Err(contract("Cargo package path does not match the descriptor"));
    }
    let repository = package.repository.as_deref().unwrap_or_default();
    let expected_repository = format!("https://github.com/{}", descriptor.product.repository);
    if repository.trim_end_matches(".git") != expected_repository {
        return Err(contract(
            "Cargo package repository does not match the descriptor",
        ));
    }
    if package.publish.as_deref() != Some(&["crates-io".to_owned()]) {
        return Err(contract(
            "selected package must publish only to the crates-io registry",
        ));
    }
    let actual = package
        .targets
        .iter()
        .filter(|target| target.kind.iter().any(|kind| kind == "bin"))
        .map(|target| target.name.as_str())
        .collect::<BTreeSet<_>>();
    let expected = descriptor
        .product
        .commands
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(contract(format!(
            "Cargo binary targets do not match the descriptor: expected {expected:?}, found {actual:?}"
        )));
    }
    Ok(())
}

fn validate_release_tag(workspace: &Path, tag: &str, commit: &str) -> Result<()> {
    let reference = format!("refs/tags/{tag}");
    let kind = command_text(workspace, "git", &["cat-file", "-t", &reference])?;
    if kind != "tag" {
        return Err(contract(format!("release tag {tag} must be annotated")));
    }
    let peeled = format!("{reference}^{{commit}}");
    let tag_commit = command_text(workspace, "git", &["rev-parse", &peeled])?;
    if tag_commit != commit {
        return Err(contract(format!(
            "release tag {tag} points to {tag_commit}, expected {commit}"
        )));
    }
    run(workspace, "git", &["verify-tag", tag])?;
    Ok(())
}

fn command_text(workspace: &Path, program: &str, args: &[&str]) -> Result<String> {
    let output = run(workspace, program, args)?;
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|_| contract(format!("{program} output was not valid UTF-8")))
}

fn run(workspace: &Path, program: &str, args: &[&str]) -> Result<Output> {
    let output = Command::new(program)
        .args(args)
        .current_dir(workspace)
        .output()
        .map_err(|source| Error::Spawn {
            program: program.to_owned(),
            source,
        })?;
    if !output.status.success() {
        return Err(Error::Command {
            program: format!("{program} {}", args.join(" ")),
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }
    Ok(output)
}
