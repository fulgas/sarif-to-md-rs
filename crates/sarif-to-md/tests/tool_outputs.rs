//! Renders real SARIF written by security tools (examples/tools/sarif, regenerated with
//! examples/tools/generate.sh) and compares it with the checked-in expected Markdown.
//! Every `*.sarif` file there is picked up, so adding a tool only needs its SARIF and
//! its two expected outputs.

use anyhow::{Context, Result};
use assert_cmd::Command;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;

#[allow(dead_code)]
mod common;

#[rstest]
fn test_tool_sarif_renders_as_expected(
    #[files("../../examples/tools/sarif/*.sarif")] sarif: PathBuf,
    #[values("common-mark", "github-flavored")] format: &str,
) -> Result<()> {
    let tool = sarif
        .file_stem()
        .and_then(|s| s.to_str())
        .context("SARIF file name is not UTF-8")?;

    let mut cmd = Command::cargo_bin("sarif-to-md")?;
    cmd.arg("-i").arg(&sarif).arg("-f").arg(format);
    if format == "github-flavored" {
        // The github-flavored expected outputs are generated with emoji.
        cmd.arg("-e");
    }
    let output = cmd.assert().success().get_output().stdout.clone();
    let actual = String::from_utf8(output)?;

    let expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/tools/expected-outputs")
        .join(format)
        .join(format!("{tool}.md"));
    let expected = fs::read_to_string(&expected_path)
        .with_context(|| format!("missing expected output {}", expected_path.display()))?;

    common::assert_markdown_equivalent(&actual, &expected, &format!("{tool} {format}"))
}

/// Every `</details>` must close a `<details>` opened earlier, or a report embedded in
/// another collapsed section (a job summary, a PR comment) closes that section early.
fn assert_details_balanced(sarif: &PathBuf) -> Result<()> {
    let output = Command::cargo_bin("sarif-to-md")?
        .arg("-i")
        .arg(sarif)
        .arg("-f")
        .arg("github-flavored")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let markdown = String::from_utf8(output)?;

    let mut open = 0usize;
    for line in markdown.lines() {
        match line.trim() {
            "<details>" => open += 1,
            "</details>" => {
                open = open
                    .checked_sub(1)
                    .with_context(|| format!("unmatched </details> in {}", sarif.display()))?;
            }
            _ => {}
        }
    }
    anyhow::ensure!(
        open == 0,
        "{open} unclosed <details> in {}",
        sarif.display()
    );
    Ok(())
}

#[rstest]
fn test_details_balanced_for_tool_sarif(
    #[files("../../examples/tools/sarif/*.sarif")] sarif: PathBuf,
) -> Result<()> {
    assert_details_balanced(&sarif)
}

#[rstest]
fn test_details_balanced_for_spec_examples(
    #[files("../../examples/sarif-files/*.sarif")] sarif: PathBuf,
) -> Result<()> {
    assert_details_balanced(&sarif)
}
