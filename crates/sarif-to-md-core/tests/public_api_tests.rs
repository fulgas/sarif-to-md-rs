//! # Public API Tests
//!
//! These tests verify the stability and reliability of the public API surface.
//! They ensure that the public interface contracts are maintained and that the
//! API behaves correctly under all conditions.
//!
//! ## What these tests cover:
//! - All public constructors and methods work as documented
//! - Builder pattern validation and error handling
//! - Error types and error handling scenarios
//! - Thread safety and memory safety guarantees
//! - Edge cases (malformed input, large data, Unicode, etc.)
//! - API stability and backward compatibility
//! - Documentation examples remain valid
//!
//! These tests answer: "Is the API reliable and stable for users?"

use anyhow::Result;
use rstest::*;
use sarif_to_md_core::{
    error::{BuilderError, Error},
    generators::SarifMarkdownGenerator,
    markdown::{MarkdownFormat, MarkdownGenerator},
    ReportProcessor, ReportProcessorBuilder,
};

mod common;
use common::{embedded_content_sarif, minimal_sarif, rule_metadata_sarif};

#[rstest]
fn test_public_api_report_processor_direct_construction(minimal_sarif: String) -> Result<()> {
    let generator = SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false);

    let processor = ReportProcessor::new(generator, minimal_sarif);
    let result = processor.generate()?;

    assert!(!result.is_empty());
    assert!(result.contains("#"));

    Ok(())
}

#[rstest]
fn test_public_api_builder_pattern_all_methods(minimal_sarif: String) -> Result<()> {
    let builder = ReportProcessorBuilder::new();
    let builder_with_generator = builder.generator(SarifMarkdownGenerator::new(
        MarkdownFormat::GitHubFlavored,
        true,
    ));
    let builder_with_content = builder_with_generator.content(minimal_sarif);
    let processor = builder_with_content.build()?;
    let result = processor.generate()?;

    assert!(result.contains("<details>") || result.contains("🛡️"));

    Ok(())
}

#[rstest]
fn test_public_api_builder_default() {
    let builder = ReportProcessorBuilder::default();

    let builder_with_generator = builder.generator(SarifMarkdownGenerator::new(
        MarkdownFormat::CommonMark,
        false,
    ));

    assert!(builder_with_generator
        .content("{}".to_string())
        .build()
        .is_ok());
}

#[rstest]
fn test_public_api_markdown_format_variants(minimal_sarif: String) -> Result<()> {
    // Test both variants exist and work
    for format in [MarkdownFormat::CommonMark, MarkdownFormat::GitHubFlavored] {
        let processor = ReportProcessorBuilder::new()
            .generator(SarifMarkdownGenerator::new(format, false))
            .content(minimal_sarif.clone())
            .build()?;

        let result = processor.generate()?;
        assert!(!result.is_empty());
    }

    Ok(())
}

#[rstest]
fn test_public_api_markdown_format_traits() {
    // Test Copy trait
    let format = MarkdownFormat::CommonMark;
    let copied_format = format;
    assert!(matches!(copied_format, MarkdownFormat::CommonMark));

    // Test Clone trait
    let cloned_format = format.clone();
    assert!(matches!(cloned_format, MarkdownFormat::CommonMark));

    // Test Debug trait
    let debug_str = format!("{:?}", format);
    assert!(debug_str.contains("CommonMark"));
}

#[rstest]
fn test_public_api_generator_constructor_variants(minimal_sarif: String) -> Result<()> {
    // Test all combinations of constructor parameters
    let test_cases = [
        (MarkdownFormat::CommonMark, false),
        (MarkdownFormat::CommonMark, true),
        (MarkdownFormat::GitHubFlavored, false),
        (MarkdownFormat::GitHubFlavored, true),
    ];

    for (format, with_emoji) in test_cases {
        let generator = SarifMarkdownGenerator::new(format, with_emoji);
        let processor = ReportProcessorBuilder::new()
            .generator(generator)
            .content(minimal_sarif.clone())
            .build()?;

        let result = processor.generate()?;
        assert!(!result.is_empty());

        let has_emoji = result.chars().any(|c| c as u32 > 127);
        assert_eq!(has_emoji, with_emoji, "Emoji setting should be respected");
    }

    Ok(())
}

