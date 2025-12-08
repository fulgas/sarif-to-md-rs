use crate::error::{BuilderError, Error};
use crate::markdown::MarkdownGenerator;

pub mod error;
pub mod generators;
pub mod markdown;

pub struct ReportProcessor<G: MarkdownGenerator> {
    generator: G,
    content: String,
}

impl<G: MarkdownGenerator> ReportProcessor<G> {
    pub fn new(generator: G, content: String) -> Self {
        Self { generator, content }
    }

    pub fn generate(self) -> Result<String, Error> {
        let data: G::Input = serde_json::from_str(&self.content)?;
        let markdown = self.generator.generate_markdown_template(&data)?;
        Ok(markdown)
    }
}

pub struct ReportProcessorBuilder<G = ()> {
    generator: G,
    content: Option<String>,
}

impl ReportProcessorBuilder<()> {
    pub fn new() -> Self {
        Self {
            generator: (),
            content: None,
        }
    }
}

impl Default for ReportProcessorBuilder<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportProcessorBuilder<()> {
    pub fn generator<G: MarkdownGenerator>(self, generator: G) -> ReportProcessorBuilder<G> {
        ReportProcessorBuilder {
            generator,
            content: self.content,
        }
    }
}

impl<G: MarkdownGenerator> ReportProcessorBuilder<G> {
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
    pub fn build(self) -> Result<ReportProcessor<G>, BuilderError> {
        let content = self.content.ok_or(BuilderError::MissingContent)?;

        Ok(ReportProcessor::new(self.generator, content))
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    fn public_api() -> Result<(), Box<dyn std::error::Error>> {
        rustup_toolchain::install(public_api::MINIMUM_NIGHTLY_RUST_VERSION)?;

        let rustdoc_json = rustdoc_json::Builder::default()
            .toolchain(public_api::MINIMUM_NIGHTLY_RUST_VERSION)
            .build()?;

        let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json).build()?;

        insta::assert_snapshot!(public_api);
        Ok(())
    }
}
