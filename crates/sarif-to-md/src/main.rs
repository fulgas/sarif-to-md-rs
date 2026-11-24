use crate::cli::Cli;
use anyhow::Context;
use clap::Parser;
use sarif_to_md_core::generators::SarifMarkdownGenerator;
use sarif_to_md_core::markdown::MarkdownFormat;
use sarif_to_md_core::ReportProcessorBuilder;
use std::env;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let input_path = &cli.input;
    let content = fs::read_to_string(input_path)
        .with_context(|| format!("Could not read file: {:?}", input_path))?;

    let with_emoji = cli.with_emoji;
    let output_format: MarkdownFormat = cli.output_format.into();

    let mut generator = SarifMarkdownGenerator::new(output_format, with_emoji)
        .include_suppressed(cli.include_suppressed);
    if let Some(time) = source_date_epoch()? {
        generator = generator.generated_at(time);
    }
    let processor = ReportProcessorBuilder::new()
        .generator(generator)
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

/// Reads `SOURCE_DATE_EPOCH` (<https://reproducible-builds.org/specs/source-date-epoch/>):
/// an integer number of seconds since the Unix epoch, as printed by `date +%s`. An empty
/// value counts as unset; a malformed one is an error, as the specification asks.
fn source_date_epoch() -> anyhow::Result<Option<SystemTime>> {
    let Some(value) = env::var_os("SOURCE_DATE_EPOCH") else {
        return Ok(None);
    };
    if value.is_empty() {
        return Ok(None);
    }
    let seconds: i64 = value
        .to_str()
        .and_then(|v| v.parse().ok())
        .with_context(|| {
            format!("SOURCE_DATE_EPOCH must be an integer number of seconds, got {value:?}")
        })?;
    let offset = Duration::from_secs(seconds.unsigned_abs());
    let time = if seconds >= 0 {
        UNIX_EPOCH.checked_add(offset)
    } else {
        UNIX_EPOCH.checked_sub(offset)
    };
    time.map(Some)
        .with_context(|| format!("SOURCE_DATE_EPOCH is out of range: {seconds}"))
}
