use crate::parser::{ParsedReport, Parser, ParserError};

pub(crate) struct SnykContainerParser;

impl Parser for SnykContainerParser {
    fn parse(&self, content: &str) -> Result<ParsedReport, ParserError> {
        Ok(ParsedReport::Container(Box::new(serde_json::from_str(
            content,
        )?)))
    }
}
