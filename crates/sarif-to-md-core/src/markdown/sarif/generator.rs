use crate::markdown::sarif::types::{
    CodeBlock, ResultProperties, RuleMetadata, SarifLevel, SarifLocation, SarifReportTemplate,
    SarifResultView, SarifRun, SeverityCount,
};
use crate::markdown::{GeneratorError, MarkdownFormat, MarkdownGenerator};
use askama::Template;
use std::collections::{BTreeMap, HashMap};
use std::time::SystemTime;

/// Generator for converting SARIF reports to Markdown format.
pub struct SarifMarkdownGenerator {
    markdown_format: MarkdownFormat,
    with_emoji: bool,
    include_suppressed: bool,
    generated_at: Option<SystemTime>,
}

impl SarifMarkdownGenerator {
    /// Create a new SARIF Markdown generator.
    pub fn new(markdown_format: MarkdownFormat, with_emoji: bool) -> Self {
        Self {
            markdown_format,
            with_emoji,
            include_suppressed: false,
            generated_at: None,
        }
    }

    /// Include suppressed results in the detailed results, marked as suppressed.
    ///
    /// By default suppressed results are left out of the report. Either way they are
    /// never counted in the severity summary; the summary reports how many there are.
    #[must_use]
    pub fn include_suppressed(mut self, include_suppressed: bool) -> Self {
        self.include_suppressed = include_suppressed;
        self
    }

    /// Show `time` in the "Report generated on" footer instead of the current time,
    /// so the same input always gives the same output (for example from
    /// `SOURCE_DATE_EPOCH`). Times outside years -9999..=9999 are clamped to that range.
    #[must_use]
    pub fn generated_at(mut self, time: SystemTime) -> Self {
        self.generated_at = Some(time);
        self
    }
}

impl MarkdownGenerator for SarifMarkdownGenerator {
    type Input = serde_sarif::sarif::Sarif;

    fn generate_markdown_template(&self, sarif: &Self::Input) -> Result<String, GeneratorError> {
        let runs = self.convert_sarif_to_view(sarif);
        let timestamp = match self.generated_at {
            None => jiff::Timestamp::now(),
            Some(time) => jiff::Timestamp::try_from(time).unwrap_or_else(|_| {
                if time < SystemTime::UNIX_EPOCH {
                    jiff::Timestamp::MIN
                } else {
                    jiff::Timestamp::MAX
                }
            }),
        }
        .strftime("%Y-%m-%d %H:%M:%S UTC")
        .to_string();

        let is_gfm = matches!(self.markdown_format, MarkdownFormat::GitHubFlavored);

        let template = SarifReportTemplate {
            runs,
            timestamp,
            with_emoji: self.with_emoji,
            is_gfm,
        };
        Ok(template.render()?)
    }
}

impl SarifMarkdownGenerator {
    fn convert_sarif_to_view(&self, sarif: &serde_sarif::sarif::Sarif) -> Vec<SarifRun> {
        sarif
            .runs
            .iter()
            .map(|run| {
                let tool_name = run.tool.driver.name.clone();
                // Template prefixes "v"; strip one already present (KICS emits "v2.1.20").
                let tool_version = run
                    .tool
                    .driver
                    .version
                    .as_deref()
                    .map(|v| v.strip_prefix(['v', 'V']).unwrap_or(v).to_string());

                let rules = run.tool.driver.rules.as_deref().unwrap_or_default();
                let rules_metadata: Vec<RuleMetadata> = rules
                    .iter()
                    .map(|r| self.extract_rule_metadata(r))
                    .collect();

                let mut results: Vec<SarifResultView> = run
                    .results
                    .as_ref()
                    .map(|results| {
                        results
                            .iter()
                            .map(|r| self.convert_result(r, rules, &rules_metadata))
                            .collect()
                    })
                    .unwrap_or_default();

                let suppressed_count = results.iter().filter(|r| r.suppressed).count();
                let total_results = results.len() - suppressed_count;
                let severity_counts =
                    self.calculate_severity_counts(results.iter().filter(|r| !r.suppressed));

                if !self.include_suppressed {
                    results.retain(|r| !r.suppressed);
                }

                SarifRun {
                    tool_name,
                    tool_version,
                    total_results,
                    suppressed_count,
                    include_suppressed: self.include_suppressed,
                    severity_counts,
                    results,
                }
            })
            .collect()
    }

