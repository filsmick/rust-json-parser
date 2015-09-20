extern crate json_parser;
use json_parser::*;
use std::collections::HashMap;

#[test]
fn nested_object_simple() {
  let input = r##"{"object_name":{"prop_name": "value in nested object"}}"##;

  let expected = {
    let mut nested_obj = HashMap::new();
    nested_obj.insert("prop_name", JsonValue::String("value in nested object"));

    let mut top_level_obj = HashMap::new();
    top_level_obj.insert("object_name", JsonValue::Object(nested_obj));

    top_level_obj
  };
  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
fn bigger_object_nested_beautified_trailing_comma() {
  let input = r##"{
    "a_string": "Hello world!",
    "an_integer": 17,
    "a_float": 3.26,
    "a_true_bool": true,
    "a_false_bool": false,
    "a_nested_object": {
        "another_nested_object": {
            "a_deeply_nested_property": 45.89
        }
    }
}"##;


  let expected = {
    let mut second_nested_obj = HashMap::new();
    second_nested_obj.insert("a_deeply_nested_property", JsonValue::Number(45.89));

    let mut nested_obj = HashMap::new();
    nested_obj.insert("another_nested_object", JsonValue::Object(second_nested_obj));

    let mut top_level_obj = HashMap::new();
    top_level_obj.insert("a_string", JsonValue::String("Hello world!"));
    top_level_obj.insert("an_integer", JsonValue::Number(17.0));
    top_level_obj.insert("a_float", JsonValue::Number(3.26));
    top_level_obj.insert("a_true_bool", JsonValue::Boolean(true));
    top_level_obj.insert("a_false_bool", JsonValue::Boolean(false));
    top_level_obj.insert("a_nested_object", JsonValue::Object(nested_obj));

    top_level_obj
  };

  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}

#[test]
#[should_panic]
fn invalid_bigger_object_nested_beautified_double_comma() {
  let input = r##"{
    "a_string": "Hello world!",
    "an_integer": 17,
    "a_float": 3.14,
    "a_true_bool": true,,
    "a_false_bool": false,
    "a_nested_object": {
        "another_nested_object": {
            "a_deeply_nested_property": 45.89
        }
    }
}"##;


  let expected = {
    let mut second_nested_obj = HashMap::new();
    second_nested_obj.insert("a_deeply_nested_property", JsonValue::Number(45.89));

    let mut nested_obj = HashMap::new();
    nested_obj.insert("another_nested_object", JsonValue::Object(second_nested_obj));

    let mut top_level_obj = HashMap::new();
    top_level_obj.insert("a_string", JsonValue::String("Hello world!"));
    top_level_obj.insert("an_integer", JsonValue::Number(17.0));
    top_level_obj.insert("a_float", JsonValue::Number(3.14));
    top_level_obj.insert("a_true_bool", JsonValue::Boolean(true));
    top_level_obj.insert("a_false_bool", JsonValue::Boolean(false));
    top_level_obj.insert("a_nested_object", JsonValue::Object(nested_obj));

    top_level_obj
  };

  let expected = JsonValue::Object(expected);

  assert_eq!(parse_json(input).unwrap(), expected);
}
