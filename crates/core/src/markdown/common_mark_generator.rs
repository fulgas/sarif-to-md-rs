use crate::markdown::{GeneratorError, MarkdownGenerator};
use crate::model::security_report::{ProjectType, SecurityReport, Severity};
use askama::Template;
use std::fmt;

#[derive(Template)]
#[template(path = "report.md")]
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

impl From<ProjectType> for ReportProjectType {
    fn from(value: ProjectType) -> Self {
        match value {
            ProjectType::DockerImage => ReportProjectType::DockerImage,
            ProjectType::Application => ReportProjectType::Application,
        }
    }
}

#[derive(Clone)]
struct ReportSummary {
    critical: usize,
    high: usize,
    medium: usize,
    low: usize,
    unique_count: usize,
}

#[derive(PartialEq)]
enum ReportSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl From<Severity> for ReportSeverity {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Critical => ReportSeverity::Critical,
            Severity::High => ReportSeverity::High,
            Severity::Medium => ReportSeverity::Medium,
            Severity::Low => ReportSeverity::Low,
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
    from_paths: Vec<String>,
}

pub(crate) struct CommonMarkGenerator {
    with_emoji: bool,
}

impl CommonMarkGenerator {
    pub(crate) fn new(with_emoji: bool) -> Box<dyn MarkdownGenerator> {
        Box::new(Self { with_emoji })
    }
}

impl MarkdownGenerator for CommonMarkGenerator {
    fn generate_markdown_report(&self, report: &SecurityReport) -> Result<String, GeneratorError> {
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let projects = report
            .projects
            .iter()
            .map(|project| ReportProject {
                name: project.name.clone(),
                organization: project.organization.clone(),
                project_type: project.project_type.clone().into(),
                target_file: project.target_file.clone(),
                summary: ReportSummary {
                    critical: project.summary.critical,
                    high: project.summary.high,
                    medium: project.summary.medium,
                    low: project.summary.low,
                    unique_count: project.summary.unique_count,
                },
                vulnerabilities: map_vulnerabilities(&project.vulnerabilities),
            })
            .collect();

        let template_ctx = ReportTemplate {
            projects,
            timestamp,
            with_emoji: self.with_emoji,
        };

        template_ctx.render().map_err(GeneratorError::AskamaError)
    }
}

fn map_vulnerabilities(
    vulns: &[crate::model::security_report::Vulnerability],
) -> Vec<ReportVulnerability> {
    vulns
        .iter()
        .map(|v| {
            let from_paths: Vec<String> =
                v.from_paths.iter().map(|path| path.join(" → ")).collect();

            ReportVulnerability {
                id: v.id.clone(),
                title: v.title.clone(),
                severity: v.severity.clone().into(),
                package_name: v.package_name.clone(),
                version: v.version.clone(),
                cvss_score: v.cvss_score.clone(),
                is_upgradable: v.is_upgradable,
                is_patchable: v.is_patchable,
                cve_ids: v.cve_ids.clone(),
                from_paths,
            }
        })
        .collect()
}
