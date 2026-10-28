use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsStr,
    fs,
    path::Path,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    error::{contract, Error, Result},
    hash::sha256,
    io::{read_json, safe_relative_path, write_bytes, write_json},
    model::{
        BuildManifest, DirectArtifact, DirectManifest, FileRecord, InstalledCommandRecord,
        InstallerPlatform, InstallerRecord, ManifestSignature, NativeArtifact, ProductDescriptor,
        ReleaseIdentity, StablePointer, SCHEMA_VERSION,
    },
};

pub(crate) fn native_record(
    descriptor: &ProductDescriptor,
    target_name: &str,
    payload: &Path,
    payload_path: &str,
    authenticity_path: &Path,
    output: &Path,
) -> Result<()> {
    descriptor.validate()?;
    let target = descriptor.target(target_name)?;
    let authenticity = read_json(authenticity_path)?;
    let (size, digest) = sha256(payload)?;
    let payload_file = payload
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| contract("payload filename is not valid UTF-8"))?
        .to_owned();
    let artifact = NativeArtifact {
        target: target_name.to_owned(),
        payload_file,
        payload_path: payload_path.to_owned(),
        size,
        sha256: digest,
        installed_commands: descriptor.installed_commands(target),
        consumer: format!("direct:{target_name}"),
        platform_authenticity: authenticity,
    };
    artifact.validate(descriptor, None)?;
    write_json(output, &artifact)
}

pub(crate) fn build_manifest(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    records_dir: &Path,
    release_toolchain: String,
    output: &Path,
) -> Result<()> {
    descriptor.validate()?;
    identity.validate_against(descriptor)?;
    let artifacts = read_records::<NativeArtifact>(records_dir)?;
    let manifest = BuildManifest {
        schema_version: SCHEMA_VERSION,
        identity: identity.clone(),
        release_toolchain,
        artifacts,
    };
    manifest.validate(descriptor, identity, None)?;
    write_json(output, &manifest)
}

pub(crate) fn verify_build(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    manifest: &BuildManifest,
    payload_root: Option<&Path>,
) -> Result<()> {
    descriptor.validate()?;
    identity.validate_against(descriptor)?;
    manifest.validate(descriptor, identity, payload_root)
}

