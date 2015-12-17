mod parse_error;
pub use parse_error::{ParseResult, ParseError};

mod json_value;
pub use json_value::JsonValue;

mod parser;
pub use parser::JsonParser;

pub fn parse_json(input: &str) -> ParseResult<JsonValue> {
    let parser = JsonParser::new(input);
    parser.parse()
}
