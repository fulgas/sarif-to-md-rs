//! # Integration Tests
//!
//! These tests verify the functional behavior and business logic of the SARIF to Markdown
//! converter. They focus on end-to-end scenarios, ensuring the library produces correct
//! output for various SARIF formats and performs adequately.
//!
//! ## What these tests cover:
//! - Different SARIF file formats work correctly
//! - Complex SARIF features (code flows, context regions, result stacks, etc.)
//! - Performance benchmarks and timing requirements
//! - Output quality and correctness validation
//!
//! These tests answer: "Does the library work correctly?"

use anyhow::Result;
use rstest::*;
use sarif_to_md_core::{
    generators::SarifMarkdownGenerator, markdown::MarkdownFormat, ReportProcessorBuilder,
};

mod common;
use common::{
    code_flows_sarif, context_region_sarif, embedded_content_sarif, minimal_sarif,
    result_stacks_sarif, rule_metadata_sarif, suppressions_sarif,
};

#[rstest]
#[case(minimal_sarif())]
#[case(rule_metadata_sarif())]
#[case(suppressions_sarif())]
#[case(code_flows_sarif())]
fn test_different_sarif_examples(#[case] sarif_content: String) -> Result<()> {
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(sarif_content)
        .build()?;

    let result = processor.generate()?;

    assert!(!result.is_empty(), "Result should not be empty");
    assert!(
        result.lines().count() > 1,
        "Result should have multiple lines"
    );

    let contains_security_content = result.to_lowercase().contains("security")
        || result.contains("#")
        || result.to_lowercase().contains("finding")
        || result.to_lowercase().contains("result");

    assert!(
        contains_security_content,
        "Result should contain security-related content"
    );

    Ok(())
}

#[rstest]
#[case(code_flows_sarif())]
#[case(context_region_sarif())]
#[case(result_stacks_sarif())]
#[case(embedded_content_sarif())]
fn test_complex_sarif_features(#[case] sarif_content: String) -> Result<()> {
    for format in [MarkdownFormat::CommonMark, MarkdownFormat::GitHubFlavored] {
        let processor = ReportProcessorBuilder::new()
            .generator(SarifMarkdownGenerator::new(format, false))
            .content(sarif_content.clone())
            .build()?;

        let result = processor.generate()?;

        assert!(
            !result.is_empty(),
            "Should generate content for complex example with format: {:?}",
            format
        );

        assert!(
            result.len() > 100,
            "Complex examples should generate substantial content"
        );
    }

    Ok(())
}

#[rstest]
fn test_performance(embedded_content_sarif: String) -> Result<()> {
    let start = std::time::Instant::now();

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::GitHubFlavored,
            true,
        ))
        .content(embedded_content_sarif)
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
