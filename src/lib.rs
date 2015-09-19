#![allow(dead_code)]
#![feature(str_char)]
use std::collections::HashMap;
use std::cell::Cell;

// fn is_token(c: char) -> bool {
//   match c {
//     'a'..'z' => true,
//     'A'..'Z' => true,
//     '_' => true,
//     _ => false
//   }
// }
//
// #[test]
// fn test_is_token() {
//   assert_eq!(is_token('a'), true);
//   assert_eq!(is_token('_'), true);
//   assert_eq!(is_token('v'), true);
//   assert_eq!(is_token('z'), true);
//   assert_eq!(is_token('u'), true);
//   assert_eq!(is_token('/'), false);
//   assert_eq!(is_token('-'), false);
//   assert_eq!(is_token('"'), false);
//   assert_eq!(is_token('~'), false);
// }


struct JsonParser<'input> {
  input: &'input str,
  remaining_data: Cell<&'input str>,
  current_idx: Cell<usize>
}

impl<'input> JsonParser<'input> {
  pub fn new(input: &str) -> JsonParser {
    JsonParser {
      input: input,
      remaining_data: Cell::new(input),
      current_idx: Cell::new(0)
    }
  }

  fn current_char(&self) -> char {
    self.input.char_at(self.current_idx.get())
  }

  fn next_char(&self) -> char {
    self.input.char_at(self.current_idx.get() + 1)
  }

  fn next(&self) {
    self.current_idx.set(self.current_idx.get() + 1);
    self.remaining_data.set(
      &self.input[self.current_idx.get()..]
    );
  }

  fn expect(&self, expected: char) {
    //println!("Expecting: {}", expected);
    assert_eq!(
      self.input.char_at(self.current_idx.get()),
      expected
    );
    self.next();
  }

  fn parse_object(&self) -> HashMap<&'input str, JsonValue<'input>> {
    let mut output: HashMap<&str, JsonValue> = HashMap::new();

    self.expect('{');

    while self.current_char() != '}' {
      let (property, value) = self.parse_key_value_pair();
      output.insert(property, value);
    }

    self.expect('}');

    output
  }

  fn parse_key_value_pair(&self) -> (&'input str, JsonValue<'input>) {
    let property_name = self.parse_string();
    println!("Got a property name: '{}'", property_name);

    self.expect(':');
    // TODO whitespace

    let value = self.parse_value();
    println!("Got a value: '{:?}'", value);

    (property_name, value)
  }

  fn parse_value(&self) -> JsonValue<'input> {
    if self.next_char() == '"' {
      JsonValue::String(self.parse_string())
    } else {
      unimplemented!()
    }
  }

  fn parse_string(&self) -> &'input str {
    self.expect('"');
    println!("remaining '{}'", self.remaining_data.get());

    let idx = self.remaining_data.get().chars().take_while(|c| *c != '"').count();

    let slice = &self.remaining_data.get()[..idx];
    self.current_idx.set(self.current_idx.get() + idx);
    println!("idx: {}", idx);

    self.expect('"');

    slice
  }
}


fn parse_json(input: &str) -> HashMap<&str, JsonValue> {
  let parser = JsonParser::new(input);
  parser.parse_object()
}

// fn parse_object(input: Chars) -> HashMap<&str, JsonValue> {
//   if input.next()
//
//   unimplemented!()
// }

#[derive(PartialEq, Debug)]
enum JsonValue<'a> {
  String(&'a str),
  Number(f64),
  Boolean(bool),
  // we don't handle nested objects for now
}

#[test]
fn test_just_one_string_minified() {
  let input = r##"{"a_string":"Hello world!"}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));

  let result = parse_json(input);

  assert_eq!(result, expected);
}



#[test]
fn test_bigger_object_minified() {
  let input = r##"{"a_string":"Hello world!","an_integer": 17,"a_float": 3.14,"a_true_bool": true,"a_false_bool": false}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));
  expected.insert("an_integer", JsonValue::Number(17.0));
  expected.insert("a_float", JsonValue::Number(3.14));
  expected.insert("a_true_bool", JsonValue::Boolean(true));
  expected.insert("a_false_bool", JsonValue::Boolean(false));

  assert_eq!(parse_json(input), expected);
}
