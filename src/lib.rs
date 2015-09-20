#![feature(str_char)]

mod json_value;
pub use json_value::JsonValue;

mod parser;
pub use parser::JsonParser;

pub fn parse_json(input: &str) -> JsonValue {
  let parser = JsonParser::new(input);
  parser.parse()
}
