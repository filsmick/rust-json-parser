extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;
use std::borrow::Cow;

#[test]
fn just_one_string_trailing_comma() {
  let input = r##"{
    "a_string": "Hello world!",
}}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a_string"), JsonValue::String(Cow::Borrowed("Hello world!")));

  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn bigger_object() {
  let input = r##"{"a_string":"Hello world!","an_integer":17,"a_float":3.14,"a_true_bool":true,"a_false_bool":false}"##;

  let mut expected = HashMap::new();
  expected.insert(Cow::Borrowed("a_string"), JsonValue::String(Cow::Borrowed("Hello world!")));
  expected.insert(Cow::Borrowed("an_integer"), JsonValue::Number(17.0));
  expected.insert(Cow::Borrowed("a_float"), JsonValue::Number(3.14));
  expected.insert(Cow::Borrowed("a_true_bool"), JsonValue::Boolean(true));
  expected.insert(Cow::Borrowed("a_false_bool"), JsonValue::Boolean(false));

  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn simple_composite_array() {
  let input = r##"[true, 1, 0, 17.9, "A string"]"##;

  let mut expected = Vec::new();
  expected.push(JsonValue::Boolean(true));
  expected.push(JsonValue::Number(1.0));
  expected.push(JsonValue::Number(0.0));
  expected.push(JsonValue::Number(17.9));
  expected.push(JsonValue::String(Cow::Borrowed("A string")));

  let expected = JsonValue::Array(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
