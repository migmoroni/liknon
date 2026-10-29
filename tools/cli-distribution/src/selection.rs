use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    error::{contract, Result},
    model::{Architecture, Libc, OperatingSystem, ProductDescriptor},
};

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Selection {
    operating_system: OperatingSystem,
    architecture: Architecture,
    detected_libcs: Vec<Libc>,
    decision_source: DecisionSource,
    selected_target: String,
}

#[derive(Clone, Copy, Debug, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
enum DecisionSource {
    Explicit,
    AutomaticGnuPreferred,
    AutomaticSingle,
    Platform,
}

pub(crate) fn select(
    descriptor: &ProductDescriptor,
    operating_system: OperatingSystem,
    architecture: Architecture,
    requested_libc: Option<Libc>,
    gnu_detected: bool,
    musl_detected: bool,
) -> Result<Selection> {
    descriptor.validate()?;
    let mut detected = Vec::new();
    if gnu_detected {
        detected.push(Libc::Gnu);
    }
    if musl_detected {
        detected.push(Libc::Musl);
    }
    let (libc, decision_source) = match operating_system {
        OperatingSystem::Linux => {
            let selected = if let Some(requested) = requested_libc {
                if (requested == Libc::Gnu && !gnu_detected)
                    || (requested == Libc::Musl && !musl_detected)
                {
                    return Err(contract(format!(
                        "requested Linux libc {requested:?} was not detected"
                    )));
                }
                (requested, DecisionSource::Explicit)
            } else if gnu_detected {
                (
                    Libc::Gnu,
                    if musl_detected {
                        DecisionSource::AutomaticGnuPreferred
                    } else {
                        DecisionSource::AutomaticSingle
                    },
                )
            } else if musl_detected {
                (Libc::Musl, DecisionSource::AutomaticSingle)
            } else {
                return Err(contract(
                    "neither a compatible GNU libc nor MUSL implementation was detected",
                ));
            };
            (Some(selected.0), selected.1)
        }
        OperatingSystem::Macos | OperatingSystem::Windows => {
            if requested_libc.is_some() || gnu_detected || musl_detected {
                return Err(contract("libc selection applies only to Linux"));
            }
            (None, DecisionSource::Platform)
        }
    };
    let target = descriptor
        .targets
        .iter()
        .find(|target| {
            target.operating_system == operating_system
                && target.architecture == architecture
                && target.libc == libc
        })
        .ok_or_else(|| contract("selected host coordinates are not supported"))?;
    Ok(Selection {
        operating_system,
        architecture,
        detected_libcs: detected,
        decision_source,
        selected_target: target.rust_target.clone(),
    })
}

#[cfg(test)]
mod tests {
    use crate::model::{
        ArchiveFormat, AuthenticityMode, DistributionIdentity, ProductIdentity,
        PublicationEnvironments, TargetDescriptor, SCHEMA_VERSION,
    };

    use super::*;

    fn descriptor() -> ProductDescriptor {
        let targets = [Libc::Gnu, Libc::Musl]
            .into_iter()
            .map(|libc| TargetDescriptor {
                rust_target: match libc {
                    Libc::Gnu => "x86_64-unknown-linux-gnu",
                    Libc::Musl => "x86_64-unknown-linux-musl",
                }
                .into(),
                runner: "ubuntu-24.04".into(),
                operating_system: OperatingSystem::Linux,
                architecture: Architecture::X86_64,
                libc: Some(libc),
                archive_format: ArchiveFormat::TarGz,
                executable_suffix: String::new(),
                authenticity: AuthenticityMode::MetadataSignature,
                runtime_baseline: "test".into(),
            })
            .collect();
        ProductDescriptor {
            schema_version: SCHEMA_VERSION,
            product: ProductIdentity {
                id: "example".into(),
                owner_plan: "TEST".into(),
                cargo_package: "example-cli".into(),
                cargo_manifest: "apps/example/Cargo.toml".into(),
                commands: vec!["example-cli".into(), "example-helper".into()],
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
                    stable: "example-stable".into(),
                },
            },
            targets,
        }
    }

    #[test]
    fn automatic_selection_prefers_gnu_when_both_are_detected() {
        let selected = select(
            &descriptor(),
            OperatingSystem::Linux,
            Architecture::X86_64,
            None,
            true,
            true,
        )
        .unwrap();
        assert_eq!(selected.selected_target, "x86_64-unknown-linux-gnu");
    }

    #[test]
    fn explicit_selection_requires_detection() {
        assert!(select(
            &descriptor(),
            OperatingSystem::Linux,
            Architecture::X86_64,
            Some(Libc::Musl),
            true,
            false,
        )
        .is_err());
    }
}
