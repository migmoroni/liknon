//! Declarative command-line contract and value mappings.

use super::help;
use clap::{Parser, Subcommand, ValueEnum};
use liknon_cli_ui::VisualOptions;
pub(super) use liknon_cli_ui::{PaletteArgument as ColorPaletteArgument, PresentationArgument};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "liknon-validator",
    version,
    about = help::root::ABOUT
)]
pub(super) struct Cli {
    #[command(flatten)]
    pub(super) visual: VisualOptions,

    #[command(subcommand)]
    pub(super) command: Command,
}

#[derive(Subcommand)]
pub(super) enum Command {
    #[command(about = help::initialization::ABOUT)]
    Init {
        #[arg(
            long,
            help = help::initialization::CONFIG,
            long_help = help::initialization::CONFIG_LONG
        )]
        config: Option<PathBuf>,

        #[arg(
            long,
            help = help::initialization::WORKSPACE,
            long_help = help::initialization::WORKSPACE_LONG
        )]
        workspace: Option<PathBuf>,

        #[arg(
            long,
            value_enum,
            default_value = "human",
            help = help::initialization::FORMAT,
            long_help = help::initialization::FORMAT_LONG
        )]
        format: Format,
    },

    #[command(
        about = help::execution::VALIDATE_ABOUT,
        long_about = help::execution::VALIDATE_LONG_ABOUT
    )]
    Validate {
        #[arg(help = help::execution::TARGET)]
        target: Option<String>,

        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,

        #[arg(
            long,
            value_enum,
            default_value = "human",
            help = help::execution::FORMAT,
            long_help = help::execution::FORMAT_LONG
        )]
        format: Format,
    },

    #[command(
        about = help::execution::CHECK_ABOUT,
        long_about = help::execution::CHECK_LONG_ABOUT
    )]
    Check {
        #[arg(help = help::execution::CHECK_ID)]
        check_id: String,

        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,

        #[arg(
            long,
            value_enum,
            default_value = "human",
            help = help::execution::FORMAT,
            long_help = help::execution::FORMAT_LONG
        )]
        format: Format,
    },

    #[command(about = help::inspection::CONFIG_ABOUT)]
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },

    #[command(about = help::inspection::LIST_ABOUT)]
    List {
        #[arg(
            long,
            help = help::inspection::TREE,
            long_help = help::inspection::TREE_LONG
        )]
        tree: bool,

        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,
    },

    #[command(about = help::inspection::EXPLAIN_ABOUT)]
    Explain {
        #[command(subcommand)]
        target: ExplainTarget,
    },

    #[command(about = help::inspection::SCHEMA_ABOUT)]
    Schema {
        #[arg(value_enum, help = help::inspection::CONTRACT)]
        contract: Contract,
    },
}

#[derive(Subcommand)]
pub(super) enum ConfigCommand {
    #[command(about = help::inspection::CONFIG_VALIDATE_ABOUT)]
    Validate {
        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub(super) enum ExplainTarget {
    #[command(about = help::inspection::GROUP_ABOUT)]
    Group {
        #[arg(help = help::inspection::GROUP_ID)]
        group_id: String,

        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,
    },

    #[command(about = help::inspection::SUITE_ABOUT)]
    Suite {
        #[arg(help = help::inspection::SUITE_ID)]
        suite_id: String,

        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,
    },

    #[command(about = help::inspection::CHECK_ABOUT)]
    Check {
        #[arg(help = help::inspection::CHECK_ID)]
        check_id: String,

        #[arg(long, help = help::CONFIG, long_help = help::CONFIG_LONG)]
        config: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub(super) enum Format {
    #[value(help = help::values::FORMAT_HUMAN)]
    Human,

    #[value(help = help::values::FORMAT_JSON)]
    Json,
}

#[derive(Clone, Copy, ValueEnum)]
pub(super) enum Contract {
    #[value(help = help::values::CONTRACT_CONFIG)]
    Config,

    #[value(help = help::values::CONTRACT_REPORT)]
    Report,
}
