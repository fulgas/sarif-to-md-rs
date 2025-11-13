use crate::parser::json::JsonParserFactory;
use crate::parser::sarif::SarifParserFactory;
use anyhow::Result;

pub(crate) mod json;
pub(crate) mod sarif;

#[derive(Clone, Debug)]
pub enum ParserFormat {
    Json,
    Sarif,
}

#[derive(Clone, Debug)]
pub enum ParserType {
    Container,
    Code,
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Failed to parse Snyk report JSON")]
    JsonError(#[from] serde_json::Error),
}

pub enum ParsedReport {
    Container(Box<serde_snyk_container::SnykContainer>),
    Code(String),
    Sarif(Box<serde_sarif::sarif::Sarif>),
}

pub(crate) trait Parser {
    fn parse(&self, content: &str) -> Result<ParsedReport, ParserError>;
}

pub(crate) struct ParserFormatFactory;

impl ParserFormatFactory {
    pub(crate) fn create_parser_format(parser_format: &ParserFormat) -> Box<dyn ParserTypeFactory> {
        match parser_format {
            ParserFormat::Json => Box::new(JsonParserFactory),
            ParserFormat::Sarif => Box::new(SarifParserFactory),
        }
    }
}

pub(crate) trait ParserTypeFactory {
    fn create_parser(&self, parser_type: ParserType) -> Box<dyn Parser>;
}
