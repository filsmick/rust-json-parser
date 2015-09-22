extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;

#[test]
fn escaped_double_quotes() {
  let input = r##"{"a \"property\"":"a \"string\""}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a \"property\""), JsonValue::String(Cow::Borrowed("a \"string\"")));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn various_escapes() {
  let input = r##"{"abc\n\r\tbcd\r\necc\rf\t\na":"abc\n\r\tbcd\r\necc\rf\t\na"}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("abc\n\r\tbcd\r\necc\rf\t\na"), JsonValue::String(Cow::Borrowed("abc\n\r\tbcd\r\necc\rf\t\na")));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
