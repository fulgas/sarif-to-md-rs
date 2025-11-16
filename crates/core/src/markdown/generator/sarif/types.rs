use askama::Template;
use std::fmt;

#[derive(Template)]
#[template(path = "sarif/report.md")]
pub(super) struct SarifReportTemplate {
    pub(super) runs: Vec<SarifRun>,
    pub(super) timestamp: String,
    pub(super) with_emoji: bool,
    pub(super) is_gfm: bool,
}

#[derive(Clone, Debug)]
pub(super) struct SarifRun {
    pub(super) tool_name: String,
    pub(super) tool_version: Option<String>,
    pub(super) total_results: usize,
    pub(super) severity_counts: Vec<SeverityCount>,
    pub(super) results: Vec<SarifResultView>,
}

#[derive(Clone, Debug)]
pub(super) struct SeverityCount {
    pub(super) level: SarifLevel,
    pub(super) count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) enum SarifLevel {
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
pub(super) struct SarifResultView {
    pub(super) rule_id: String,
    pub(super) level: SarifLevel,
    pub(super) message: String,
    pub(super) locations: Vec<SarifLocation>,
    pub(super) rule_metadata: Option<RuleMetadata>,
}

#[derive(Clone, Debug)]
pub(super) struct RuleMetadata {
    pub(super) name: Option<String>,
    pub(super) description: Option<String>,
    pub(super) help_uri: Option<String>,
    pub(super) cwe_ids: Vec<String>,
    pub(super) tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub(super) struct SarifLocation {
    pub(super) file: Option<String>,
    pub(super) line: Option<i64>,
    pub(super) column: Option<i64>,
}
