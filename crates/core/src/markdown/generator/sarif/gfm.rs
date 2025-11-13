use crate::markdown::generator::{GeneratorError, MarkdownGenerator};
use crate::parser::ParsedReport;

pub(crate) struct SarifGitHubFlavoredMarkdownGenerator {
    with_emoji: bool,
}

impl SarifGitHubFlavoredMarkdownGenerator {
    pub(crate) fn new(with_emoji: bool) -> Self {
        Self { with_emoji }
    }
}

impl MarkdownGenerator for SarifGitHubFlavoredMarkdownGenerator {
    fn generate_markdown_template(
        &self,
        parsed_report: &ParsedReport,
    ) -> Result<String, GeneratorError> {
        // For now, GitHub Flavored Markdown uses the same template as CommonMark
        // In the future, we could create a separate GFM template with GitHub-specific features
        use super::common_mark::SarifCommonMarkGenerator;

        let generator = SarifCommonMarkGenerator::new(self.with_emoji);
        generator.generate_markdown_template(parsed_report)
    }
}
