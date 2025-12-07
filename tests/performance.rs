use anyhow::Result;
use assert_cmd::Command;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_performance_smoke_test() -> Result<()> {
    let start = std::time::Instant::now();
    let temp_dir = tempdir()?;
    let output = temp_dir.path().join("perf-test.md");

    Command::cargo_bin("sarif-to-md")?
        .arg("-i")
        .arg("examples/sarif-files/08-embedded-content.sarif") // Largest example
        .arg("-o")
        .arg(&output)
        .assert()
        .success();

    let duration = start.elapsed();

    assert!(
        duration < Duration::from_secs(10),
        "Performance regression detected: conversion took {:?} (threshold: 10s)",
        duration
    );

    let output_size = std::fs::metadata(&output)?.len();
    assert!(output_size > 0, "Output file should not be empty");

    Ok(())
}

#[test]
fn test_resource_usage_smoke() -> Result<()> {
    let temp_dir = tempdir()?;

    for example in ["01-minimal", "02-rule-metadata", "04-code-flows"] {
        let input = format!("examples/sarif-files/{}.sarif", example);
        let output = temp_dir.path().join(format!("{}.md", example));

        Command::cargo_bin("sarif-to-md")?
            .arg("-i")
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .assert()
            .success();

        let output_size = std::fs::metadata(&output)?.len();
        assert!(output_size > 50, "Output too small for {}", example);
    }

    Ok(())
}

#[test]
fn test_format_performance_parity() -> Result<()> {
    let temp_dir = tempdir()?;
    let input = "examples/sarif-files/02-rule-metadata.sarif";

    for (format, threshold_ms) in [("common-mark", 5000), ("github-flavored", 5000)] {
        let start = std::time::Instant::now();
        let output = temp_dir.path().join(format!("{}.md", format));

        Command::cargo_bin("sarif-to-md")?
            .arg("-i")
            .arg(input)
            .arg("-o")
            .arg(&output)
            .arg("-f")
            .arg(format)
            .assert()
            .success();

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < threshold_ms,
            "{} format took too long: {:?}ms",
            format,
            duration.as_millis()
        );
    }

    Ok(())
}
