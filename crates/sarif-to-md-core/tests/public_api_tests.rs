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
use common::{embedded_content_sarif, minimal_sarif, rule_metadata_sarif, suppressions_sarif};

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

#[rstest]
#[case("v2.1.20")]
#[case("2.1.20")]
fn test_tool_version_single_v_prefix(#[case] version: &str) -> Result<()> {
    let sarif = format!(
        r#"{{"version":"2.1.0","runs":[{{"tool":{{"driver":{{"name":"KICS","version":"{version}"}}}},"results":[]}}]}}"#
    );
    for format in [MarkdownFormat::CommonMark, MarkdownFormat::GitHubFlavored] {
        let result =
            ReportProcessor::new(SarifMarkdownGenerator::new(format, false), sarif.clone())
                .generate()?;
        assert!(result.contains("KICS v2.1.20"), "{result}");
        assert!(!result.contains("vv2.1.20"));
    }
    Ok(())
}

#[rstest]
#[case(MarkdownFormat::CommonMark)]
#[case(MarkdownFormat::GitHubFlavored)]
fn test_suppressed_results_hidden_by_default(
    suppressions_sarif: String,
    #[case] format: MarkdownFormat,
) -> Result<()> {
    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(format, false),
        suppressions_sarif,
    )
    .generate()?;

    // Only results whose suppressions are all accepted or status-less are suppressed.
    assert!(result.contains("| Warning | 6 |"), "{result}");
    assert!(result.contains("**Suppressed:** 3 (not counted above; not shown)"));
    assert!(!result.contains("suppressed in source"));
    assert!(!result.contains("suppressed externally"));
    assert!(!result.contains("its suppression was accepted"));
    assert!(result.contains("its suppression was rejected"));
    assert!(result.contains("still under review."));
    assert!(result.contains("at least one suppression was accepted"));
    Ok(())
}

#[rstest]
fn test_include_suppressed_marks_them(suppressions_sarif: String) -> Result<()> {
    let generator =
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false).include_suppressed(true);
    let result = ReportProcessor::new(generator, suppressions_sarif).generate()?;

    assert!(result.contains("| Warning | 6 |"), "{result}");
    assert!(result.contains("listed below and marked as suppressed"));
    assert!(result.contains("suppressed in source"));
    assert!(result.contains("**Suppressed:** no status (inSource)"));
    assert!(result.contains("**Suppressed:** accepted (external)"));
    assert_eq!(result.matches("(suppressed)").count(), 3);
    Ok(())
}

#[rstest]
fn test_result_level_falls_back_to_rule_default_configuration() -> Result<()> {
    // A: level from defaultConfiguration via ruleIndex; B: default "none";
    // C: kind "pass" without level is "none"; D: an explicit level wins;
    // E: no level and no rule default is "warning".
    let sarif = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"t","rules":[
        {"id":"A","defaultConfiguration":{"level":"error"}},
        {"id":"B","defaultConfiguration":{"level":"none"}}]}},
      "results":[
        {"ruleId":"A","ruleIndex":0,"message":{"text":"a"}},
        {"ruleId":"B","ruleIndex":1,"message":{"text":"b"}},
        {"ruleId":"A","ruleIndex":0,"kind":"pass","message":{"text":"c"}},
        {"ruleId":"A","ruleIndex":0,"level":"note","message":{"text":"d"}},
        {"ruleId":"Z","message":{"text":"e"}}]}]}"#;

    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false),
        sarif.to_string(),
    )
    .generate()?;

    assert!(result.contains("| Error | 1 |"), "{result}");
    assert!(result.contains("| Warning | 1 |"), "{result}");
    assert!(result.contains("| Note | 1 |"), "{result}");
    assert!(result.contains("| None | 2 |"), "{result}");
    Ok(())
}

#[rstest]
fn test_rule_lookup_via_rule_reference_and_explicit_levels() -> Result<()> {
    // Results locate their rule through `rule.index` or `rule.id` (no ruleId/ruleIndex),
    // carry explicit warning/none levels, or reference no rule at all.
    let sarif = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"t","rules":[
        {"id":"R0","name":"ByIndex","defaultConfiguration":{"level":"error"}},
        {"id":"R1","name":"ById","defaultConfiguration":{"level":"note"}}]}},
      "results":[
        {"rule":{"index":0},"message":{"text":"by rule.index"}},
        {"rule":{"id":"R1"},"message":{"text":"by rule.id"}},
        {"ruleId":"R0","level":"warning","message":{"text":"explicit warning"}},
        {"ruleId":"R0","level":"none","message":{"text":"explicit none"}},
        {"message":{"text":"no rule"}}]}]}"#;

    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false),
        sarif.to_string(),
    )
    .generate()?;

    assert!(result.contains("#### Error - R0"), "{result}");
    assert!(result.contains("#### Note - R1"), "{result}");
    assert!(result.contains("| Warning | 2 |"), "{result}");
    assert!(result.contains("| None | 1 |"), "{result}");
    assert!(result.contains("#### Warning - unknown"), "{result}");
    Ok(())
}

