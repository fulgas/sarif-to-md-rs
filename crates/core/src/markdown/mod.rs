use crate::markdown::common_mark_generator::CommonMarkGenerator;
use crate::markdown::gfm_generator::GitFlavouredMarkdownGenerator;
use crate::model::security_report::SecurityReport;
mod common_mark_generator;
mod gfm_generator;

#[allow(dead_code)]
mod badge;

#[derive(Debug, Clone, Copy)]
pub enum MarkdownFormat {
    CommonMark,
    GitHubFlavored,
}

#[derive(Debug, thiserror::Error)]
pub enum GeneratorError {
    #[error("Failed to generate markdown report from template")]
    AskamaError(#[from] askama::Error),
}

pub(crate) trait MarkdownGenerator {
    fn generate_markdown_report(
        &self,
        report: &SecurityReport,
    ) -> anyhow::Result<String, GeneratorError>;
}

pub(crate) struct MarkdownGeneratorFactory;

impl MarkdownGeneratorFactory {
    pub fn create_generator(
        format: MarkdownFormat,
        with_emoji: bool,
    ) -> Box<dyn MarkdownGenerator> {
        match format {
            MarkdownFormat::CommonMark => CommonMarkGenerator::new(with_emoji),
            MarkdownFormat::GitHubFlavored => Box::new(GitFlavouredMarkdownGenerator),
        }
    }
}
