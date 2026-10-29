use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use liknon_resource_contracts::{ResourceKind, TrustClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum OutputFormat {
    Human,
    Json,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct WorkspaceArgs {
    /// Uses this exact consumer workspace boundary instead of discovery.
    #[arg(long, value_name = "PATH")]
    pub(crate) workspace: Option<PathBuf>,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct HubArgs {
    /// Authenticated HTTPS Hub origin, file:// URL, or local test path.
    #[arg(long, env = "LIKNON_HUB_URL", value_name = "ORIGIN")]
    pub(crate) hub: Option<String>,
    /// Separately obtained public Hub trust-root JSON.
    #[arg(long, env = "LIKNON_HUB_TRUST_ROOT", value_name = "PATH")]
    pub(crate) trust_root: Option<PathBuf>,
    /// Explicit operating-system cache location.
    #[arg(long, env = "LIKNON_CACHE_DIR", value_name = "PATH")]
    pub(crate) cache: Option<PathBuf>,
    /// Complete request timeout in seconds.
    #[arg(long, default_value_t = 30, value_name = "SECONDS", value_parser = clap::value_parser!(u64).range(1..=300))]
    pub(crate) timeout: u64,
}

#[derive(Debug, Clone, Args)]
pub(crate) struct MutationArgs {
    /// Resolves and prints the complete plan without changing the workspace.
    #[arg(long)]
    pub(crate) dry_run: bool,
    /// Applies the complete plan without an interactive confirmation.
    #[arg(long)]
    pub(crate) yes: bool,
}

#[derive(Debug, Parser)]
#[command(
    name = "liknon-manager",
    version,
    about = "Discovers and transactionally manages authenticated Liknon resources",
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
pub(crate) struct Cli {
    /// Selects human-readable or versioned machine-readable output.
    #[arg(long, global = true, default_value = "human", value_name = "FORMAT")]
    format: OutputFormat,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    pub(crate) const fn format(&self) -> OutputFormat {
        self.format
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Creates an empty resource manifest and lock at one workspace boundary.
    Init {
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
    /// Searches one authenticated Hub catalog without downloading payloads.
    Search {
        /// Case-insensitive identity, title, or summary text.
        #[arg(default_value = "")]
        query: String,
        /// Restricts results to one resource kind.
        #[arg(long)]
        kind: Option<ResourceKind>,
        /// Restricts results to one publisher namespace.
        #[arg(long)]
        namespace: Option<String>,
        /// Restricts results to one explicit trust class.
        #[arg(long)]
        trust: Option<TrustClass>,
        #[command(flatten)]
        hub: HubArgs,
    },
    /// Shows administrative metadata for one available or installed resource.
    Info {
        /// Resource identity or identity with a semantic version requirement.
        resource: String,
        /// Reads only installed locked state and performs no network access.
        #[arg(long)]
        installed: bool,
        #[command(flatten)]
        workspace: WorkspaceArgs,
        #[command(flatten)]
        hub: HubArgs,
    },
    /// Resolves and installs explicit Hub or local resource selections.
    Install {
        /// Hub resource specs using <kind>:<namespace>/<name>[@<requirement>].
        #[arg(value_name = "RESOURCE")]
        resources: Vec<String>,
        /// Explicit local resource directory containing resource.json and payloads.
        #[arg(long = "local", value_name = "PATH")]
        local: Vec<PathBuf>,
        /// Allows resolution of community resources.
        #[arg(long)]
        allow_community: bool,
        #[command(flatten)]
        workspace: WorkspaceArgs,
        #[command(flatten)]
        hub: HubArgs,
        #[command(flatten)]
        mutation: MutationArgs,
    },
    /// Re-resolves all or selected desired resources under existing constraints.
    Update {
        /// Identities to update; omission updates every desired root.
        #[arg(value_name = "RESOURCE_ID")]
        resources: Vec<String>,
        /// Allows resolution of community resources.
        #[arg(long)]
        allow_community: bool,
        #[command(flatten)]
        workspace: WorkspaceArgs,
        #[command(flatten)]
        hub: HubArgs,
        #[command(flatten)]
        mutation: MutationArgs,
    },
    /// Removes desired roots when the remaining graph stays valid.
    Remove {
        /// Desired root identities to remove.
        #[arg(required = true, value_name = "RESOURCE_ID")]
        resources: Vec<String>,
        #[command(flatten)]
        workspace: WorkspaceArgs,
        #[command(flatten)]
        mutation: MutationArgs,
    },
    /// Compares desired, locked, installed, missing, and drifted resource state.
    List {
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
    /// Verifies schemas, metadata authentication, and exact installed bytes.
    Verify {
        /// Performs no Hub request and verifies only retained local evidence.
        #[arg(long)]
        offline: bool,
        #[command(flatten)]
        workspace: WorkspaceArgs,
        #[command(flatten)]
        hub: HubArgs,
    },
    /// Projects one locked payload to an explicit workspace-relative destination.
    Materialize {
        /// Exact installed resource identity.
        resource: String,
        /// Payload path declared by the locked resource.
        #[arg(long, value_name = "PATH")]
        payload: String,
        /// Workspace-relative destination path.
        #[arg(long, value_name = "PATH")]
        destination: PathBuf,
        #[command(flatten)]
        workspace: WorkspaceArgs,
        #[command(flatten)]
        mutation: MutationArgs,
    },
    /// Prints a public resource JSON Schema to standard output.
    Schema {
        /// Public contract whose schema is requested.
        #[arg(value_enum)]
        contract: SchemaContract,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub(crate) enum SchemaContract {
    Resource,
    Catalog,
    Manifest,
    Lock,
    TrustRoot,
    Signature,
}
