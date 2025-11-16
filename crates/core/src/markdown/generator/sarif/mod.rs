mod common_mark;
mod converter;
mod gfm;
mod types;

use crate::markdown::generator::sarif::common_mark::SarifCommonMarkGenerator;
use crate::markdown::generator::sarif::gfm::SarifGitHubFlavoredMarkdownGenerator;
use crate::markdown::generator::{MarkdownFormatFactory, MarkdownGenerator};
use crate::markdown::MarkdownFormat;

pub(crate) struct SarifMarkdownGeneratorFactory;

impl MarkdownFormatFactory for SarifMarkdownGeneratorFactory {
    fn create_generator(
        &self,
        markdown_format: MarkdownFormat,
        with_emoji: bool,
    ) -> Box<dyn MarkdownGenerator> {
        match markdown_format {
            MarkdownFormat::CommonMark => Box::new(SarifCommonMarkGenerator::new(with_emoji)),
            MarkdownFormat::GitHubFlavored => {
                Box::new(SarifGitHubFlavoredMarkdownGenerator::new(with_emoji))
            }
        }
    }
}