#[rstest]
fn test_null_values_are_treated_as_absent() -> Result<()> {
    // gosec emits `"relationships": [null]`; other producers write `null` for optional fields.
    let sarif = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"gosec","version":null,
        "rules":[{"id":"G101","relationships":[null],"helpUri":null}]}},
      "results":[{"ruleId":"G101","level":"error","message":{"text":"hardcoded credentials"},
        "locations":null}]}]}"#;

    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false),
        sarif.to_string(),
    )
    .generate()?;

    assert!(result.contains("| Error | 1 |"), "{result}");
    assert!(result.contains("hardcoded credentials"));
    Ok(())
}

#[rstest]
#[case::common_mark(MarkdownFormat::CommonMark)]
#[case::github_flavored(MarkdownFormat::GitHubFlavored)]
fn test_multiline_messages_keep_their_line_breaks(#[case] format: MarkdownFormat) -> Result<()> {
    // trivy puts package, version and fix on separate lines of one message.
    let sarif = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"Trivy"}},
      "results":[{"ruleId":"CVE-2020-14343","level":"error",
        "message":{"text":"Package: PyYAML\nInstalled Version: 5.3"}}]}]}"#;

    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(format, false),
        sarif.to_string(),
    )
    .generate()?;

    assert!(
        result.contains("**Message:**\n\n```text\nPackage: PyYAML\nInstalled Version: 5.3\n```"),
        "{result}"
    );
    Ok(())
}

#[rstest]
#[case::common_mark(MarkdownFormat::CommonMark)]
#[case::github_flavored(MarkdownFormat::GitHubFlavored)]
fn test_message_and_description_render_as_literal_text(
    #[case] format: MarkdownFormat,
) -> Result<()> {
    // Issue #99: messages quote scanned content, so they can carry HTML and Markdown
    // written by the author of the code under review: closing tags, block markers,
    // links, remote images, GitHub mentions and code fences.
    let sarif = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"t","rules":[
        {"id":"R1","shortDescription":{"text":"desc </summary></details><h1>x</h1>"}}]}},
      "results":[{"ruleId":"R1","message":{"text":"line one\n- bullet two\n# heading three\n</details>injected\n![i](https://x.test/p.png) [click](https://evil.test) @team\n```\nfence"},
        "locations":[{"physicalLocation":{"artifactLocation":{"uri":"a.py"},"region":{"startLine":3}}}]}]}]}"#;

    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(format, false),
        sarif.to_string(),
    )
    .generate()?;

    let message = concat!(
        "**Message:**\n\n````text\nline one\n- bullet two\n# heading three\n",
        "</details>injected\n![i](https://x.test/p.png) [click](https://evil.test) @team\n",
        "```\nfence\n````"
    );
    let description = "**Description:**\n\n```text\ndesc </summary></details><h1>x</h1>\n```";
    assert!(result.contains(message), "{result}");
    assert!(result.contains(description), "{result}");
    // Inside the code block the text is literal, so only the report's own tags count.
    let tags = |tag: &str| result.lines().filter(|l| l.trim() == tag).count();
    assert_eq!(tags("<details>"), tags("</details>"), "{result}");
    Ok(())
}

#[rstest]
fn test_message_markdown_is_not_used_as_message_text() -> Result<()> {
    // `markdown` is the producer's own Markdown, not plain text: it is never
    // shown in place of a missing `text`.
    let sarif = r#"{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"t"}},
      "results":[{"ruleId":"R1","message":{"markdown":"<b>bold</b> [x](https://x.test)"}}]}]}"#;

    let result = ReportProcessor::new(
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false),
        sarif.to_string(),
    )
    .generate()?;

    assert!(result.contains("```text\nNo message\n```"), "{result}");
    assert!(!result.contains("bold"), "{result}");
    Ok(())
}

#[rstest]
#[case::current_time(None, "Report generated on 20")]
#[case::epoch(Some(1_700_000_000), "Report generated on 2023-11-14 22:13:20 UTC")]
#[case::unix_epoch(Some(0), "Report generated on 1970-01-01 00:00:00 UTC")]
fn test_report_footer_timestamp(
    minimal_sarif: String,
    #[case] epoch: Option<u64>,
    #[case] expected: &str,
) -> Result<()> {
    use std::time::{Duration, UNIX_EPOCH};

    let mut generator = SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false);
    if let Some(seconds) = epoch {
        generator = generator.generated_at(UNIX_EPOCH + Duration::from_secs(seconds));
    }
    let result = ReportProcessor::new(generator, minimal_sarif).generate()?;

    assert!(result.contains(expected), "{result}");
    Ok(())
}

#[rstest]
fn test_report_footer_clamps_out_of_range_times(minimal_sarif: String) -> Result<()> {
    use std::time::{Duration, UNIX_EPOCH};

    let far_future = UNIX_EPOCH
        .checked_add(Duration::from_secs(400_000_000_000))
        .expect("representable on this platform");
    let generator =
        SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false).generated_at(far_future);
    let result = ReportProcessor::new(generator, minimal_sarif.clone()).generate()?;
    assert!(result.contains("Report generated on 9999-"), "{result}");

    if let Some(far_past) = UNIX_EPOCH.checked_sub(Duration::from_secs(400_000_000_000)) {
        let generator =
            SarifMarkdownGenerator::new(MarkdownFormat::CommonMark, false).generated_at(far_past);
        let result = ReportProcessor::new(generator, minimal_sarif).generate()?;
        assert!(result.contains("Report generated on -9999-"), "{result}");
    }
    Ok(())
}
