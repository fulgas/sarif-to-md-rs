mod code;
mod container;

use crate::parser::json::code::SnykCodeParser;
use crate::parser::json::container::SnykContainerParser;
use crate::parser::{Parser, ParserType, ParserTypeFactory};

pub(crate) struct JsonParserFactory;

impl ParserTypeFactory for JsonParserFactory {
    fn create_parser(&self, parser_type: ParserType) -> Box<dyn Parser> {
        match parser_type {
            ParserType::Container => Box::new(SnykContainerParser),
            ParserType::Code => Box::new(SnykCodeParser),
        }
    }
}