#[rstest]
fn test_public_api_error_builder_missing_content() {
    let result = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .build();

    match result {
        Err(BuilderError::MissingContent) => {}
        _ => panic!("Expected BuilderError::MissingContent"),
    }
}

#[rstest]
fn test_public_api_error_invalid_json() {
    let invalid_json = "{ invalid json content";
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(invalid_json.to_string())
        .build()
        .expect("Builder should succeed");

    let result = processor.generate();
    match result {
        Err(Error::JsonError(_)) => {}
        _ => panic!("Expected Error::JsonError for invalid JSON"),
    }
}

#[rstest]
fn test_public_api_error_empty_sarif() {
    let empty_sarif = "{}";
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(empty_sarif.to_string())
        .build()
        .expect("Builder should succeed");

    let result = processor.generate();
    // Should handle empty SARIF gracefully or return an appropriate error
    assert!(result.is_err() || result.unwrap().contains("#"));
}

#[rstest]
fn test_public_api_markdown_generator_trait(minimal_sarif: String) -> Result<()> {
    let generator = SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false);

    // Parse SARIF to test trait method directly
    let sarif_data: serde_sarif::sarif::Sarif = serde_json::from_str(&minimal_sarif)?;
    let result = generator.generate_markdown_template(&sarif_data)?;

    assert!(!result.is_empty());
    assert!(result.contains("#"));

    Ok(())
}

#[rstest]
fn test_public_api_error_display_messages() {
    // Test BuilderError display
    let builder_error = BuilderError::MissingContent;
    assert_eq!(builder_error.to_string(), "Content was not provided");

    // Test that errors implement std::error::Error trait
    use std::error::Error as StdError;
    let _: &dyn StdError = &builder_error;
}

#[rstest]
fn test_public_api_thread_safety(minimal_sarif: String) -> Result<()> {
    // Test that types implement Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SarifMarkdownGenerator>();
    assert_send_sync::<MarkdownFormat>();
    assert_send_sync::<BuilderError>();

    // Test actual multi-threaded usage
    let handles: Vec<_> = (0..3)
        .map(|_| {
            let sarif = minimal_sarif.clone();
            std::thread::spawn(move || {
                let processor = ReportProcessorBuilder::new()
                    .generator(SarifMarkdownGenerator::new(
                        MarkdownFormat::CommonMark,
                        false,
                    ))
                    .content(sarif)
                    .build()
                    .expect("Build should succeed");

                processor.generate().expect("Generation should succeed")
            })
        })
        .collect();

    for handle in handles {
        let result = handle.join().expect("Thread should not panic");
        assert!(!result.is_empty());
    }

    Ok(())
}

#[rstest]
fn test_public_api_memory_safety_large_content(embedded_content_sarif: String) -> Result<()> {
    for _ in 0..5 {
        let processor = ReportProcessorBuilder::new()
            .generator(SarifMarkdownGenerator::new(
                MarkdownFormat::GitHubFlavored,
                true,
            ))
            .content(embedded_content_sarif.clone())
            .build()?;

        let result = processor.generate()?;
        assert!(
            result.len() > 100,
            "Large SARIF should produce substantial output"
        );
    }

    Ok(())
}

#[rstest]
fn test_public_api_processor_consumption(minimal_sarif: String) -> Result<()> {
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(minimal_sarif)
        .build()?;

    let result = processor.generate()?;
    assert!(!result.is_empty());

    Ok(())
}

#[rstest]
fn test_public_api_builder_method_chaining(minimal_sarif: String) -> Result<()> {
    let result = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::GitHubFlavored,
            true,
        ))
        .content(minimal_sarif)
        .build()?
        .generate()?;

    assert!(result.contains("🛡️") || result.contains("<details>"));

    Ok(())
}

// ==================== EDGE CASE AND BOUNDARY TESTS ====================

#[rstest]
fn test_edge_case_malformed_json_variants() {
    let malformed_cases = [
        "",                          // Empty string
        "{",                         // Incomplete JSON
        "}",                         // Invalid start
        "null",                      // Null value
        "[]",                        // Array instead of object
        r#"{"runs": null}"#,         // Null runs
        r#"{"runs": []}"#,           // Empty runs
        r#"{"version": "invalid"}"#, // Invalid version
    ];

    for (i, case) in malformed_cases.iter().enumerate() {
        let processor = ReportProcessorBuilder::new()
            .generator(SarifMarkdownGenerator::new(
                MarkdownFormat::CommonMark,
                false,
            ))
            .content(case.to_string())
            .build()
            .expect("Builder should succeed");

        let result = processor.generate();
        assert!(result.is_err(), "Case {} should fail: {}", i, case);
    }
}