    fn extract_rule_metadata(
        &self,
        rule: &serde_sarif::sarif::ReportingDescriptor,
    ) -> RuleMetadata {
        let cwe_ids: Vec<String> = rule
            .properties
            .as_ref()
            .and_then(|props| props.additional_properties.get("cwe"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let cve_ids: Vec<String> = rule
            .properties
            .as_ref()
            .and_then(|props| props.additional_properties.get("cve"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let tags: Vec<String> = rule
            .properties
            .as_ref()
            .and_then(|props| props.tags.clone())
            .unwrap_or_default();

        RuleMetadata {
            name: rule.name.clone(),
            description: rule
                .short_description
                .as_ref()
                .map(|sd| sd.text.as_str())
                .or_else(|| rule.full_description.as_ref().map(|fd| fd.text.as_str()))
                .map(CodeBlock::new),
            help_uri: rule.help_uri.clone(),
            tags,
            cwe_ids,
            cve_ids,
        }
    }

    fn convert_result(
        &self,
        result: &serde_sarif::sarif::Result,
        rules: &[serde_sarif::sarif::ReportingDescriptor],
        rules_metadata: &[RuleMetadata],
    ) -> SarifResultView {
        let rule_index = find_rule(result, rules);
        let rule = rule_index.map(|i| &rules[i]);
        let level = result_level(result, rule);

        let message = result
            .message
            .text
            .as_deref()
            // `text` is plain text and always present (SARIF 2.1.0 §3.11.9 requires it
            // whenever `markdown` is); `markdown` is the producer's own Markdown, not a
            // stand-in for it.
            .unwrap_or("No message");
        let message = CodeBlock::new(message);

        let locations = result
            .locations
            .as_ref()
            .map(|locs| {
                locs.iter()
                    .filter_map(|loc| {
                        loc.physical_location.as_ref().map(|pl| SarifLocation {
                            file: pl.artifact_location.as_ref().and_then(|al| al.uri.clone()),
                            line: pl.region.as_ref().and_then(|r| r.start_line),
                            column: pl.region.as_ref().and_then(|r| r.start_column),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let rule_id = result
            .rule_id
            .clone()
            .or_else(|| result.rule.as_ref().and_then(|r| r.id.clone()))
            .or_else(|| rule.map(|r| r.id.clone()))
            .unwrap_or_else(|| "unknown".to_string());
        let rule_metadata = rule_index.map(|i| rules_metadata[i].clone());

        let properties = result
            .properties
            .as_ref()
            .map(|props| self.extract_result_properties(&props.additional_properties));

        let suppressions = result.suppressions.as_deref().unwrap_or_default();
        let suppressed = is_suppressed(suppressions);
        let suppressions = if suppressed {
            suppressions.iter().map(describe_suppression).collect()
        } else {
            Vec::new()
        };

        SarifResultView {
            rule_id,
            level,
            message,
            locations,
            rule_metadata,
            properties,
            suppressed,
            suppressions,
        }
    }

    fn extract_result_properties(
        &self,
        additional_properties: &BTreeMap<String, serde_json::Value>,
    ) -> ResultProperties {
        let issue_confidence = additional_properties
            .get("issue_confidence")
            .and_then(|v| v.as_str().map(String::from));

        let precision = additional_properties
            .get("precision")
            .and_then(|v| v.as_str().map(String::from));

        let problem_severity = additional_properties
            .get("problem.severity")
            .and_then(|v| v.as_str().map(String::from));

        let security_severity = additional_properties
            .get("security-severity")
            .and_then(|v| v.as_str().map(String::from))
            .or_else(|| {
                additional_properties
                    .get("securitySeverity")
                    .and_then(|v| v.as_str().map(String::from))
            });

        let custom_fields = additional_properties
            .iter()
            .filter_map(|(k, v)| {
                // Skip known fields
                if matches!(
                    k.as_str(),
                    "issue_confidence"
                        | "precision"
                        | "problem.severity"
                        | "security-severity"
                        | "securitySeverity"
                ) {
                    return None;
                }

                // Convert value to string representation
                let value_str = match v {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => serde_json::to_string(v).ok()?,
                };

                Some((k.clone(), value_str))
            })
            .collect();

        ResultProperties {
            issue_confidence,
            precision,
            problem_severity,
            security_severity,
            custom_fields,
        }
    }

    fn calculate_severity_counts<'a>(
        &self,
        results: impl IntoIterator<Item = &'a SarifResultView>,
    ) -> Vec<SeverityCount> {
        let mut level_counts = HashMap::new();
        for result in results {
            *level_counts.entry(result.level.clone()).or_insert(0) += 1;
        }

        let mut severity_counts: Vec<SeverityCount> = vec![
            SeverityCount {
                level: SarifLevel::Error,
                count: *level_counts.get(&SarifLevel::Error).unwrap_or(&0),
            },
            SeverityCount {
                level: SarifLevel::Warning,
                count: *level_counts.get(&SarifLevel::Warning).unwrap_or(&0),
            },
            SeverityCount {
                level: SarifLevel::Note,
                count: *level_counts.get(&SarifLevel::Note).unwrap_or(&0),
            },
            SeverityCount {
                level: SarifLevel::None,
                count: *level_counts.get(&SarifLevel::None).unwrap_or(&0),
            },
        ];

        severity_counts.retain(|sc| sc.count > 0);
        severity_counts
    }
}

/// SARIF 2.1.0 §3.27.23 says to examine the suppressions' `status` values but does not
/// say how to combine several of them. A result counts as suppressed only when it has at
/// least one suppression and every one of them is `accepted` or has no status, so a
/// result anyone marked `rejected` or `underReview` is never hidden.
fn is_suppressed(suppressions: &[serde_sarif::sarif::Suppression]) -> bool {
    !suppressions.is_empty()
        && suppressions.iter().all(|s| {
            matches!(
                s.status.as_ref().and_then(|v| v.as_str()),
                None | Some("accepted")
            )
        })
}

/// "accepted (external)", "no status (inSource)", ...
fn describe_suppression(suppression: &serde_sarif::sarif::Suppression) -> String {
    let status = suppression
        .status
        .as_ref()
        .and_then(|v| v.as_str())
        .unwrap_or("no status");
    match suppression.kind.as_str() {
        Some(kind) => format!("{status} ({kind})"),
        None => status.to_string(),
    }
}

/// Index of the result's rule in the driver's rules: by index first (`rule.index`, then
/// `ruleIndex`), then by id (`rule.id`, then `ruleId`). SARIF 2.1.0 §3.27.5-§3.27.7.
fn find_rule(
    result: &serde_sarif::sarif::Result,
    rules: &[serde_sarif::sarif::ReportingDescriptor],
) -> Option<usize> {
    let index = result
        .rule
        .as_ref()
        .and_then(|r| r.index)
        .or(result.rule_index);
    let by_index = index
        .and_then(|i| usize::try_from(i).ok())
        .filter(|&i| i < rules.len());
    let id = result
        .rule
        .as_ref()
        .and_then(|r| r.id.as_deref())
        .or(result.rule_id.as_deref());
    by_index.or_else(|| id.and_then(|id| rules.iter().position(|r| r.id == id)))
}

/// SARIF 2.1.0 §3.27.10: an explicit `level` wins. Without one, a result whose `kind`
/// is not `fail` (absent means `fail`) is `none`; otherwise it takes the rule's
/// `defaultConfiguration.level`, falling back to `warning`.
// ponytail: per-invocation ruleConfigurationOverrides are not consulted; add them if
// a producer relies on them.
fn result_level(
    result: &serde_sarif::sarif::Result,
    rule: Option<&serde_sarif::sarif::ReportingDescriptor>,
) -> SarifLevel {
    use serde_sarif::sarif::{ResultKind, ResultLevel};

    if let Some(level) = result.level {
        return match level {
            ResultLevel::Error => SarifLevel::Error,
            ResultLevel::Warning => SarifLevel::Warning,
            ResultLevel::Note => SarifLevel::Note,
            ResultLevel::None => SarifLevel::None,
        };
    }
    if result.kind.as_ref().is_some_and(|k| *k != ResultKind::Fail) {
        return SarifLevel::None;
    }
    let default_level = rule
        .and_then(|r| r.default_configuration.as_ref())
        .and_then(|c| c.level.as_ref())
        .and_then(|l| l.as_str());
    match default_level {
        Some("error") => SarifLevel::Error,
        Some("note") => SarifLevel::Note,
        Some("none") => SarifLevel::None,
        _ => SarifLevel::Warning,
    }
}
