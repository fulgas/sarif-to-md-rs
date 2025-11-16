use super::converter::convert_sarif_to_view;
use crate::markdown::generator::sarif::types::SarifReportTemplate;
use crate::markdown::generator::{GeneratorError, MarkdownGenerator};
use crate::parser::ParsedReport;
use askama::Template;

pub(crate) struct SarifCommonMarkGenerator {
    with_emoji: bool,
}

impl SarifCommonMarkGenerator {
    pub(crate) fn new(with_emoji: bool) -> Self {
        Self { with_emoji }
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

        let runs = convert_sarif_to_view(sarif);
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let template = SarifReportTemplate {
            runs,
            timestamp,
            with_emoji: self.with_emoji,
            is_gfm: false,
        };

        template.render().map_err(GeneratorError::AskamaError)
    }
}
