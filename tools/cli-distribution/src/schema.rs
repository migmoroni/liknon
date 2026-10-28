use std::path::Path;

use clap::ValueEnum;
use schemars::{schema_for, JsonSchema};
use serde::Serialize;

use crate::{
    error::Result,
    io::write_json,
    model::{
        BuildManifest, DirectManifest, ProductDescriptor, PublicationState, ReleaseIdentity,
        StablePointer,
    },
    selection::Selection,
};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum Contract {
    Product,
    ReleaseIdentity,
    BuildManifest,
    DirectManifest,
    StablePointer,
    PublicationState,
    TargetSelection,
}

impl Contract {
    pub(crate) fn file_name(self) -> &'static str {
        match self {
            Self::Product => "product.schema.json",
            Self::ReleaseIdentity => "release-identity.schema.json",
            Self::BuildManifest => "build-manifest.schema.json",
            Self::DirectManifest => "direct-manifest.schema.json",
            Self::StablePointer => "stable-pointer.schema.json",
            Self::PublicationState => "publication-state.schema.json",
            Self::TargetSelection => "target-selection.schema.json",
        }
    }
}

pub(crate) fn write_contract(contract: Contract, output: &Path) -> Result<()> {
    match contract {
        Contract::Product => write_schema::<ProductDescriptor>(output),
        Contract::ReleaseIdentity => write_schema::<ReleaseIdentity>(output),
        Contract::BuildManifest => write_schema::<BuildManifest>(output),
        Contract::DirectManifest => write_schema::<DirectManifest>(output),
        Contract::StablePointer => write_schema::<StablePointer>(output),
        Contract::PublicationState => write_schema::<PublicationState>(output),
        Contract::TargetSelection => write_schema::<Selection>(output),
    }
}

pub(crate) fn write_all(output_dir: &Path) -> Result<()> {
    for contract in [
        Contract::Product,
        Contract::ReleaseIdentity,
        Contract::BuildManifest,
        Contract::DirectManifest,
        Contract::StablePointer,
        Contract::PublicationState,
        Contract::TargetSelection,
    ] {
        write_contract(contract, &output_dir.join(contract.file_name()))?;
    }
    Ok(())
}

fn write_schema<T: JsonSchema + Serialize>(output: &Path) -> Result<()> {
    write_json(output, &schema_for!(T))
}
