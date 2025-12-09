//! Convert SARIF security reports to Markdown with customizable output formats.
//!
//! ```rust
//! use sarif_to_md_core::{
//!     ReportProcessorBuilder,
//!     generators::SarifMarkdownGenerator,
//!     markdown::MarkdownFormat,
//! };
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let sarif_json = r#"{"version": "2.1.0", "runs": []}"#;
//! let processor = ReportProcessorBuilder::new()
//!     .generator(SarifMarkdownGenerator::new(MarkdownFormat::GitHubFlavored, true))
//!     .content(sarif_json.to_string())
//!     .build()?;
//!
//! let markdown = processor.generate()?;
//! # Ok(())
//! # }
//! ```

use crate::error::{BuilderError, Error};
use crate::markdown::MarkdownGenerator;

pub mod error;
pub mod generators;
pub mod markdown;

/// Processes reports into Markdown using configurable generators.
pub struct ReportProcessor<G: MarkdownGenerator> {
    generator: G,
    content: String,
}

impl<G: MarkdownGenerator> ReportProcessor<G> {
    /// Create a processor with the specified generator and JSON content.
    pub fn new(generator: G, content: String) -> Self {
        Self { generator, content }
    }

    /// Parse content and generate Markdown output.
    pub fn generate(self) -> Result<String, Error> {
        let data: G::Input = serde_json::from_str(&self.content)?;
        let markdown = self.generator.generate_markdown_template(&data)?;
        Ok(markdown)
    }
}

/// Builder for configuring [`ReportProcessor`] instances using type-state pattern.
pub struct ReportProcessorBuilder<G = ()> {
    generator: G,
    content: Option<String>,
}

impl ReportProcessorBuilder<()> {
    /// Create a new builder instance.
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
    /// Set the Markdown generator to use.
    pub fn generator<G: MarkdownGenerator>(self, generator: G) -> ReportProcessorBuilder<G> {
        ReportProcessorBuilder {
            generator,
            content: self.content,
        }
    }
}

impl<G: MarkdownGenerator> ReportProcessorBuilder<G> {
    /// Set the JSON report content to process.
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Build the configured processor.
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
