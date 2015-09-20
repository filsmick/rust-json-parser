extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;


#[test]
fn test_just_one_string_trailing_comma() {
  let input = r##"{
    "a_string": "Hello world!",
}}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));

  assert_eq!(parse_json(input), expected);
}