pub(crate) fn assemble_archives(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    manifest: &BuildManifest,
    payload_root: &Path,
    repository_root: &Path,
    output_dir: &Path,
    source_date_epoch: u64,
) -> Result<()> {
    verify_build(descriptor, identity, manifest, Some(payload_root))?;
    fs::create_dir_all(output_dir).map_err(|source| Error::Write {
        path: output_dir.to_path_buf(),
        source,
    })?;
    let records_dir = output_dir.join(".records");
    fs::create_dir_all(&records_dir).map_err(|source| Error::Write {
        path: records_dir.clone(),
        source,
    })?;
    for artifact in &manifest.artifacts {
        let target = descriptor.target(&artifact.target)?;
        let staging = output_dir.join(format!(".stage-{}", artifact.target));
        if staging.exists() {
            fs::remove_dir_all(&staging).map_err(|source| Error::Write {
                path: staging.clone(),
                source,
            })?;
        }
        fs::create_dir_all(&staging).map_err(|source| Error::Write {
            path: staging.clone(),
            source,
        })?;
        let payload = payload_root.join(&artifact.payload_path);
        let commands = descriptor.installed_commands(target);
        for command in &commands {
            let destination = staging.join(command);
            fs::copy(&payload, &destination).map_err(|source| Error::Write {
                path: destination.clone(),
                source,
            })?;
            set_executable(&destination)?;
        }
        for license in &descriptor.product.license_files {
            let source = repository_root.join(license);
            let file_name = Path::new(license)
                .file_name()
                .ok_or_else(|| contract(format!("license path {license} has no filename")))?;
            let destination = staging.join(file_name);
            fs::copy(&source, &destination).map_err(|source| Error::Write {
                path: destination,
                source,
            })?;
        }
        normalize_timestamps(&staging, source_date_epoch)?;
        let archive_name = format!(
            "{}{}-{}.{}",
            descriptor.product.tag_prefix,
            identity.version,
            artifact.target,
            target.archive_format.extension()
        );
        let archive_path = output_dir.join(&archive_name);
        create_archive(
            target.archive_format,
            &staging,
            &archive_path,
            source_date_epoch,
        )?;
        let (archive_size, archive_sha256) = sha256(&archive_path)?;
        let canonical_digest = sha256(&staging.join(&commands[0]))?;
        let command_records = commands
            .iter()
            .map(|command| {
                let (size, digest) = sha256(&staging.join(command))?;
                Ok(InstalledCommandRecord {
                    name: command.clone(),
                    path: command.clone(),
                    size,
                    equals_canonical: (size, digest.as_str())
                        == (canonical_digest.0, canonical_digest.1.as_str()),
                    sha256: digest,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let record = DirectArtifact {
            target: artifact.target.clone(),
            format: target.archive_format,
            archive: FileRecord {
                path: archive_name,
                size: archive_size,
                sha256: archive_sha256,
            },
            commands: command_records,
            platform_authenticity: artifact.platform_authenticity.clone(),
        };
        write_json(
            &records_dir.join(format!("{}.json", artifact.target)),
            &record,
        )?;
        fs::remove_dir_all(&staging).map_err(|source| Error::Write {
            path: staging,
            source,
        })?;
    }
    Ok(())
}

pub(crate) struct InstallerRender<'a> {
    pub(crate) descriptor: &'a ProductDescriptor,
    pub(crate) identity: &'a ReleaseIdentity,
    pub(crate) base_url: &'a str,
    pub(crate) public_key: &'a Path,
    pub(crate) posix_template: &'a Path,
    pub(crate) powershell_template: &'a Path,
    pub(crate) output_dir: &'a Path,
    pub(crate) allow_http: bool,
}

pub(crate) fn render_installers(input: InstallerRender<'_>) -> Result<()> {
    input.descriptor.validate()?;
    input.identity.validate_against(input.descriptor)?;
    if !(input.base_url.starts_with("https://")
        || (input.allow_http && input.base_url.starts_with("http://")))
    {
        return Err(contract("installer base URL must use HTTPS"));
    }
    let public_key = fs::read_to_string(input.public_key).map_err(|source| Error::Read {
        path: input.public_key.to_path_buf(),
        source,
    })?;
    if !public_key.contains("BEGIN PUBLIC KEY") || !public_key.contains("END PUBLIC KEY") {
        return Err(contract(
            "detached-signature trust root is not a PEM public key",
        ));
    }
    fs::create_dir_all(input.output_dir).map_err(|source| Error::Write {
        path: input.output_dir.to_path_buf(),
        source,
    })?;
    let replacements = [
        ("@@PRODUCT@@", input.descriptor.product.id.as_str()),
        (
            "@@PACKAGE@@",
            input.descriptor.product.cargo_package.as_str(),
        ),
        ("@@VERSION@@", input.identity.version.as_str()),
        ("@@TAG@@", input.identity.tag.as_str()),
        ("@@COMMIT@@", input.identity.commit.as_str()),
        ("@@BASE_URL@@", input.base_url.trim_end_matches('/')),
        (
            "@@RELEASE_PREFIX@@",
            input.descriptor.distribution.release_prefix.as_str(),
        ),
        (
            "@@SIGNING_KEY_ID@@",
            input
                .descriptor
                .distribution
                .manifest_signing_key_id
                .as_str(),
        ),
        (
            "@@CANONICAL_COMMAND@@",
            input.descriptor.product.canonical_command.as_str(),
        ),
        (
            "@@ALIAS_COMMAND@@",
            input.descriptor.product.aliases[0].as_str(),
        ),
        ("@@PUBLIC_KEY_PEM@@", public_key.trim()),
    ];
    render_template(
        input.posix_template,
        &input.output_dir.join("install.sh"),
        &replacements,
    )?;
    render_template(
        input.powershell_template,
        &input.output_dir.join("install.ps1"),
        &replacements,
    )?;
    set_executable(&input.output_dir.join("install.sh"))?;
    Ok(())
}

pub(crate) fn write_checksums(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    root: &Path,
) -> Result<()> {
    descriptor.validate()?;
    identity.validate_against(descriptor)?;
    let mut paths = descriptor
        .targets
        .iter()
        .map(|target| {
            format!(
                "{}{}-{}.{}",
                descriptor.product.tag_prefix,
                identity.version,
                target.rust_target,
                target.archive_format.extension()
            )
        })
        .collect::<Vec<_>>();
    paths.extend(["install.ps1".to_owned(), "install.sh".to_owned()]);
    paths.sort();
    let mut content = String::new();
    for relative in paths {
        safe_relative_path(&relative, "checksum path")?;
        let (_, digest) = sha256(&root.join(&relative))?;
        content.push_str(&format!("{digest}  {relative}\n"));
    }
    write_bytes(&root.join("SHA256SUMS"), content.as_bytes())
}

pub(crate) fn direct_manifest(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    root: &Path,
    output: &Path,
) -> Result<()> {
    descriptor.validate()?;
    identity.validate_against(descriptor)?;
    let artifacts = read_records::<DirectArtifact>(&root.join(".records"))?;
    let installers = [
        (InstallerPlatform::Posix, "install.sh"),
        (InstallerPlatform::Powershell, "install.ps1"),
    ]
    .into_iter()
    .map(|(platform, path)| {
        Ok(InstallerRecord {
            platform,
            file: file_record(root, path)?,
            signature_path: format!("{path}.sig"),
            signature_algorithm: "rsa-sha256".into(),
            signing_key_id: descriptor.distribution.manifest_signing_key_id.clone(),
        })
    })
    .collect::<Result<Vec<_>>>()?;
    let manifest = DirectManifest {
        schema_version: SCHEMA_VERSION,
        identity: identity.clone(),
        artifacts,
        installers,
        checksums: file_record(root, "SHA256SUMS")?,
        signature: ManifestSignature {
            path: "manifest.sig".into(),
            algorithm: "rsa-sha256".into(),
            signing_key_id: descriptor.distribution.manifest_signing_key_id.clone(),
        },
    };
    manifest.validate(descriptor, identity, Some(root))?;
    write_json(output, &manifest)
}

pub(crate) fn verify_direct(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    manifest_path: &Path,
    root: &Path,
    public_key: Option<&Path>,
) -> Result<()> {
    let manifest: DirectManifest = read_json(manifest_path)?;
    manifest.validate(descriptor, identity, Some(root))?;
    verify_checksum_projection(&manifest, root)?;
    if let Some(public_key) = public_key {
        verify_signature(
            public_key,
            &root.join(&manifest.signature.path),
            manifest_path,
        )?;
        for installer in &manifest.installers {
            verify_signature(
                public_key,
                &root.join(&installer.signature_path),
                &root.join(&installer.file.path),
            )?;
        }
    }
    for artifact in &manifest.artifacts {
        verify_archive(descriptor, artifact, root)?;
    }
    Ok(())
}

pub(crate) fn stable_pointer(
    descriptor: &ProductDescriptor,
    identity: &ReleaseIdentity,
    manifest_path: &Path,
    output: &Path,
) -> Result<()> {
    descriptor.validate()?;
    identity.validate_against(descriptor)?;
    let (_, manifest_sha256) = sha256(manifest_path)?;
    let pointer = StablePointer {
        schema_version: SCHEMA_VERSION,
        product: identity.product.clone(),
        version: identity.version.clone(),
        tag: identity.tag.clone(),
        commit: identity.commit.clone(),
        manifest_path: format!(
            "{}/v{}/manifest.json",
            descriptor.distribution.release_prefix, identity.version
        ),
        manifest_sha256,
    };
    write_json(output, &pointer)
}

fn read_records<T: serde::de::DeserializeOwned>(directory: &Path) -> Result<Vec<T>> {
    let entries = fs::read_dir(directory).map_err(|source| Error::Read {
        path: directory.to_path_buf(),
        source,
    })?;
    let mut paths = entries
        .map(|entry| {
            entry
                .map(|entry| entry.path())
                .map_err(|source| Error::Read {
                    path: directory.to_path_buf(),
                    source,
                })
        })
        .collect::<Result<Vec<_>>>()?;
    paths.retain(|path| path.extension() == Some(OsStr::new("json")));
    paths.sort();
    paths
        .iter()
        .map(|path| read_json(path))
        .collect::<Result<Vec<_>>>()
}

fn render_template(path: &Path, output: &Path, replacements: &[(&str, &str)]) -> Result<()> {
    let mut template = fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_path_buf(),
        source,
    })?;
    for (marker, value) in replacements {
        template = template.replace(marker, value);
    }
    if template.contains("@@") {
        return Err(contract(format!(
            "template {} contains an unresolved marker",
            path.display()
        )));
    }
    write_bytes(output, template.as_bytes())
}

fn file_record(root: &Path, relative: &str) -> Result<FileRecord> {
    safe_relative_path(relative, "file path")?;
    let (size, sha256) = sha256(&root.join(relative))?;
    Ok(FileRecord {
        path: relative.to_owned(),
        size,
        sha256,
    })
}

fn normalize_timestamps(staging: &Path, source_date_epoch: u64) -> Result<()> {
    let epoch = format!("@{source_date_epoch}");
    let mut entries = fs::read_dir(staging)
        .map_err(|source| Error::Read {
            path: staging.to_path_buf(),
            source,
        })?
        .map(|entry| {
            entry
                .map(|entry| entry.path())
                .map_err(|source| Error::Read {
                    path: staging.to_path_buf(),
                    source,
                })
        })
        .collect::<Result<Vec<_>>>()?;
    entries.sort();
    for entry in entries {
        run_command(Command::new("touch").arg("-d").arg(&epoch).arg(entry))?;
    }
    Ok(())
}

fn create_archive(
    format: crate::model::ArchiveFormat,
    staging: &Path,
    output: &Path,
    source_date_epoch: u64,
) -> Result<()> {
    let mut entries = fs::read_dir(staging)
        .map_err(|source| Error::Read {
            path: staging.to_path_buf(),
            source,
        })?
        .map(|entry| {
            entry
                .map(|entry| entry.file_name())
                .map_err(|source| Error::Read {
                    path: staging.to_path_buf(),
                    source,
                })
        })
        .collect::<Result<Vec<_>>>()?;
    entries.sort();
    match format {
        crate::model::ArchiveFormat::TarGz => {
            let mut command = Command::new("tar");
            command
                .current_dir(staging)
                .arg("--sort=name")
                .arg(format!("--mtime=@{source_date_epoch}"))
                .arg("--owner=0")
                .arg("--group=0")
                .arg("--numeric-owner")
                .arg("--format=gnu")
                .arg("-czf")
                .arg(output)
                .arg("--");
            command.args(entries);
            run_command(&mut command)?;
        }
        crate::model::ArchiveFormat::Zip => {
            let mut command = Command::new("zip");
            command
                .current_dir(staging)
                .arg("-X")
                .arg("-q")
                .arg(output)
                .arg("--");
            command.args(entries);
            run_command(&mut command)?;
        }
    }
    Ok(())
}

fn verify_checksum_projection(manifest: &DirectManifest, root: &Path) -> Result<()> {
    let content =
        fs::read_to_string(root.join(&manifest.checksums.path)).map_err(|source| Error::Read {
            path: root.join(&manifest.checksums.path),
            source,
        })?;
    let mut declared = BTreeMap::new();
    for line in content.lines() {
        let (digest, path) = line
            .split_once("  ")
            .ok_or_else(|| contract("SHA256SUMS contains a malformed line"))?;
        if declared.insert(path, digest).is_some() {
            return Err(contract("SHA256SUMS contains a duplicate path"));
        }
    }
    let expected = manifest
        .artifacts
        .iter()
        .map(|artifact| &artifact.archive)
        .chain(manifest.installers.iter().map(|installer| &installer.file))
        .map(|file| (file.path.as_str(), file.sha256.as_str()))
        .collect::<BTreeMap<_, _>>();
    if declared != expected {
        return Err(contract(
            "SHA256SUMS does not match the signed manifest projection",
        ));
    }
    Ok(())
}

fn verify_signature(public_key: &Path, signature: &Path, content: &Path) -> Result<()> {
    run_command(
        Command::new("openssl")
            .arg("dgst")
            .arg("-sha256")
            .arg("-verify")
            .arg(public_key)
            .arg("-signature")
            .arg(signature)
            .arg(content),
    )?;
    Ok(())
}

fn verify_archive(
    descriptor: &ProductDescriptor,
    artifact: &DirectArtifact,
    root: &Path,
) -> Result<()> {
    let target = descriptor.target(&artifact.target)?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| contract("system clock is before the Unix epoch"))?
        .as_nanos();
    let extraction = std::env::temp_dir().join(format!(
        "liknon-direct-verify-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&extraction).map_err(|source| Error::Write {
        path: extraction.clone(),
        source,
    })?;
    let archive = root.join(&artifact.archive.path);
    let result = match target.archive_format {
        crate::model::ArchiveFormat::TarGz => run_command(
            Command::new("tar")
                .arg("-xzf")
                .arg(&archive)
                .arg("-C")
                .arg(&extraction),
        ),
        crate::model::ArchiveFormat::Zip => run_command(
            Command::new("unzip")
                .arg("-q")
                .arg(&archive)
                .arg("-d")
                .arg(&extraction),
        ),
    };
    if let Err(error) = result {
        let _ = fs::remove_dir_all(&extraction);
        return Err(error);
    }
    let mut actual = fs::read_dir(&extraction)
        .map_err(|source| Error::Read {
            path: extraction.clone(),
            source,
        })?
        .map(|entry| {
            entry
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .map_err(|source| Error::Read {
                    path: extraction.clone(),
                    source,
                })
        })
        .collect::<Result<BTreeSet<_>>>()?;
    let mut expected = descriptor
        .installed_commands(target)
        .into_iter()
        .collect::<BTreeSet<_>>();
    for license in &descriptor.product.license_files {
        let name = Path::new(license)
            .file_name()
            .ok_or_else(|| contract("license path has no filename"))?
            .to_string_lossy()
            .into_owned();
        expected.insert(name);
    }
    if actual != expected {
        let _ = fs::remove_dir_all(&extraction);
        return Err(contract(format!(
            "archive {} has an unexpected inventory",
            artifact.archive.path
        )));
    }
    for command in &artifact.commands {
        actual.remove(&command.name);
        let (size, digest) = sha256(&extraction.join(&command.path))?;
        if size != command.size || digest != command.sha256 {
            let _ = fs::remove_dir_all(&extraction);
            return Err(contract(format!(
                "extracted command {} does not match the manifest",
                command.name
            )));
        }
    }
    fs::remove_dir_all(&extraction).map_err(|source| Error::Write {
        path: extraction,
        source,
    })?;
    Ok(())
}

fn run_command(command: &mut Command) -> Result<Output> {
    let program = command.get_program().to_string_lossy().into_owned();
    let rendered = std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(|part| part.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");
    let output = command.output().map_err(|source| Error::Spawn {
        program: program.clone(),
        source,
    })?;
    if !output.status.success() {
        return Err(Error::Command {
            program: rendered,
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        });
    }
    Ok(output)
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .map_err(|source| Error::Read {
            path: path.to_path_buf(),
            source,
        })?
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).map_err(|source| Error::Write {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> Result<()> {
    Ok(())
}
