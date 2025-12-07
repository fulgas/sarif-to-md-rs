use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_expected_output(name: &str, format: &str) -> Result<String> {
    let path = format!("examples/expected-outputs/{}/{}.md", format, name);
    fs::read_to_string(&path).with_context(|| format!("Failed to load expected output: {}", path))
}

pub fn normalize_markdown(content: &str) -> String {
    content
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

pub fn assert_markdown_equivalent(actual: &str, expected: &str, context: &str) -> Result<()> {
    let actual_normalized = normalize_markdown(actual);
    let expected_normalized = normalize_markdown(expected);

    if actual_normalized != expected_normalized {
        anyhow::bail!(
            "Markdown content mismatch in {}\nExpected length: {}\nActual length: {}\nFirst difference at position: {:?}",
            context,
            expected_normalized.len(),
            actual_normalized.len(),
            actual_normalized
                .chars()
                .zip(expected_normalized.chars())
                .position(|(a, e)| a != e)
        );
    }

    Ok(())
}

pub fn has_expected_output(example: &str, format: &str) -> bool {
    let path = format!("examples/expected-outputs/{}/{}.md", format, example);
    Path::new(&path).exists()
}
