use crate::cli::Cli;
use anyhow::Context;
use clap::Parser;
use snyk_to_md_core::ReportProcessorBuilder;
use std::fs;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let json_content = fs::read_to_string(&cli.input)
        .with_context(|| format!("Could not read file: {:?}", cli.input))?;

    let markdown_report = ReportProcessorBuilder::new()
        .parser_type(cli.command.into())
        .parser_format(cli.input_format.into())
        .markdown_format(cli.output_format.into())
        .with_emoji(cli.with_emoji)
        .content(&json_content)
        .build()
        .context("Failed to configure the report processor")?
        .generate()
        .context("Failed to generate the markdown report")?;

    match cli.output {
        Some(output_path) => {
            fs::write(&output_path, markdown_report).with_context(|| {
                format!("Failed to write report to file: {}", output_path.display())
            })?;
        }
        None => {
            println!("{}", markdown_report);
        }
    }
    Ok(())
}
