use anyhow::Result;
use rstest::*;
use sarif_to_md_core::{
    generators::SarifMarkdownGenerator, markdown::MarkdownFormat, ReportProcessorBuilder,
};
use std::fs;
fn load_example_sarif(name: &str) -> Result<String> {
    let path = format!("../../examples/sarif-files/{}.sarif", name);
    fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to load SARIF example {}: {}", name, e))
}

#[rstest]
#[case(MarkdownFormat::CommonMark, false)]
#[case(MarkdownFormat::CommonMark, true)]
#[case(MarkdownFormat::GitHubFlavored, false)]
#[case(MarkdownFormat::GitHubFlavored, true)]
fn test_generator_combinations(
    #[case] format: MarkdownFormat,
    #[case] with_emoji: bool,
) -> Result<()> {
    let sample_sarif = load_example_sarif("01-minimal")?;

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(format, with_emoji))
        .content(sample_sarif)
        .build()?;

    let result = processor.generate()?;

    assert!(!result.is_empty(), "Generated markdown should not be empty");
    assert!(
        result.contains("#") || result.contains("Security"),
        "Generated markdown should contain headers or security-related content"
    );
    match format {
        MarkdownFormat::GitHubFlavored => {
            assert!(
                result.contains("<details>") || result.contains("<summary>"),
                "GitHub Flavored output should contain collapsible elements"
            );
        }
        MarkdownFormat::CommonMark => {
            assert!(
                !result.contains("<details>"),
                "CommonMark output should not contain HTML elements"
            );
        }
    }

    // Emoji validation
    let has_emoji = result.chars().any(|c| c as u32 > 127);
    assert_eq!(
        has_emoji, with_emoji,
        "Emoji presence should match with_emoji flag"
    );

    Ok(())
}

#[rstest]
fn test_different_sarif_examples(
    #[values("01-minimal", "02-rule-metadata", "03-suppressions", "04-code-flows")] example: &str,
) -> Result<()> {
    let sarif_content = load_example_sarif(example)?;

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(sarif_content)
        .build()?;

    let result = processor.generate()?;

    assert!(
        !result.is_empty(),
        "Result should not be empty for {}",
        example
    );
    assert!(
        result.lines().count() > 1,
        "Result should have multiple lines for {}",
        example
    );

    let contains_security_content = result.to_lowercase().contains("security")
        || result.contains("#")
        || result.to_lowercase().contains("finding")
        || result.to_lowercase().contains("result");

    assert!(
        contains_security_content,
        "Result should contain security-related content for {}",
        example
    );

    Ok(())
}

#[rstest]
#[case("")]
#[case("invalid json")]
#[case("{}")]
#[case(r#"{"version": "invalid"}"#)]
fn test_invalid_sarif_handling(#[case] invalid_sarif: &str) -> Result<()> {
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(invalid_sarif.to_string())
        .build()?;

    let result = processor.generate();

    assert!(
        result.is_err(),
        "Should return error for invalid SARIF: {}",
        invalid_sarif
    );

    Ok(())
}

#[test]
fn test_builder_validation() -> Result<()> {
    let result = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .build();

    assert!(result.is_err(), "Should error when content is missing");

    let sample_sarif = load_example_sarif("01-minimal")?;
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(sample_sarif)
        .build()?;

    let result = processor.generate()?;
    assert!(!result.is_empty());

    Ok(())
}

#[rstest]
fn test_complex_sarif_features(
    #[values(
        "04-code-flows",
        "05-context-region",
        "07-result-stacks",
        "08-embedded-content"
    )]
    example: &str,
) -> Result<()> {
    let sarif_content = load_example_sarif(example)?;

    for format in [MarkdownFormat::CommonMark, MarkdownFormat::GitHubFlavored] {
        let processor = ReportProcessorBuilder::new()
            .generator(SarifMarkdownGenerator::new(format, false))
            .content(sarif_content.clone())
            .build()?;

        let result = processor.generate()?;

        assert!(
            !result.is_empty(),
            "Should generate content for complex example: {} with format: {:?}",
            example,
            format
        );

        assert!(
            result.len() > 100,
            "Complex examples should generate substantial content for {}",
            example
        );
    }

    Ok(())
}

#[test]
fn test_performance() -> Result<()> {
    let sarif_content = load_example_sarif("08-embedded-content")?;

    let start = std::time::Instant::now();

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::GitHubFlavored,
            true,
        ))
        .content(sarif_content)
        .build()?;

    let _result = processor.generate()?;

    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 5000,
        "Generation should complete within 5 seconds, took: {:?}",
        duration
    );

    Ok(())
}
