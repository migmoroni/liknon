use std::collections::{BTreeMap, BTreeSet};

use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::{contract, Result},
    hash::is_sha256,
    io::{safe_identifier, safe_relative_path},
};

pub(crate) const SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ProductDescriptor {
    pub(crate) schema_version: u32,
    pub(crate) product: ProductIdentity,
    pub(crate) distribution: DistributionIdentity,
    pub(crate) targets: Vec<TargetDescriptor>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ProductIdentity {
    pub(crate) id: String,
    pub(crate) owner_plan: String,
    pub(crate) cargo_package: String,
    pub(crate) cargo_manifest: String,
    pub(crate) canonical_command: String,
    #[schemars(length(min = 1, max = 1))]
    pub(crate) aliases: Vec<String>,
    pub(crate) tag_prefix: String,
    pub(crate) release_rust_toolchain: String,
    pub(crate) repository: String,
    pub(crate) license_files: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DistributionIdentity {
    pub(crate) crates_registry: String,
    pub(crate) release_prefix: String,
    pub(crate) stable_pointer: String,
    pub(crate) manifest_signing_key_id: String,
    pub(crate) environments: PublicationEnvironments,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PublicationEnvironments {
    pub(crate) approval: String,
    pub(crate) crates: String,
    pub(crate) detached_signing: String,
    pub(crate) windows_signing: String,
    pub(crate) github: String,
    pub(crate) cloudflare: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct TargetDescriptor {
    pub(crate) rust_target: String,
    pub(crate) runner: String,
    pub(crate) operating_system: OperatingSystem,
    pub(crate) architecture: Architecture,
    pub(crate) libc: Option<Libc>,
    pub(crate) archive_format: ArchiveFormat,
    pub(crate) executable_suffix: String,
    pub(crate) authenticity: AuthenticityMode,
    pub(crate) runtime_baseline: String,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Serialize,
    JsonSchema,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub(crate) enum OperatingSystem {
    Linux,
    Macos,
    Windows,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Serialize,
    JsonSchema,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub(crate) enum Architecture {
    X86_64,
    Aarch64,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Serialize,
    JsonSchema,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub(crate) enum Libc {
    Gnu,
    Musl,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ArchiveFormat {
    TarGz,
    Zip,
}

impl ArchiveFormat {
    pub(crate) fn extension(self) -> &'static str {
        match self {
            Self::TarGz => "tar.gz",
            Self::Zip => "zip",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum AuthenticityMode {
    MetadataSignature,
    AdHoc,
    SelfSignedAuthenticode,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AuthenticityEvidence {
    pub(crate) mode: AuthenticityMode,
    pub(crate) verified: bool,
    pub(crate) method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) certificate_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) timestamped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) notarized: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ReleaseIdentity {
    pub(crate) schema_version: u32,
    pub(crate) product: String,
    pub(crate) package: String,
    pub(crate) package_path: String,
    pub(crate) version: String,
    pub(crate) tag: String,
    pub(crate) commit: String,
    pub(crate) cargo_lock_sha256: String,
    pub(crate) binary_targets: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct NativeArtifact {
    pub(crate) target: String,
    pub(crate) payload_file: String,
    pub(crate) payload_path: String,
    pub(crate) size: u64,
    pub(crate) sha256: String,
    pub(crate) installed_commands: Vec<String>,
    pub(crate) consumer: String,
    pub(crate) platform_authenticity: AuthenticityEvidence,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct BuildManifest {
    pub(crate) schema_version: u32,
    pub(crate) identity: ReleaseIdentity,
    pub(crate) release_toolchain: String,
    pub(crate) artifacts: Vec<NativeArtifact>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct FileRecord {
    pub(crate) path: String,
    pub(crate) size: u64,
    pub(crate) sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct InstalledCommandRecord {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) size: u64,
    pub(crate) sha256: String,
    pub(crate) equals_canonical: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DirectArtifact {
    pub(crate) target: String,
    pub(crate) format: ArchiveFormat,
    pub(crate) archive: FileRecord,
    pub(crate) commands: Vec<InstalledCommandRecord>,
    pub(crate) platform_authenticity: AuthenticityEvidence,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct InstallerRecord {
    pub(crate) platform: InstallerPlatform,
    pub(crate) file: FileRecord,
    pub(crate) signature_path: String,
    pub(crate) signature_algorithm: String,
    pub(crate) signing_key_id: String,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum InstallerPlatform {
    Posix,
    Powershell,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ManifestSignature {
    pub(crate) path: String,
    pub(crate) algorithm: String,
    pub(crate) signing_key_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DirectManifest {
    pub(crate) schema_version: u32,
    pub(crate) identity: ReleaseIdentity,
    pub(crate) artifacts: Vec<DirectArtifact>,
    pub(crate) installers: Vec<InstallerRecord>,
    pub(crate) checksums: FileRecord,
    pub(crate) signature: ManifestSignature,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StablePointer {
    pub(crate) schema_version: u32,
    pub(crate) product: String,
    pub(crate) version: String,
    pub(crate) tag: String,
    pub(crate) commit: String,
    pub(crate) manifest_path: String,
    pub(crate) manifest_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum PublicationStatus {
    Pending,
    Published,
    Verified,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PublicationObjectState {
    pub(crate) status: PublicationStatus,
    pub(crate) sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) detail: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PublicationState {
    pub(crate) schema_version: u32,
    pub(crate) identity: ReleaseIdentity,
    pub(crate) destinations: BTreeMap<String, PublicationObjectState>,
}

impl ProductDescriptor {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(contract(format!(
                "unsupported product descriptor schema version {}",
                self.schema_version
            )));
        }
        safe_identifier(&self.product.id, "product.id")?;
        safe_identifier(&self.product.cargo_package, "product.cargoPackage")?;
        safe_identifier(&self.product.canonical_command, "product.canonicalCommand")?;
        safe_relative_path(&self.product.cargo_manifest, "product.cargoManifest")?;
        if self.product.owner_plan.trim().is_empty() {
            return Err(contract("product.ownerPlan must not be empty"));
        }
        if self.product.repository.split('/').count() != 2
            || self.product.repository.contains(char::is_whitespace)
        {
            return Err(contract(
                "product.repository must have canonical owner/repository form",
            ));
        }
        if !self.product.tag_prefix.ends_with("-v")
            || !self
                .product
                .tag_prefix
                .starts_with(&self.product.cargo_package)
        {
            return Err(contract(
                "product.tagPrefix must be product-scoped and end in -v",
            ));
        }
        let release_toolchain = semver::Version::parse(&self.product.release_rust_toolchain)
            .map_err(|error| contract(format!("invalid release Rust toolchain: {error}")))?;
        if !release_toolchain.pre.is_empty()
            || !release_toolchain.build.is_empty()
            || release_toolchain.to_string() != self.product.release_rust_toolchain
        {
            return Err(contract(
                "product.releaseRustToolchain must use canonical MAJOR.MINOR.PATCH form",
            ));
        }
        if self.product.aliases.len() != 1 {
            return Err(contract(
                "product.aliases must contain exactly one compact command alias",
            ));
        }
        let mut commands = BTreeSet::from([self.product.canonical_command.as_str()]);
        for alias in &self.product.aliases {
            safe_identifier(alias, "product.aliases")?;
            if !commands.insert(alias) {
                return Err(contract("installed command names must be unique"));
            }
        }
        if self.product.license_files.is_empty() {
            return Err(contract("product.licenseFiles must not be empty"));
        }
        for license in &self.product.license_files {
            safe_relative_path(license, "product.licenseFiles")?;
        }
        if self.distribution.crates_registry != "crates-io" {
            return Err(contract(
                "distribution.cratesRegistry must be crates-io for this foundation version",
            ));
        }
        let expected_release_prefix = format!("releases/{}", self.product.cargo_package);
        if self.distribution.release_prefix != expected_release_prefix {
            return Err(contract(format!(
                "distribution.releasePrefix must be {expected_release_prefix}"
            )));
        }
        let expected_stable = format!("channels/{}/stable.json", self.product.cargo_package);
        if self.distribution.stable_pointer != expected_stable {
            return Err(contract(format!(
                "distribution.stablePointer must be {expected_stable}"
            )));
        }
        safe_identifier(
            &self.distribution.manifest_signing_key_id,
            "distribution.manifestSigningKeyId",
        )?;
        self.validate_environments()?;
        self.validate_targets()?;
        Ok(())
    }

    fn validate_environments(&self) -> Result<()> {
        let environments = &self.distribution.environments;
        let values = [
            environments.approval.as_str(),
            environments.crates.as_str(),
            environments.detached_signing.as_str(),
            environments.windows_signing.as_str(),
            environments.github.as_str(),
            environments.cloudflare.as_str(),
        ];
        let mut unique = BTreeSet::new();
        for value in values {
            safe_identifier(value, "distribution.environments")?;
            if !unique.insert(value) {
                return Err(contract("protected environment names must be unique"));
            }
        }
        Ok(())
    }

    fn validate_targets(&self) -> Result<()> {
        if self.targets.is_empty() {
            return Err(contract("targets must not be empty"));
        }
        let mut targets = BTreeSet::new();
        let mut coordinates = BTreeSet::new();
        for target in &self.targets {
            if !targets.insert(target.rust_target.as_str()) {
                return Err(contract(format!(
                    "duplicate Rust target {}",
                    target.rust_target
                )));
            }
            if !coordinates.insert((target.operating_system, target.architecture, target.libc)) {
                return Err(contract(format!(
                    "duplicate platform coordinates for {}",
                    target.rust_target
                )));
            }
            if target.runner.trim().is_empty() || target.runner.contains(char::is_whitespace) {
                return Err(contract(format!(
                    "target {} has an invalid runner",
                    target.rust_target
                )));
            }
            if target.runtime_baseline.trim().is_empty() {
                return Err(contract(format!(
                    "target {} must declare a runtime baseline",
                    target.rust_target
                )));
            }
            validate_target_shape(target)?;
        }
        Ok(())
    }

    pub(crate) fn target(&self, rust_target: &str) -> Result<&TargetDescriptor> {
        self.targets
            .iter()
            .find(|candidate| candidate.rust_target == rust_target)
            .ok_or_else(|| contract(format!("target {rust_target} is not declared")))
    }

    pub(crate) fn installed_commands(&self, target: &TargetDescriptor) -> Vec<String> {
        std::iter::once(&self.product.canonical_command)
            .chain(self.product.aliases.iter())
            .map(|command| format!("{command}{}", target.executable_suffix))
            .collect()
    }
}

fn validate_target_shape(target: &TargetDescriptor) -> Result<()> {
    let expected = match (target.operating_system, target.architecture, target.libc) {
        (OperatingSystem::Linux, Architecture::X86_64, Some(Libc::Gnu)) => {
            "x86_64-unknown-linux-gnu"
        }
        (OperatingSystem::Linux, Architecture::X86_64, Some(Libc::Musl)) => {
            "x86_64-unknown-linux-musl"
        }
        (OperatingSystem::Linux, Architecture::Aarch64, Some(Libc::Gnu)) => {
            "aarch64-unknown-linux-gnu"
        }
        (OperatingSystem::Linux, Architecture::Aarch64, Some(Libc::Musl)) => {
            "aarch64-unknown-linux-musl"
        }
        (OperatingSystem::Macos, Architecture::X86_64, None) => "x86_64-apple-darwin",
        (OperatingSystem::Macos, Architecture::Aarch64, None) => "aarch64-apple-darwin",
        (OperatingSystem::Windows, Architecture::X86_64, None) => "x86_64-pc-windows-msvc",
        _ => {
            return Err(contract(format!(
                "unsupported platform coordinates for {}",
                target.rust_target
            )))
        }
    };
    if target.rust_target != expected {
        return Err(contract(format!(
            "target coordinates require Rust target {expected}, found {}",
            target.rust_target
        )));
    }
    let (format, suffix, authenticity) = match target.operating_system {
        OperatingSystem::Linux => (
            ArchiveFormat::TarGz,
            "",
            AuthenticityMode::MetadataSignature,
        ),
        OperatingSystem::Macos => (ArchiveFormat::TarGz, "", AuthenticityMode::AdHoc),
        OperatingSystem::Windows => (
            ArchiveFormat::Zip,
            ".exe",
            AuthenticityMode::SelfSignedAuthenticode,
        ),
    };
    if target.archive_format != format
        || target.executable_suffix != suffix
        || target.authenticity != authenticity
    {
        return Err(contract(format!(
            "target {} has inconsistent archive, suffix, or authenticity settings",
            target.rust_target
        )));
    }
    Ok(())
}

impl AuthenticityEvidence {
    pub(crate) fn validate_for(&self, target: &TargetDescriptor) -> Result<()> {
        if self.mode != target.authenticity || self.method.trim().is_empty() {
            return Err(contract(format!(
                "authenticity evidence does not match {}",
                target.rust_target
            )));
        }
        match self.mode {
            AuthenticityMode::MetadataSignature => {
                if self.certificate_fingerprint.is_some()
                    || self.timestamped.is_some()
                    || self.notarized.is_some()
                {
                    return Err(contract(
                        "Linux metadata evidence cannot claim native signing",
                    ));
                }
            }
            AuthenticityMode::AdHoc => {
                if !self.verified || self.notarized != Some(false) {
                    return Err(contract(
                        "macOS evidence must verify ad hoc signing and state notarized=false",
                    ));
                }
                if self.certificate_fingerprint.is_some() || self.timestamped.is_some() {
                    return Err(contract(
                        "macOS ad hoc evidence cannot claim a certificate or timestamp",
                    ));
                }
            }
            AuthenticityMode::SelfSignedAuthenticode => {
                if !self.verified
                    || self.timestamped != Some(true)
                    || self
                        .certificate_fingerprint
                        .as_deref()
                        .is_none_or(str::is_empty)
                {
                    return Err(contract(
                        "Windows evidence requires a verified fingerprint and RFC 3161 timestamp",
                    ));
                }
                if self.notarized.is_some() {
                    return Err(contract("Windows evidence cannot contain notarization"));
                }
            }
        }
        Ok(())
    }
}

impl ReleaseIdentity {
    pub(crate) fn validate_against(&self, descriptor: &ProductDescriptor) -> Result<()> {
        if self.schema_version != SCHEMA_VERSION
            || self.product != descriptor.product.id
            || self.package != descriptor.product.cargo_package
            || self.package_path != descriptor.product.cargo_manifest
            || self.tag != format!("{}{}", descriptor.product.tag_prefix, self.version)
            || !is_sha256(&self.cargo_lock_sha256)
            || self.commit.len() != 40
            || !self.commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(contract(
                "release identity does not match product descriptor",
            ));
        }
        let mut expected = std::iter::once(descriptor.product.canonical_command.clone())
            .chain(descriptor.product.aliases.iter().cloned())
            .collect::<Vec<_>>();
        expected.sort();
        let mut actual = self.binary_targets.clone();
        actual.sort();
        if actual != expected {
            return Err(contract(
                "release identity has an unexpected binary inventory",
            ));
        }
        Ok(())
    }
}

impl NativeArtifact {
    pub(crate) fn validate(
        &self,
        descriptor: &ProductDescriptor,
        root: Option<&std::path::Path>,
    ) -> Result<()> {
        let target = descriptor.target(&self.target)?;
        let expected_payload = format!(
            "{}{}",
            descriptor.product.canonical_command, target.executable_suffix
        );
        if self.payload_file != expected_payload
            || self.installed_commands != descriptor.installed_commands(target)
            || self.consumer != format!("direct:{}", self.target)
            || !is_sha256(&self.sha256)
            || self.size == 0
        {
            return Err(contract(format!(
                "native artifact record for {} violates its target contract",
                self.target
            )));
        }
        safe_relative_path(&self.payload_path, "artifacts.payloadPath")?;
        self.platform_authenticity.validate_for(target)?;
        if let Some(root) = root {
            let path = root.join(&self.payload_path);
            let (size, sha256) = crate::hash::sha256(&path)?;
            if size != self.size || sha256 != self.sha256 {
                return Err(contract(format!(
                    "native payload {} does not match its record",
                    path.display()
                )));
            }
        }
        Ok(())
    }
}

impl BuildManifest {
    pub(crate) fn validate(
        &self,
        descriptor: &ProductDescriptor,
        identity: &ReleaseIdentity,
        root: Option<&std::path::Path>,
    ) -> Result<()> {
        if self.schema_version != SCHEMA_VERSION || &self.identity != identity {
            return Err(contract("build manifest identity mismatch"));
        }
        if self.release_toolchain.trim().is_empty() {
            return Err(contract(
                "build manifest releaseToolchain must not be empty",
            ));
        }
        let mut targets = BTreeSet::new();
        for artifact in &self.artifacts {
            if !targets.insert(artifact.target.as_str()) {
                return Err(contract(format!(
                    "duplicate build artifact for {}",
                    artifact.target
                )));
            }
            artifact.validate(descriptor, root)?;
        }
        let expected = descriptor
            .targets
            .iter()
            .map(|target| target.rust_target.as_str())
            .collect::<BTreeSet<_>>();
        if targets != expected {
            return Err(contract(
                "build manifest does not contain exactly one artifact per target",
            ));
        }
        Ok(())
    }
}

impl FileRecord {
    pub(crate) fn validate(&self, root: Option<&std::path::Path>) -> Result<()> {
        safe_relative_path(&self.path, "file.path")?;
        if self.size == 0 || !is_sha256(&self.sha256) {
            return Err(contract(format!("invalid file record for {}", self.path)));
        }
        if let Some(root) = root {
            let path = root.join(&self.path);
            let (size, sha256) = crate::hash::sha256(&path)?;
            if size != self.size || sha256 != self.sha256 {
                return Err(contract(format!(
                    "file {} does not match its record",
                    path.display()
                )));
            }
        }
        Ok(())
    }
}

impl DirectManifest {
    pub(crate) fn validate(
        &self,
        descriptor: &ProductDescriptor,
        identity: &ReleaseIdentity,
        root: Option<&std::path::Path>,
    ) -> Result<()> {
        if self.schema_version != SCHEMA_VERSION || &self.identity != identity {
            return Err(contract("direct manifest identity mismatch"));
        }
        let mut targets = BTreeSet::new();
        for artifact in &self.artifacts {
            let target = descriptor.target(&artifact.target)?;
            if !targets.insert(artifact.target.as_str()) || artifact.format != target.archive_format
            {
                return Err(contract(format!(
                    "invalid or duplicate direct artifact {}",
                    artifact.target
                )));
            }
            artifact.archive.validate(root)?;
            artifact.platform_authenticity.validate_for(target)?;
            let expected_commands = descriptor.installed_commands(target);
            let actual_commands = artifact
                .commands
                .iter()
                .map(|command| command.name.clone())
                .collect::<Vec<_>>();
            if actual_commands != expected_commands
                || artifact.commands.iter().any(|command| {
                    !command.equals_canonical
                        || command.size == 0
                        || !is_sha256(&command.sha256)
                        || safe_relative_path(&command.path, "commands.path").is_err()
                })
            {
                return Err(contract(format!(
                    "direct command inventory for {} is invalid",
                    artifact.target
                )));
            }
            let canonical_hash = &artifact.commands[0].sha256;
            if artifact
                .commands
                .iter()
                .any(|command| &command.sha256 != canonical_hash)
            {
                return Err(contract(format!(
                    "direct aliases for {} are not byte-identical",
                    artifact.target
                )));
            }
        }
        let expected_targets = descriptor
            .targets
            .iter()
            .map(|target| target.rust_target.as_str())
            .collect::<BTreeSet<_>>();
        if targets != expected_targets {
            return Err(contract("direct manifest target inventory is incomplete"));
        }
        let platforms = self
            .installers
            .iter()
            .map(|installer| installer.platform)
            .collect::<BTreeSet<_>>();
        if platforms != BTreeSet::from([InstallerPlatform::Posix, InstallerPlatform::Powershell]) {
            return Err(contract(
                "direct manifest requires POSIX and PowerShell installers",
            ));
        }
        for installer in &self.installers {
            installer.file.validate(root)?;
            safe_relative_path(&installer.signature_path, "installer.signaturePath")?;
            if installer.signature_algorithm != "rsa-sha256"
                || installer.signing_key_id != descriptor.distribution.manifest_signing_key_id
            {
                return Err(contract("installer signature contract mismatch"));
            }
        }
        self.checksums.validate(root)?;
        safe_relative_path(&self.signature.path, "signature.path")?;
        if self.signature.algorithm != "rsa-sha256"
            || self.signature.signing_key_id != descriptor.distribution.manifest_signing_key_id
        {
            return Err(contract("manifest signature contract mismatch"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> ProductDescriptor {
        ProductDescriptor {
            schema_version: 1,
            product: ProductIdentity {
                id: "example".into(),
                owner_plan: "TEST".into(),
                cargo_package: "example-cli".into(),
                cargo_manifest: "apps/example/Cargo.toml".into(),
                canonical_command: "example-cli".into(),
                aliases: vec!["exc".into()],
                tag_prefix: "example-cli-v".into(),
                release_rust_toolchain: "1.87.0".into(),
                repository: "owner/repository".into(),
                license_files: vec!["LICENSE".into()],
            },
            distribution: DistributionIdentity {
                crates_registry: "crates-io".into(),
                release_prefix: "releases/example-cli".into(),
                stable_pointer: "channels/example-cli/stable.json".into(),
                manifest_signing_key_id: "example-release-v1".into(),
                environments: PublicationEnvironments {
                    approval: "example-approval".into(),
                    crates: "example-crates".into(),
                    detached_signing: "example-signing".into(),
                    windows_signing: "example-windows".into(),
                    github: "example-github".into(),
                    cloudflare: "example-cloudflare".into(),
                },
            },
            targets: vec![TargetDescriptor {
                rust_target: "x86_64-unknown-linux-gnu".into(),
                runner: "ubuntu-24.04".into(),
                operating_system: OperatingSystem::Linux,
                architecture: Architecture::X86_64,
                libc: Some(Libc::Gnu),
                archive_format: ArchiveFormat::TarGz,
                executable_suffix: String::new(),
                authenticity: AuthenticityMode::MetadataSignature,
                runtime_baseline: "glibc 2.35".into(),
            }],
        }
    }

    #[test]
    fn accepts_unrelated_product_descriptor() {
        descriptor().validate().unwrap();
    }

    #[test]
    fn rejects_cross_product_storage_namespace() {
        let mut value = descriptor();
        value.distribution.release_prefix = "releases/liknon-validator".into();
        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_more_than_one_compact_alias() {
        let mut value = descriptor();
        value.product.aliases.push("example".into());
        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_linux_native_signing_claim() {
        let descriptor = descriptor();
        let evidence = AuthenticityEvidence {
            mode: AuthenticityMode::MetadataSignature,
            verified: true,
            method: "manifest".into(),
            certificate_fingerprint: Some("invented".into()),
            timestamped: None,
            notarized: None,
        };
        assert!(evidence.validate_for(&descriptor.targets[0]).is_err());
    }
}
