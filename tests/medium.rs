extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;


#[test]
fn just_one_string_trailing_comma() {
  let input = r##"{
    "a_string": "Hello world!",
}}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));

  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input), expected);
}

#[test]
fn bigger_object() {
  let input = r##"{"a_string":"Hello world!","an_integer":17,"a_float":3.14,"a_true_bool":true,"a_false_bool":false}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));
  expected.insert("an_integer", JsonValue::Number(17.0));
  expected.insert("a_float", JsonValue::Number(3.14));
  expected.insert("a_true_bool", JsonValue::Boolean(true));
  expected.insert("a_false_bool", JsonValue::Boolean(false));

  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input), expected);
}

#[test]
fn simple_composite_array() {
  let input = r##"[true, 1, 0, 17.9, "A string"]"##;

  let mut expected = Vec::new();
  expected.push(JsonValue::Boolean(true));
  expected.push(JsonValue::Number(1.0));
  expected.push(JsonValue::Number(0.0));
  expected.push(JsonValue::Number(17.9));
  expected.push(JsonValue::String("A string"));

  let expected = JsonValue::Array(expected);

  assert_eq!(parse_json(input), expected);
}
