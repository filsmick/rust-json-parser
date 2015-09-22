extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;


#[test]
fn just_one_string() {
  let input = r##"{"a_string":"Hello world!"}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a_string"), JsonValue::String(Cow::Borrowed("Hello world!")));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn just_one_string_beautified() {
  let input = r##"{
    "a_string": "Hello world!"
}}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a_string"), JsonValue::String(Cow::Borrowed("Hello world!")));
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn null() {
  let input = r##"{
    "a_null_property": null
}}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a_null_property"), JsonValue::Null);
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
