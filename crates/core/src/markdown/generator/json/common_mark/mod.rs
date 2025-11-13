use crate::markdown::generator::{GeneratorError, MarkdownGenerator};
use crate::parser::ParsedReport;
use askama::Template;
use serde_snyk_container::{Severity, VulnerabilitiesItem};
use std::collections::HashMap;
use std::fmt;

#[derive(Template)]
#[template(path = "json/report.md")]
struct ReportTemplate {
    projects: Vec<ReportProject>,
    timestamp: String,
    with_emoji: bool,
}
struct ReportProject {
    name: String,
    organization: String,
    project_type: ReportProjectType,
    target_file: String,
    summary: ReportSummary,
    vulnerabilities: Vec<ReportVulnerability>,
}

enum ReportProjectType {
    DockerImage,
    Application,
}

#[derive(Clone, Default)]
struct ReportSummary {
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
    unique_count: usize,
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
enum ReportSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl From<Severity> for ReportSeverity {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Low => ReportSeverity::Low,
            Severity::Medium => ReportSeverity::Medium,
            Severity::High => ReportSeverity::High,
            Severity::Critical => ReportSeverity::Critical,
        }
    }
}

impl fmt::Display for ReportSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportSeverity::Critical => write!(f, "Critical"),
            ReportSeverity::High => write!(f, "High"),
            ReportSeverity::Medium => write!(f, "Medium"),
            ReportSeverity::Low => write!(f, "Low"),
        }
    }
}

struct ReportVulnerability {
    id: String,
    title: String,
    severity: ReportSeverity,
    package_name: String,
    version: String,
    cvss_score: Option<f64>,
    is_upgradable: bool,
    is_patchable: bool,
    cve_ids: Vec<String>,
    from_paths: Vec<Vec<String>>,
}

pub(crate) struct CommonMarkGenerator {
    with_emoji: bool,
}

impl CommonMarkGenerator {
    pub(crate) fn new(with_emoji: bool) -> CommonMarkGenerator {
        Self { with_emoji }
    }
}

impl MarkdownGenerator for CommonMarkGenerator {
    fn generate_markdown_template(
        &self,
        parsed_report: &ParsedReport,
    ) -> Result<String, GeneratorError> {
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let projects = match parsed_report {
            ParsedReport::Container(snyk_container) => {
                let mut projects = Vec::new();

                let image_vulns = parse_and_deduplicate_vulnerabilities(
                    &snyk_container.vulnerabilities,
                    snyk_container.unique_count as usize,
                );

                let image_summary = calculate_summary(&image_vulns);

                projects.push(ReportProject {
                    name: snyk_container.project_name.clone(),
                    organization: snyk_container.org.clone(),
                    project_type: ReportProjectType::DockerImage,
                    target_file: snyk_container.target_file.clone(),
                    vulnerabilities: image_vulns,
                    summary: image_summary,
                });

                for app in &snyk_container.applications {
                    let app_vulns = parse_and_deduplicate_vulnerabilities(
                        &app.vulnerabilities,
                        app.unique_count as usize,
                    );
                    let app_summary = calculate_summary(&app_vulns);
                    projects.push(ReportProject {
                        name: app.project_name.clone(),
                        organization: app.org.clone(),
                        project_type: ReportProjectType::Application,
                        target_file: app.target_file.clone(),
                        vulnerabilities: app_vulns,
                        summary: app_summary,
                    });
                }

                projects
            }
            _ => {
                return Err(GeneratorError::AskamaError(askama::Error::Custom(
                    "Expected Json report".into(),
                )))
            }
        };

        let template_ctx = ReportTemplate {
            projects,
            timestamp,
            with_emoji: self.with_emoji,
        };
        template_ctx.render().map_err(GeneratorError::AskamaError)
    }
}

fn parse_and_deduplicate_vulnerabilities(
    vulns: &[VulnerabilitiesItem],
    expected_unique_count: usize,
) -> Vec<ReportVulnerability> {
    let mut vuln_map: HashMap<String, ReportVulnerability> = HashMap::new();

    for v in vulns {
        // Skip if no ID
        let Some(id) = &v.id else { continue };

        if let Some(existing_vuln) = vuln_map.get_mut(id) {
            if !v.from.is_empty() {
                existing_vuln.from_paths.push(v.from.clone());
            }
        } else {
            let cvss_score = v.cvss_details.first().and_then(|d| d.cvss_v3_base_score);

            let cve_ids = v
                .identifiers
                .as_ref()
                .map(|i| i.cve.clone())
                .unwrap_or_default();

            let from_paths = if !v.from.is_empty() {
                vec![v.from.clone()]
            } else {
                vec![]
            };

            vuln_map.insert(
                id.clone(),
                ReportVulnerability {
                    id: id.clone(),
                    title: v.title.clone().unwrap_or_else(|| id.clone()),
                    severity: v.severity.into(),
                    package_name: v.package_name.clone().unwrap_or_default(),
                    version: v.version.clone().unwrap_or_default(),
                    cvss_score,
                    is_upgradable: v.is_upgradable.unwrap_or(false),
                    is_patchable: v.is_patchable.unwrap_or(false),
                    cve_ids,
                    from_paths,
                },
            );
        }
    }

    let mut deduplicated: Vec<_> = vuln_map.into_values().collect();
    deduplicated.sort_by(|a, b| a.severity.cmp(&b.severity));

    // Validation check
    if expected_unique_count > 0 && deduplicated.len() != expected_unique_count {
        eprintln!(
            "      ⚠️  Warning: Expected {} unique vulnerabilities but found {}",
            expected_unique_count,
            deduplicated.len()
        );
    }

    deduplicated
}

fn calculate_summary(vulnerabilities: &[ReportVulnerability]) -> ReportSummary {
    let mut summary = ReportSummary::default();

    for vuln in vulnerabilities {
        match vuln.severity {
            ReportSeverity::Critical => summary.critical += 1,
            ReportSeverity::High => summary.high += 1,
            ReportSeverity::Medium => summary.medium += 1,
            ReportSeverity::Low => summary.low += 1,
        }
    }

    summary.unique_count = vulnerabilities.len();
    summary
}
