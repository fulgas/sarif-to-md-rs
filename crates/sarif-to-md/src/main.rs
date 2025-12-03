use crate::cli::Cli;
use anyhow::Context;
use clap::Parser;
use sarif_to_md_core::generators::SarifMarkdownGenerator;
use sarif_to_md_core::markdown::MarkdownFormat;
use sarif_to_md_core::ReportProcessorBuilder;
use std::fs;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_path = &cli.input;
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Could not read file: {:?}", input_path))?;

    let with_emoji = cli.with_emoji;
    let output_format: MarkdownFormat = cli.output_format.into();

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(output_format, with_emoji))
        .content(content)
        .build()
        .context("Failed to configure report processor")?;

    let markdown = processor
        .generate()
        .context("Failed to generate markdown report")?;

    match &cli.output {
        Some(output_path) => {
            fs::write(output_path, markdown)?;
        }
        None => {
            println!("{}", markdown);
        }
    }
    Ok(())
}
