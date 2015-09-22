extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;

#[test]
fn hebrew_russian_ascii() {
  let input = r##"{"a_non_ascii_string":"שלום привет hello"}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a_non_ascii_string"), JsonValue::String(Cow::Borrowed("שלום привет hello")));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
