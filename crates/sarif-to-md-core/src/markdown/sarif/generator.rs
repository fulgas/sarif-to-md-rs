use crate::markdown::sarif::types::{
    ResultProperties, RuleMetadata, SarifLevel, SarifLocation, SarifReportTemplate,
    SarifResultView, SarifRun, SeverityCount,
};
use crate::markdown::{GeneratorError, MarkdownFormat, MarkdownGenerator};
use askama::Template;
use std::collections::{BTreeMap, HashMap};

/// Generator for converting SARIF reports to Markdown format.
pub struct SarifMarkdownGenerator {
    markdown_format: MarkdownFormat,
    with_emoji: bool,
}

impl SarifMarkdownGenerator {
    /// Create a new SARIF Markdown generator.
    pub fn new(markdown_format: MarkdownFormat, with_emoji: bool) -> Self {
        Self {
            markdown_format,
            with_emoji,
        }
    }
}

impl MarkdownGenerator for SarifMarkdownGenerator {
    type Input = serde_sarif::sarif::Sarif;

    fn generate_markdown_template(&self, sarif: &Self::Input) -> Result<String, GeneratorError> {
        let runs = self.convert_sarif_to_view(sarif);
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
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
                let tool_version = run.tool.driver.version.clone();

                let mut rules_map = HashMap::new();
                if let Some(rules) = &run.tool.driver.rules {
                    for rule in rules {
                        let metadata = self.extract_rule_metadata(rule);
                        rules_map.insert(rule.id.clone(), metadata);
                    }
                }

                let results: Vec<SarifResultView> = run
                    .results
                    .as_ref()
                    .map(|results| {
                        results
                            .iter()
                            .map(|r| self.convert_result(r, &rules_map))
                            .collect()
                    })
                    .unwrap_or_default();

                let severity_counts = self.calculate_severity_counts(&results);

                SarifRun {
                    tool_name,
                    tool_version,
                    total_results: results.len(),
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
                .map(|sd| sd.text.clone())
                .or_else(|| rule.full_description.as_ref().map(|fd| fd.text.clone())),
            help_uri: rule.help_uri.clone(),
            tags,
            cwe_ids,
            cve_ids,
        }
    }

    fn convert_result(
        &self,
        result: &serde_sarif::sarif::Result,
        rules_map: &HashMap<String, RuleMetadata>,
    ) -> SarifResultView {
        let level = result
            .level
            .as_ref()
            .map(|l| match l {
                serde_sarif::sarif::ResultLevel::Error => SarifLevel::Error,
                serde_sarif::sarif::ResultLevel::Warning => SarifLevel::Warning,
                serde_sarif::sarif::ResultLevel::Note => SarifLevel::Note,
                _ => SarifLevel::None,
            })
            .unwrap_or(SarifLevel::Warning);

        let message = result
            .message
            .text
            .as_deref()
            .or(result.message.markdown.as_deref())
            .unwrap_or("No message")
            .to_string();

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
            .unwrap_or_else(|| "unknown".to_string());
        let rule_metadata = rules_map.get(&rule_id).cloned();

        let properties = result
            .properties
            .as_ref()
            .map(|props| self.extract_result_properties(&props.additional_properties));

        SarifResultView {
            rule_id,
            level,
            message,
            locations,
            rule_metadata,
            properties,
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

    fn calculate_severity_counts(&self, results: &[SarifResultView]) -> Vec<SeverityCount> {
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
