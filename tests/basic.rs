extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;


#[test]
fn just_one_string() {
  let input = r##"{"a_string":"Hello world!"}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn just_one_string_beautified() {
  let input = r##"{
    "a_string": "Hello world!"
}}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn null() {
  let input = r##"{
    "a_null_property": null
}}"##;

  let mut expected = HashMap::new();
  expected.insert("a_null_property", JsonValue::Null);
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