#[rstest]
fn test_edge_case_very_large_strings() {
    let large_content = "a".repeat(1_000_000);
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(large_content)
        .build()
        .expect("Builder should succeed");

    let result = processor.generate();
    assert!(result.is_err(), "Very large non-JSON content should fail");
}

#[rstest]
fn test_edge_case_unicode_content(minimal_sarif: String) -> Result<()> {
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::GitHubFlavored,
            true,
        ))
        .content(minimal_sarif)
        .build()?;

    let result = processor.generate()?;

    // Should handle Unicode properly (String is always valid UTF-8)
    assert!(!result.is_empty());
    assert!(
        result.chars().any(|c| c as u32 > 127),
        "Should contain Unicode characters"
    );

    Ok(())
}

#[rstest]
fn test_edge_case_zero_sized_types() {
    let builder = ReportProcessorBuilder::new();
    assert_eq!(
        std::mem::size_of_val(&builder),
        std::mem::size_of::<()>() + std::mem::size_of::<Option<String>>()
    );
}

// ==================== DOCUMENTATION AND USAGE TESTS ====================

#[rstest]
fn test_documentation_example_basic_usage(minimal_sarif: String) -> Result<()> {
    // This test replicates the basic usage example that should be in docs

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content(minimal_sarif)
        .build()?;

    let markdown = processor.generate()?;

    assert!(!markdown.is_empty());
    assert!(markdown.contains("#"));

    Ok(())
}

#[rstest]
fn test_documentation_example_github_flavored(rule_metadata_sarif: String) -> Result<()> {
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::GitHubFlavored,
            true,
        ))
        .content(rule_metadata_sarif)
        .build()?;

    let markdown = processor.generate()?;

    assert!(markdown.contains("<details>") || markdown.contains("🛡️"));

    Ok(())
}

#[rstest]
fn test_documentation_error_handling() {
    let result = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .build();

    match result {
        Ok(_) => panic!("Should fail without content"),
        Err(BuilderError::MissingContent) => {
            // Expected error
        }
    }

    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::CommonMark,
            false,
        ))
        .content("invalid json".to_string())
        .build()
        .expect("Builder should succeed");

    match processor.generate() {
        Ok(_) => panic!("Should fail with invalid JSON"),
        Err(Error::JsonError(_)) => {
            // Expected error
        }
        Err(other) => panic!("Unexpected error type: {:?}", other),
    }
}

// ==================== API STABILITY AND BACKWARD COMPATIBILITY TESTS ====================

#[rstest]
fn test_api_stability_public_types_exist() {
    let _: ReportProcessor<SarifMarkdownGenerator> = ReportProcessor::new(
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false),
        "{}".to_string(),
    );

    let _: ReportProcessorBuilder<()> = ReportProcessorBuilder::new();
    let _: MarkdownFormat = MarkdownFormat::CommonMark;
    let _: BuilderError = BuilderError::MissingContent;
}

#[rstest]
fn test_api_stability_trait_methods() -> Result<()> {
    let generator = SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false);
    let sarif_data: serde_sarif::sarif::Sarif =
        serde_json::from_str("{\"version\":\"2.1.0\",\"runs\":[]}")?;

    let _result = generator.generate_markdown_template(&sarif_data)?;

    Ok(())
}

#[rstest]
fn test_api_stability_error_types() {
    use std::error::Error as StdError;

    let builder_error = BuilderError::MissingContent;
    let _: &dyn StdError = &builder_error;

    let json_error_result: Result<serde_json::Value, _> = serde_json::from_str("invalid json");
    let json_error = json_error_result.unwrap_err();
    let _: Error = json_error.into();
}

#[rstest]
fn test_api_stability_constructor_signatures() {
    let _generator = SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false);
    let _processor = ReportProcessor::new(_generator, "test".to_string());
    let _builder = ReportProcessorBuilder::new();
    let _default_builder = ReportProcessorBuilder::default();
}
