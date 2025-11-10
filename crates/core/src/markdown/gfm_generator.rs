use crate::markdown::{GeneratorError, MarkdownGenerator};
use crate::model::security_report::SecurityReport;

pub(crate) struct GitFlavouredMarkdownGenerator;

impl MarkdownGenerator for GitFlavouredMarkdownGenerator {
    fn generate_markdown_report(
        &self,
        _: &SecurityReport,
    ) -> anyhow::Result<String, GeneratorError> {
        todo!()
    }
}
