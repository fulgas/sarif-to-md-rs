use crate::markdown::generator::{GeneratorError, MarkdownGenerator};
use crate::parser::ParsedReport;
use askama::Template;
use serde_sarif::sarif::{ResultLevel, Sarif};
use std::collections::HashMap;
use std::fmt;

#[derive(Template)]
#[template(path = "sarif/sarif_report.md")]
struct SarifReportTemplate {
    runs: Vec<SarifRun>,
    timestamp: String,
    with_emoji: bool,
}

#[derive(Clone, Debug)]
struct SarifRun {
    tool_name: String,
    tool_version: Option<String>,
    total_results: usize,
    severity_counts: Vec<SeverityCount>,
    results: Vec<SarifResultView>,
}

#[derive(Clone, Debug)]
struct SeverityCount {
    level: SarifLevel,
    count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SarifLevel {
    Error,
    Warning,
    Note,
    None,
}

impl fmt::Display for SarifLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SarifLevel::Error => write!(f, "Error"),
            SarifLevel::Warning => write!(f, "Warning"),
            SarifLevel::Note => write!(f, "Note"),
            SarifLevel::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Debug)]
struct SarifResultView {
    rule_id: String,
    level: SarifLevel,
    message: String,
    locations: Vec<SarifLocation>,
    rule_metadata: Option<RuleMetadata>,
}

#[derive(Clone, Debug)]
struct RuleMetadata {
    name: Option<String>,
    description: Option<String>,
    help_uri: Option<String>,
    cwe_ids: Vec<String>,
    tags: Vec<String>,
}

#[derive(Clone, Debug)]
struct SarifLocation {
    file: Option<String>,
    line: Option<i64>,
    column: Option<i64>,
}

pub(crate) struct SarifCommonMarkGenerator {
    with_emoji: bool,
}

impl SarifCommonMarkGenerator {
    pub(crate) fn new(with_emoji: bool) -> Self {
        Self { with_emoji }
    }

    fn convert_sarif_to_view(&self, sarif: &Sarif) -> Vec<SarifRun> {
        sarif
            .runs
            .iter()
            .map(|run| {
                let tool_name = run.tool.driver.name.clone();
                let tool_version = run.tool.driver.version.clone();

                let mut rules_map = HashMap::new();
                if let Some(rules) = &run.tool.driver.rules {
                    for rule in rules {
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

                        let tags: Vec<String> = rule
                            .properties
                            .as_ref()
                            .and_then(|props| props.tags.clone())
                            .unwrap_or_default();

                        let description = rule
                            .short_description
                            .as_ref()
                            .map(|sd| sd.text.clone())
                            .or_else(|| rule.full_description.as_ref().map(|fd| fd.text.clone()));

                        let metadata = RuleMetadata {
                            name: rule.name.clone(),
                            description,
                            help_uri: rule.help_uri.clone(),
                            cwe_ids,
                            tags,
                        };

                        rules_map.insert(rule.id.clone(), metadata);
                    }
                }

                let results: Vec<SarifResultView> = run
                    .results
                    .as_ref()
                    .map(|results| {
                        results
                            .iter()
                            .map(|r| {
                                let level = r
                                    .level
                                    .as_ref()
                                    .map(|l| match l {
                                        ResultLevel::Error => SarifLevel::Error,
                                        ResultLevel::Warning => SarifLevel::Warning,
                                        ResultLevel::Note => SarifLevel::Note,
                                        _ => SarifLevel::None,
                                    })
                                    .unwrap_or(SarifLevel::Warning);

                                let message = r
                                    .message
                                    .text
                                    .as_deref()
                                    .or(r.message.markdown.as_deref())
                                    .unwrap_or("No message")
                                    .to_string();

                                let locations = r
                                    .locations
                                    .as_ref()
                                    .map(|locs| {
                                        locs.iter()
                                            .filter_map(|loc| {
                                                loc.physical_location.as_ref().map(|pl| {
                                                    SarifLocation {
                                                        file: pl
                                                            .artifact_location
                                                            .as_ref()
                                                            .and_then(|al| al.uri.clone()),
                                                        line: pl
                                                            .region
                                                            .as_ref()
                                                            .and_then(|r| r.start_line),
                                                        column: pl
                                                            .region
                                                            .as_ref()
                                                            .and_then(|r| r.start_column),
                                                    }
                                                })
                                            })
                                            .collect()
                                    })
                                    .unwrap_or_default();

                                let rule_id =
                                    r.rule_id.clone().unwrap_or_else(|| "unknown".to_string());

                                let rule_metadata = rules_map.get(&rule_id).cloned();

                                SarifResultView {
                                    rule_id,
                                    level,
                                    message,
                                    locations,
                                    rule_metadata,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                // Count by severity
                let mut level_counts: HashMap<SarifLevel, usize> = HashMap::new();
                for result in &results {
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

                // Remove zero counts
                severity_counts.retain(|sc| sc.count > 0);

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
}

impl MarkdownGenerator for SarifCommonMarkGenerator {
    fn generate_markdown_template(
        &self,
        parsed_report: &ParsedReport,
    ) -> Result<String, GeneratorError> {
        let sarif = match parsed_report {
            ParsedReport::Sarif(s) => s,
            _ => {
                return Err(GeneratorError::AskamaError(askama::Error::Custom(
                    "Expected SARIF report".into(),
                )))
            }
        };

        let runs = self.convert_sarif_to_view(sarif);
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let template = SarifReportTemplate {
            runs,
            timestamp,
            with_emoji: self.with_emoji,
        };

        template.render().map_err(GeneratorError::AskamaError)
    }
}
