use crate::cli::Cli;
use anyhow::Context;
use clap::Parser;
use snyk_to_md_core::ReportProcessorBuilder;
use std::fs;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_path = cli.get_input();
    let json_content = fs::read_to_string(input_path)
        .with_context(|| format!("Could not read file: {:?}", input_path))?;

    let mut builder = ReportProcessorBuilder::new()
        .parser_format(cli.get_parser_format())
        .markdown_format(cli.get_output_format())
        .with_emoji(cli.get_with_emoji())
        .content(&json_content);

    if let Some(parser_type) = cli.get_parser_type() {
        builder = builder.parser_type(parser_type);
    }

    let markdown_report = builder
        .build()
        .context("Failed to configure the report processor")?
        .generate()
        .context("Failed to generate the markdown report")?;

    match cli.get_output() {
        Some(output_path) => {
            fs::write(output_path, markdown_report).with_context(|| {
                format!("Failed to write report to file: {}", output_path.display())
            })?;
        }
        None => {
            println!("{}", markdown_report);
        }
    }
    Ok(())
}
