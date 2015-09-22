extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;

#[test]
fn hebrew_russian_ascii() {
  let input = r##"{"привет world":"שלום привет hello"}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("привет world"), JsonValue::String(Cow::Borrowed("שלום привет hello")));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
