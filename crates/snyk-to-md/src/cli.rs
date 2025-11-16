use clap::{Parser, Subcommand, ValueEnum};
use snyk_to_md_core::markdown::MarkdownFormat;
use snyk_to_md_core::parser::{ParserFormat, ParserType};
use std::path::PathBuf;

#[derive(Debug, Clone, Subcommand, ValueEnum)]
pub(crate) enum CliJsonReportType {
    Container,
    Code,
}

impl From<CliJsonReportType> for ParserType {
    fn from(cli_type: CliJsonReportType) -> Self {
        match cli_type {
            CliJsonReportType::Container => ParserType::Container,
            CliJsonReportType::Code => ParserType::Code,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum CliOutputFormat {
    #[value(name = "github-flavored")]
    GitHubFlavored,
    #[value(name = "common-mark")]
    CommonMark,
}

impl From<CliOutputFormat> for MarkdownFormat {
    fn from(cli_format: CliOutputFormat) -> Self {
        match cli_format {
            CliOutputFormat::GitHubFlavored => MarkdownFormat::GitHubFlavored,
            CliOutputFormat::CommonMark => MarkdownFormat::CommonMark,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub(crate) enum CliInputFormat {
    #[value(name = "json")]
    Json,
    #[value(name = "sarif")]
    Sarif,
}

impl From<CliInputFormat> for ParserFormat {
    fn from(cli_type: CliInputFormat) -> Self {
        match cli_type {
            CliInputFormat::Json => ParserFormat::Json,
            CliInputFormat::Sarif => ParserFormat::Sarif,
        }
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Process Snyk JSON reports (container or code)
    Json {
        /// Type of JSON report to process
        #[arg(value_enum)]
        report_type: CliJsonReportType,

        /// Input JSON file path
        #[arg(short = 'i', long)]
        input: PathBuf,

        /// Output markdown file path (prints to stdout if not specified)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Markdown output format
        #[arg(short = 'f', long, value_parser, default_value = "common-mark")]
        output_format: CliOutputFormat,

        /// Include emoji in the output
        #[arg(short = 'e', long, default_value = "false")]
        with_emoji: bool,
    },
    /// Process SARIF reports
    Sarif {
        /// Input SARIF file path
        #[arg(short = 'i', long)]
        input: PathBuf,

        /// Output markdown file path (prints to stdout if not specified)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Markdown output format
        #[arg(short = 'f', long, value_parser, default_value = "common-mark")]
        output_format: CliOutputFormat,

        /// Include emoji in the output
        #[arg(short = 'e', long, default_value = "false")]
        with_emoji: bool,
    },
}

#[derive(Parser)]
#[command(name = "snyk-to-md")]
#[command(about = "Convert Snyk security reports to Markdown format", version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

impl Cli {
    pub(crate) fn get_parser_format(&self) -> ParserFormat {
        match &self.command {
            Commands::Json { .. } => ParserFormat::Json,
            Commands::Sarif { .. } => ParserFormat::Sarif,
        }
    }

    pub(crate) fn get_parser_type(&self) -> Option<ParserType> {
        match &self.command {
            Commands::Json { report_type, .. } => Some(report_type.clone().into()),
            Commands::Sarif { .. } => Some(ParserType::Container),
        }
    }

    pub(crate) fn get_input(&self) -> &PathBuf {
        match &self.command {
            Commands::Json { input, .. } => input,
            Commands::Sarif { input, .. } => input,
        }
    }

    pub(crate) fn get_output(&self) -> Option<&PathBuf> {
        match &self.command {
            Commands::Json { output, .. } => output.as_ref(),
            Commands::Sarif { output, .. } => output.as_ref(),
        }
    }

    pub(crate) fn get_output_format(&self) -> MarkdownFormat {
        match &self.command {
            Commands::Json { output_format, .. } => output_format.clone().into(),
            Commands::Sarif { output_format, .. } => output_format.clone().into(),
        }
    }

    pub(crate) fn get_with_emoji(&self) -> bool {
        match &self.command {
            Commands::Json { with_emoji, .. } => *with_emoji,
            Commands::Sarif { with_emoji, .. } => *with_emoji,
        }
    }
}
