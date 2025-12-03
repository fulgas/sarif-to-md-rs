use clap::{Parser, ValueEnum};
use sarif_to_md_core::markdown::MarkdownFormat;
use std::path::PathBuf;

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

#[derive(Parser)]
#[command(name = "sarif-to-md")]
#[command(about = "Convert Snyk security reports to Markdown format", version)]
pub(crate) struct Cli {
    /// Input JSON file path
    #[arg(short = 'i', long)]
    pub(crate) input: PathBuf,

    /// Output markdown file path (prints to stdout if not specified)
    #[arg(short = 'o', long)]
    pub(crate) output: Option<PathBuf>,

    /// Markdown output format
    #[arg(short = 'f', long, value_parser, default_value = "common-mark")]
    pub(crate) output_format: CliOutputFormat,

    /// Include emoji in the output
    #[arg(short = 'e', long, default_value = "false")]
    pub(crate) with_emoji: bool,
}
