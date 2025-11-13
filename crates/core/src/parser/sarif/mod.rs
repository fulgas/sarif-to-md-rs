use crate::parser::{ParsedReport, Parser, ParserError, ParserType, ParserTypeFactory};
use serde_sarif::sarif::Sarif;

pub(crate) struct SarifParserFactory;

impl ParserTypeFactory for SarifParserFactory {
    fn create_parser(&self, _: ParserType) -> Box<dyn Parser> {
        Box::new(SarifParser)
    }
}

pub(crate) struct SarifParser;
impl Parser for SarifParser {
    fn parse(&self, content: &str) -> Result<ParsedReport, ParserError> {
        let sarif: Sarif = serde_json::from_str(content)?;
        Ok(ParsedReport::Sarif(Box::new(sarif)))
    }
}
