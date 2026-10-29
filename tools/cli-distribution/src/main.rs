mod cargo;
mod error;
mod hash;
mod io;
mod model;
mod operations;
mod schema;
mod selection;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{
    error::Result,
    io::{read_json, write_json},
    model::{
        Architecture, BuildManifest, Libc, OperatingSystem, ProductDescriptor, ReleaseIdentity,
    },
};

#[derive(Debug, Parser)]
#[command(about = "Private release-contract tooling for Liknon native CLIs")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Schemas {
        #[arg(long)]
        output_dir: PathBuf,
    },
    Schema {
        #[arg(value_enum)]
        contract: schema::Contract,
        #[arg(long)]
        output: PathBuf,
    },
    ValidateDescriptor(DescriptorArgs),
    Inspect {
        #[command(flatten)]
        contract: ContractArgs,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        release: bool,
    },
    NativeRecord {
        #[arg(long)]
        descriptor: PathBuf,
        #[arg(long)]
        target: String,
        #[arg(long)]
        payload_root: PathBuf,
        #[arg(long)]
        authenticity: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    BuildManifest {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        records_dir: PathBuf,
        #[arg(long)]
        release_toolchain: String,
        #[arg(long)]
        output: PathBuf,
    },
    VerifyBuild {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        build_manifest: PathBuf,
        #[arg(long)]
        payload_root: Option<PathBuf>,
    },
    AssembleArchives {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        build_manifest: PathBuf,
        #[arg(long)]
        payload_root: PathBuf,
        #[arg(long)]
        output_dir: PathBuf,
        #[arg(long)]
        source_date_epoch: u64,
    },
    RenderInstallers {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        base_url: String,
        #[arg(long)]
        public_key: PathBuf,
        #[arg(long)]
        posix_template: PathBuf,
        #[arg(long)]
        powershell_template: PathBuf,
        #[arg(long)]
        output_dir: PathBuf,
        #[arg(long)]
        allow_http: bool,
    },
    Checksums {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        root: PathBuf,
    },
    DirectManifest {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    VerifyDirect {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        public_key: Option<PathBuf>,
    },
    StablePointer {
        #[command(flatten)]
        files: ManifestFiles,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    SelectTarget {
        #[arg(long)]
        descriptor: PathBuf,
        #[arg(long, value_enum)]
        operating_system: OperatingSystem,
        #[arg(long, value_enum)]
        architecture: Architecture,
        #[arg(long, value_enum)]
        libc: Option<Libc>,
        #[arg(long)]
        gnu_detected: bool,
        #[arg(long)]
        musl_detected: bool,
        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Debug, Args)]
struct DescriptorArgs {
    #[arg(long)]
    descriptor: PathBuf,
}

#[derive(Debug, Args)]
struct ContractArgs {
    #[arg(long)]
    descriptor: PathBuf,
    #[arg(long, default_value = ".")]
    workspace: PathBuf,
}

#[derive(Debug, Args)]
struct ManifestFiles {
    #[arg(long)]
    descriptor: PathBuf,
    #[arg(long)]
    identity: PathBuf,
    #[arg(long, default_value = ".")]
    workspace: PathBuf,
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Schemas { output_dir } => schema::write_all(&output_dir),
        Command::Schema { contract, output } => schema::write_contract(contract, &output),
        Command::ValidateDescriptor(args) => {
            let descriptor: ProductDescriptor = read_json(&args.descriptor)?;
            descriptor.validate()
        }
        Command::Inspect {
            contract,
            output,
            release,
        } => {
            let descriptor: ProductDescriptor = read_json(&contract.descriptor)?;
            let identity = cargo::inspect(&descriptor, &contract.workspace, release)?;
            identity.validate_against(&descriptor)?;
            write_json(&output, &identity)
        }
        Command::NativeRecord {
            descriptor,
            target,
            payload_root,
            authenticity,
            output,
        } => {
            let descriptor = read_json(&descriptor)?;
            operations::native_record(&descriptor, &target, &payload_root, &authenticity, &output)
        }
        Command::BuildManifest {
            files,
            records_dir,
            release_toolchain,
            output,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            operations::build_manifest(
                &descriptor,
                &identity,
                &records_dir,
                release_toolchain,
                &output,
            )
        }
        Command::VerifyBuild {
            files,
            build_manifest,
            payload_root,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            let manifest: BuildManifest = read_json(&build_manifest)?;
            operations::verify_build(&descriptor, &identity, &manifest, payload_root.as_deref())
        }
        Command::AssembleArchives {
            files,
            build_manifest,
            payload_root,
            output_dir,
            source_date_epoch,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            let manifest = read_json(&build_manifest)?;
            operations::assemble_archives(
                &descriptor,
                &identity,
                &manifest,
                &payload_root,
                &files.workspace,
                &output_dir,
                source_date_epoch,
            )
        }
        Command::RenderInstallers {
            files,
            base_url,
            public_key,
            posix_template,
            powershell_template,
            output_dir,
            allow_http,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            operations::render_installers(operations::InstallerRender {
                descriptor: &descriptor,
                identity: &identity,
                base_url: &base_url,
                public_key: &public_key,
                posix_template: &posix_template,
                powershell_template: &powershell_template,
                output_dir: &output_dir,
                allow_http,
            })
        }
        Command::Checksums { files, root } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            operations::write_checksums(&descriptor, &identity, &root)
        }
        Command::DirectManifest {
            files,
            root,
            output,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            operations::direct_manifest(&descriptor, &identity, &root, &output)
        }
        Command::VerifyDirect {
            files,
            manifest,
            root,
            public_key,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            operations::verify_direct(
                &descriptor,
                &identity,
                &manifest,
                &root,
                public_key.as_deref(),
            )
        }
        Command::StablePointer {
            files,
            manifest,
            output,
        } => {
            let (descriptor, identity) = load_manifest_files(&files)?;
            operations::stable_pointer(&descriptor, &identity, &manifest, &output)
        }
        Command::SelectTarget {
            descriptor,
            operating_system,
            architecture,
            libc,
            gnu_detected,
            musl_detected,
            output,
        } => {
            let descriptor: ProductDescriptor = read_json(&descriptor)?;
            let selection = selection::select(
                &descriptor,
                operating_system,
                architecture,
                libc,
                gnu_detected,
                musl_detected,
            )?;
            write_json(&output, &selection)
        }
    }
}

fn load_manifest_files(files: &ManifestFiles) -> Result<(ProductDescriptor, ReleaseIdentity)> {
    let descriptor: ProductDescriptor = read_json(&files.descriptor)?;
    descriptor.validate()?;
    let identity: ReleaseIdentity = read_json(&files.identity)?;
    identity.validate_against(&descriptor)?;
    Ok((descriptor, identity))
}
