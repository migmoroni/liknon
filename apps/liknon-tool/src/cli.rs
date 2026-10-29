//! Command grammar and Tool-specific argument applicability.

use std::path::PathBuf;

use clap::{Arg, Args, Parser, Subcommand, ValueEnum};
use liknon_cli_ui::VisualOptions;
use liknon_resource_contracts::{ResourceKind, TrustClass};

/// Maximum number of catalog or search rows accepted from one invocation.
pub(crate) const MAX_RESULT_LIMIT: usize = 1_000;
/// Maximum number of files accepted in one context envelope.
pub(crate) const MAX_CONTEXT_FILES: usize = 1_000;
/// Maximum number of payload bytes accepted in one context envelope.
pub(crate) const MAX_CONTEXT_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
/// Serialization format selected for a Tool response.
pub(crate) enum OutputFormat {
    /// Accessible, terminal-oriented output.
    Human,
    /// Stable machine-readable JSON output.
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
/// Representation used when returning one payload.
pub(crate) enum ContentEncoding {
    /// Require UTF-8 textual content and preserve its exact characters.
    Text,
    /// Return arbitrary payload bytes as standard padded Base64.
    Base64,
}

#[derive(Debug, Clone, Args)]
/// Workspace boundary shared by local resource operations.
pub(crate) struct WorkspaceArgs {
    /// Uses this exact consumer workspace boundary instead of upward discovery.
    #[arg(long, value_name = "PATH")]
    pub(crate) workspace: Option<PathBuf>,
}

#[derive(Debug, Clone, Args)]
/// Metadata filters shared by catalog and search operations.
pub(crate) struct CatalogFilters {
    /// Restricts results to one resource kind.
    #[arg(long)]
    pub(crate) kind: Option<ResourceKind>,
    /// Restricts results to one publisher namespace.
    #[arg(long)]
    pub(crate) namespace: Option<String>,
    /// Restricts results to one explicit trust class.
    #[arg(long)]
    pub(crate) trust: Option<TrustClass>,
    /// Bounds the number of returned rows after deterministic ordering.
    #[arg(long, default_value_t = 100, value_name = "COUNT")]
    pub(crate) limit: usize,
}

#[derive(Debug, Parser)]
#[command(
    name = "liknon-tool",
    version,
    about = "Discovers and returns installed Liknon resources offline",
    arg_required_else_help = true,
    disable_help_subcommand = true
)]
/// Parsed Tool invocation.
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) visual: VisualOptions,
    /// Selects human-readable or versioned machine-readable output.
    #[arg(long, global = true, default_value = "human", value_name = "FORMAT")]
    format: OutputFormat,
    #[command(subcommand)]
    pub(crate) command: Command,
}

impl Cli {
    /// Returns the requested response format.
    pub(crate) const fn format(&self) -> OutputFormat {
        self.format
    }

    /// Returns the shared visual options applied to human output.
    pub(crate) const fn visual(&self) -> VisualOptions {
        self.visual
    }
}

#[derive(Debug, Subcommand)]
/// Read-only operations exposed by the offline resource Tool.
pub(crate) enum Command {
    /// Lists bounded routing metadata for installed resources.
    Catalog {
        #[command(flatten)]
        filters: CatalogFilters,
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
    /// Searches installed resource metadata without loading payload content.
    Search {
        /// Case-insensitive identity, title, summary, author, or capability text.
        query: String,
        #[command(flatten)]
        filters: CatalogFilters,
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
    /// Returns one installed resource or one exact declared payload.
    Show {
        /// Exact resource identity using <kind>:<namespace>/<name>.
        resource: String,
        /// Exact declared payload path; required when the resource has multiple files.
        #[arg(long, value_name = "PATH")]
        file: Option<String>,
        /// Explicit payload encoding; binary content requires base64.
        #[arg(long, value_enum, default_value = "text")]
        encoding: ContentEncoding,
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
    /// Returns a stable topological closure from exact locked dependency edges.
    Resolve {
        /// Exact root identities whose locked closure is requested.
        #[arg(required = true, value_name = "RESOURCE_ID")]
        resources: Vec<String>,
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
    /// Builds a bounded envelope from explicitly selected installed resources.
    Context {
        /// Exact resource identities selected for the envelope.
        #[arg(required = true, value_name = "RESOURCE_ID")]
        resources: Vec<String>,
        /// Includes locked dependencies; collection roots always include their members.
        #[arg(long)]
        include_dependencies: bool,
        /// Maximum payload files represented in the envelope.
        #[arg(long, default_value_t = 64, value_name = "COUNT")]
        max_files: usize,
        /// Maximum aggregate declared payload bytes represented in the envelope.
        #[arg(long, default_value_t = 1024 * 1024, value_name = "BYTES")]
        max_bytes: u64,
        #[command(flatten)]
        workspace: WorkspaceArgs,
    },
}

/// Applies Tool-owned execution applicability to shared visual arguments.
pub(crate) fn configure(command: clap::Command) -> clap::Command {
    let visual_arguments = ["color", "presentation"].map(|id| {
        command
            .get_arguments()
            .find(|argument| argument.get_id() == id)
            .unwrap_or_else(|| panic!("root visual argument `{id}` is undefined"))
            .clone()
    });

    command.mut_subcommands(|subcommand| {
        visual_arguments
            .iter()
            .cloned()
            .fold(subcommand, |subcommand, argument| {
                subcommand.arg(execution_argument(argument))
            })
    })
}

fn execution_argument(argument: Arg) -> Arg {
    match argument.get_id().as_str() {
        "color" => argument
            .help("Styles human Tool output with ANSI color")
            .long_help("Styles human Tool output with ANSI color. See root --help for palettes and rendering behavior.")
            .hide_possible_values(true),
        "presentation" => argument
            .help("Selects the layout and emphasis of human Tool output")
            .long_help("Selects the layout and emphasis of human Tool output. See root --help for presentation modes and rendering behavior.")
            .hide_possible_values(true),
        id => panic!("unsupported visual argument `{id}`"),
    }
}
