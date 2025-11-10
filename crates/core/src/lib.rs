use crate::markdown::{MarkdownFormat, MarkdownGeneratorFactory};
use crate::parser::{ParserFactory, ParserType};

pub mod markdown;
mod model;
pub mod parser;

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("JSON content was not provided")]
    MissingContent,
    #[error("Parser type was not specified")]
    MissingParserType,
    #[error("Markdown format was not specified")]
    MissingMarkdownFormat,
    #[error(transparent)]
    ParseError(#[from] parser::ParserError),
    #[error(transparent)]
    GeneratedReportError(#[from] markdown::GeneratorError),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] parser::ParserError),
    #[error(transparent)]
    GeneratedReportError(#[from] markdown::GeneratorError),
}

pub struct ReportProcessor {
    parser_type: ParserType,
    content: String,
    markdown_format: MarkdownFormat,
    with_emoji: bool,
}

impl ReportProcessor {
    pub fn generate(self) -> anyhow::Result<String, Error> {
        let parser = ParserFactory::create_parser(self.parser_type);

        let content = parser.parse(&self.content)?;

        let generator =
            MarkdownGeneratorFactory::create_generator(self.markdown_format, self.with_emoji);

        let markdown_report = generator.generate_markdown_report(&content)?;

        Ok(markdown_report)
    }
}

pub struct ReportProcessorBuilder {
    parser_type: Option<ParserType>,
    content: Option<String>,
    markdown_format: Option<MarkdownFormat>,
    with_emoji: bool,
}

impl ReportProcessorBuilder {
    pub fn new() -> Self {
        Self {
            parser_type: None,
            content: None,
            markdown_format: None,
            with_emoji: false,
        }
    }

    pub fn parser_type(mut self, parser_type: ParserType) -> Self {
        self.parser_type = Some(parser_type);
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn markdown_format(mut self, markdown_format: MarkdownFormat) -> Self {
        self.markdown_format = Some(markdown_format);
        self
    }

    pub fn with_emoji(mut self, with_emoji: bool) -> Self {
        self.with_emoji = with_emoji;
        self
    }

    pub fn build(self) -> Result<ReportProcessor, BuilderError> {
        Ok(ReportProcessor {
            parser_type: self.parser_type.ok_or(BuilderError::MissingParserType)?,
            content: self.content.ok_or(BuilderError::MissingContent)?,
            markdown_format: self
                .markdown_format
                .ok_or(BuilderError::MissingMarkdownFormat)?,
            with_emoji: self.with_emoji,
        })
    }
}
