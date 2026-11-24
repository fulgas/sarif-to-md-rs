use askama::Template;
use std::fmt;

#[derive(Template)]
#[template(path = "sarif/report.md")]
pub(in crate::markdown) struct SarifReportTemplate {
    pub(in crate::markdown) runs: Vec<SarifRun>,
    pub(in crate::markdown) timestamp: String,
    pub(in crate::markdown) with_emoji: bool,
    pub(in crate::markdown) is_gfm: bool,
}

#[derive(Clone, Debug)]
pub(in crate::markdown) struct SarifRun {
    pub(in crate::markdown) tool_name: String,
    pub(in crate::markdown) tool_version: Option<String>,
    pub(in crate::markdown) total_results: usize,
    pub(in crate::markdown) suppressed_count: usize,
    pub(in crate::markdown) include_suppressed: bool,
    pub(in crate::markdown) severity_counts: Vec<SeverityCount>,
    pub(in crate::markdown) results: Vec<SarifResultView>,
}

#[derive(Clone, Debug)]
pub(in crate::markdown) struct SeverityCount {
    pub(in crate::markdown) level: SarifLevel,
    pub(in crate::markdown) count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(in crate::markdown) enum SarifLevel {
    Error,
    Warning,
    Note,
    None,
}

impl fmt::Display for SarifLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SarifLevel::Error => write!(f, "Error"),
            SarifLevel::Warning => write!(f, "Warning"),
            SarifLevel::Note => write!(f, "Note"),
            SarifLevel::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Debug)]
pub(in crate::markdown) struct SarifResultView {
    pub(in crate::markdown) rule_id: String,
    pub(in crate::markdown) level: SarifLevel,
    pub(in crate::markdown) message: String,
    pub(in crate::markdown) locations: Vec<SarifLocation>,
    pub(in crate::markdown) rule_metadata: Option<RuleMetadata>,
    pub(in crate::markdown) properties: Option<ResultProperties>,
    pub(in crate::markdown) suppressed: bool,
    pub(in crate::markdown) suppressions: Vec<String>,
}

#[derive(Clone, Debug)]
pub(in crate::markdown) struct RuleMetadata {
    pub(in crate::markdown) name: Option<String>,
    pub(in crate::markdown) description: Option<String>,
    pub(in crate::markdown) help_uri: Option<String>,
    pub(in crate::markdown) tags: Vec<String>,
    pub(in crate::markdown) cwe_ids: Vec<String>,
    pub(in crate::markdown) cve_ids: Vec<String>,
}

#[derive(Clone, Debug)]
pub(in crate::markdown) struct ResultProperties {
    pub(in crate::markdown) issue_confidence: Option<String>,
    pub(in crate::markdown) precision: Option<String>,
    pub(in crate::markdown) problem_severity: Option<String>,
    #[allow(dead_code)]
    pub(in crate::markdown) security_severity: Option<String>,
    pub(in crate::markdown) custom_fields: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub(in crate::markdown) struct SarifLocation {
    pub(in crate::markdown) file: Option<String>,
    pub(in crate::markdown) line: Option<i64>,
    pub(in crate::markdown) column: Option<i64>,
}
