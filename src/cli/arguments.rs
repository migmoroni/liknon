//! Declarative command-line contract and value mappings.

use super::help;
use crate::theme::{PaletteProfile, PresentationProfile};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "workspace-validator",
    version,
    about = help::root::ABOUT
)]
pub(super) struct Cli {
    #[arg(
        long,
        global = true,
        value_enum,
        num_args = 0..=1,
        default_missing_value = "standard",
        require_equals = true,
        value_name = "PALETTE",
        help = help::root::COLOR,
        long_help = help::root::COLOR_LONG
    )]
    pub(super) color: Option<ColorPaletteArgument>,

    #[arg(
        long,
        global = true,
        value_enum,
        require_equals = true,
        value_name = "MODE",
        help = help::root::PRESENTATION,
        long_help = help::root::PRESENTATION_LONG
    )]
    pub(super) presentation: Option<PresentationArgument>,

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

    #[command(about = help::knowledge::ABOUT)]
    Knowledge {
        #[command(subcommand)]
        command: KnowledgeCommand,
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
pub(super) enum KnowledgeCommand {
    #[command(about = help::knowledge::CATALOG_ABOUT)]
    Catalog {
        #[arg(
            long,
            value_enum,
            default_value = "human",
            help = help::knowledge::CATALOG_FORMAT,
            long_help = help::knowledge::CATALOG_FORMAT_LONG
        )]
        format: Format,
    },

    #[command(about = help::knowledge::SHOW_ABOUT)]
    Show {
        #[arg(help = help::knowledge::DOCUMENT_ID)]
        document_id: String,
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
pub(super) enum ColorPaletteArgument {
    #[value(help = help::values::COLOR_STANDARD)]
    Standard,

    #[value(help = help::values::COLOR_HIGH_CONTRAST)]
    HighContrast,

    #[value(help = help::values::COLOR_PROTANOPIA)]
    Protanopia,

    #[value(help = help::values::COLOR_DEUTERANOPIA)]
    Deuteranopia,

    #[value(help = help::values::COLOR_TRITANOPIA)]
    Tritanopia,

    #[value(help = help::values::COLOR_ACHROMATOPSIA)]
    Achromatopsia,
}

/// Resolves the explicit palette early enough to style Clap's own output.
pub(super) fn requested_palette(arguments: &[std::ffi::OsString]) -> PaletteProfile {
    arguments
        .iter()
        .skip(1)
        .take_while(|argument| *argument != "--")
        .find_map(|argument| {
            let argument = argument.to_str()?;
            if argument == "--color" {
                return Some(ColorPaletteArgument::Standard.into());
            }
            let value = argument.strip_prefix("--color=")?;
            ColorPaletteArgument::from_str(value, true)
                .ok()
                .map(Into::into)
        })
        .unwrap_or(PaletteProfile::Plain)
}

/// Resolves the explicit presentation before Clap renders help or diagnostics.
pub(super) fn requested_presentation(arguments: &[std::ffi::OsString]) -> PresentationProfile {
    arguments
        .iter()
        .skip(1)
        .take_while(|argument| *argument != "--")
        .find_map(|argument| {
            let value = argument.to_str()?.strip_prefix("--presentation=")?;
            PresentationArgument::from_str(value, true)
                .ok()
                .map(Into::into)
        })
        .unwrap_or(PresentationProfile::Standard)
}

#[derive(Clone, Copy, ValueEnum)]
pub(super) enum PresentationArgument {
    #[value(help = help::values::PRESENTATION_STANDARD)]
    Standard,

    #[value(help = help::values::PRESENTATION_LOW_VISION)]
    LowVision,
}

impl From<ColorPaletteArgument> for PaletteProfile {
    fn from(value: ColorPaletteArgument) -> Self {
        match value {
            ColorPaletteArgument::Standard => Self::Standard,
            ColorPaletteArgument::HighContrast => Self::HighContrast,
            ColorPaletteArgument::Protanopia => Self::Protanopia,
            ColorPaletteArgument::Deuteranopia => Self::Deuteranopia,
            ColorPaletteArgument::Tritanopia => Self::Tritanopia,
            ColorPaletteArgument::Achromatopsia => Self::Achromatopsia,
        }
    }
}

impl From<PresentationArgument> for PresentationProfile {
    fn from(value: PresentationArgument) -> Self {
        match value {
            PresentationArgument::Standard => Self::Standard,
            PresentationArgument::LowVision => Self::LowVision,
        }
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub(super) enum Contract {
    #[value(help = help::values::CONTRACT_CONFIG)]
    Config,

    #[value(help = help::values::CONTRACT_REPORT)]
    Report,
}
