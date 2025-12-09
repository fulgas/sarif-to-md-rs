use crate::error::GeneratorError;
use serde::de::DeserializeOwned;

pub mod sarif;

/// Supported Markdown output formats.
///
/// Determines the style and features available in the generated Markdown output.
/// Choose the format that best matches your target platform or renderer.
#[derive(Debug, Clone, Copy)]
pub enum MarkdownFormat {
    /// Standard CommonMark format for maximum compatibility.
    ///
    /// Produces clean, portable Markdown that works with any CommonMark-compliant
    /// renderer. Uses standard Markdown syntax without HTML extensions.
    ///
    /// Best for: Documentation sites, static site generators, general purpose output.
    CommonMark,

    /// GitHub Flavored Markdown with HTML extensions.
    ///
    /// Uses HTML `<details>` tags for collapsible sections and other GitHub-specific
    /// features for enhanced readability in GitHub PRs, issues, and README files.
    ///
    /// Best for: GitHub repositories, pull request comments, issue reports.
    GitHubFlavored,
}

/// Trait for converting structured data into Markdown.
///
/// Implement this trait to create custom generators for different report formats.
pub trait MarkdownGenerator {
    /// The input data type this generator can process.
    type Input: DeserializeOwned;

    /// Generate Markdown from the input data.
    fn generate_markdown_template(&self, report: &Self::Input) -> Result<String, GeneratorError>;
}
