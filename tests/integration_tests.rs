use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use rstest::*;
use std::fs;
use tempfile::tempdir;

mod common;

#[rstest]
fn test_example_commonmark_conversion(
    #[values(
        "01-minimal",
        "02-rule-metadata",
        "03-suppressions",
        "04-code-flows",
        "05-context-region",
        "06-baseline",
        "07-result-stacks",
        "08-embedded-content",
        "09-uri-base-ids",
        "10-default-config"
    )]
    example: &str,
) -> Result<()> {
    let input = format!("examples/sarif-files/{}.sarif", example);
    let temp_dir = tempdir()?;
    let output = temp_dir.path().join("output.md");

    Command::cargo_bin("sarif-to-md")?
        .arg("-i")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .assert()
        .success();

    let actual = fs::read_to_string(&output)?;

    if common::has_expected_output(example, "common-mark") {
        let expected = common::load_expected_output(example, "common-mark")?;
        common::assert_markdown_equivalent(&actual, &expected, &format!("{} CommonMark", example))?;
    } else {
        assert!(
            !actual.is_empty(),
            "Output should not be empty for {}",
            example
        );
        assert!(
            actual.contains("# Security Report") || actual.contains("#"),
            "Output should contain markdown headers for {}",
            example
        );
    }

    Ok(())
}

#[rstest]
fn test_example_github_conversion(
    #[values("01-minimal", "02-rule-metadata", "03-suppressions", "04-code-flows")] example: &str,
) -> Result<()> {
    let input = format!("examples/sarif-files/{}.sarif", example);
    let temp_dir = tempdir()?;
    let output = temp_dir.path().join("output.md");

    Command::cargo_bin("sarif-to-md")?
        .arg("-i")
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .arg("-f")
        .arg("github-flavored")
        .assert()
        .success();

    let actual = fs::read_to_string(&output)?;

    assert!(
        actual.contains("<details>") || actual.contains("<summary>"),
        "GitHub Flavored output should contain collapsible elements for {}",
        example
    );

    if common::has_expected_output(example, "github-flavored") {
        let expected = common::load_expected_output(example, "github-flavored")?;
        common::assert_markdown_equivalent(&actual, &expected, &format!("{} GitHub", example))?;
    }

    Ok(())
}

#[rstest]
#[case("common-mark", false)]
#[case("common-mark", true)]
#[case("github-flavored", false)]
#[case("github-flavored", true)]
fn test_format_emoji_combinations(#[case] format: &str, #[case] with_emoji: bool) -> Result<()> {
    let temp_dir = tempdir()?;
    let output = temp_dir.path().join("output.md");

    let mut cmd = Command::cargo_bin("sarif-to-md")?;
    cmd.arg("-i")
        .arg("examples/sarif-files/02-rule-metadata.sarif")
        .arg("-o")
        .arg(&output)
        .arg("-f")
        .arg(format);

    if with_emoji {
        cmd.arg("-e");
    }

    cmd.assert().success();

    let content = fs::read_to_string(&output)?;

    match format {
        "github-flavored" => {
            assert!(
                content.contains("<details>") || content.contains("<summary>"),
                "GitHub Flavored output should contain collapsible elements"
            );
        }
        "common-mark" => {
            assert!(
                !content.contains("<details>"),
                "CommonMark output should not contain HTML elements"
            );
        }
        _ => unreachable!(),
    }

    let has_emoji = content.chars().any(|c| c as u32 > 127);
    assert_eq!(
        has_emoji, with_emoji,
        "Emoji presence should match --with-emoji flag for format: {}",
        format
    );

    Ok(())
}

#[rstest]
fn test_stdout_output() -> Result<()> {
    let expected = common::load_expected_output("01-minimal", "common-mark")
        .unwrap_or_else(|_| "# Security Report".to_string());

    Command::cargo_bin("sarif-to-md")?
        .arg("-i")
        .arg("examples/sarif-files/01-minimal.sarif")
        .assert()
        .success()
        .stdout(predicate::str::contains("# Security Report") | predicate::str::contains("#"));

    Ok(())
}

#[rstest]
#[case("nonexistent.sarif", "No such file")]
#[case("examples/README.md", "Failed to parse")] // Wrong file type
fn test_error_handling(
    #[case] input_file: &str,
    #[case] expected_error_fragment: &str,
) -> Result<()> {
    Command::cargo_bin("sarif-to-md")?
        .arg("-i")
        .arg(input_file)
        .arg("-o")
        .arg("output.md")
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected_error_fragment));

    Ok(())
}

#[rstest]
#[case(&["--help"], "Usage")]
#[case(&["--version"], "sarif-to-md")]
fn test_cli_info_commands(#[case] args: &[&str], #[case] expected_output: &str) -> Result<()> {
    let mut cmd = Command::cargo_bin("sarif-to-md")?;

    for arg in args {
        cmd.arg(arg);
    }

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected_output));

    Ok(())
}

