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
  current_idx: Cell<usize>,
  current_line: Cell<usize>,
  current_column: Cell<usize>
}

impl<'input> JsonParser<'input> {
  pub fn new(input: &str) -> JsonParser {
    JsonParser {
      input: input,
      remaining_data: Cell::new(input),
      current_idx: Cell::new(0),
      current_line: Cell::new(1), // lines and columns are 1-indexed
      current_column: Cell::new(1)
    }
  }

  fn current_char(&self) -> char {
    self.input.char_at(self.current_idx.get())
  }

  fn next_char(&self) -> Option<char> {
    let next_idx = self.current_idx.get() + 1;
    if next_idx < self.input.len() {
      Some(self.input.char_at(next_idx))
    } else {
      None
    }
  }

  fn next(&self) {
    println!("Current: {}, next: {:?}", self.current_char(), self.next_char());

    let new_idx = self.current_idx.get() + 1;

    if new_idx < self.input.len() {
      self.current_idx.set(new_idx);
      self.remaining_data.set(
        &self.input[self.current_idx.get()..]
      );
    } else {
      println!("Reached end of input"); // XXX: maybe current_char should return an Option.
    }
  }

  fn expect(&self, expected: char) {
    assert_eq!(
      self.current_char(),
      expected
    );
    self.next();
  }

  fn expect_one_of(&self, expected: &[char]) {
    let current_char = self.current_char();
    if !expected.contains(&current_char) {
      panic!("Expected {:?}, found {}", expected, current_char);
    }

    self.next();
  }

  fn expect_optional_whitespace(&self) {
    while self.current_char().is_whitespace() {
      self.next();
    }
  }

  fn read_chars(&self, n: usize) -> &'input str {
    self.current_idx.set(self.current_idx.get() + n);
    &self.remaining_data.get()[..n]
  }

  fn parse_object(&self) -> HashMap<&'input str, JsonValue<'input>> {
    let mut output: HashMap<&str, JsonValue> = HashMap::new();

    self.expect_optional_whitespace();
    self.expect('{');
    self.expect_optional_whitespace();

     loop {
      println!("{:?}", self.current_idx);
      let (property, value) = self.parse_key_value_pair();
      output.insert(property, value);

      self.expect_optional_whitespace();

      match self.current_char() {
        ',' => {
          self.expect(',');
          continue;
        },
        '}' => {
          self.expect('}');
          break;
        },
        c => panic!("Unexpected character '{}' at {}", c, self.current_idx.get()),
      }
    }

    self.expect_optional_whitespace();

    output
  }

  fn parse_key_value_pair(&self) -> (&'input str, JsonValue<'input>) {
    let property_name = self.parse_string();
    println!("Got a property name: '{}'", property_name);

    self.expect_optional_whitespace();
    self.expect(':');
    self.expect_optional_whitespace();
    // TODO whitespace

    let value = self.parse_value();
    println!("Got a value: '{:?}'", value);

    (property_name, value)
  }

  fn parse_value(&self) -> JsonValue<'input> {
    if self.current_char() == '"' {
      JsonValue::String(self.parse_string())
    } else if self.current_char().is_digit(10) || self.current_char() == '-' {
      JsonValue::Number(self.parse_number())
    } else if self.current_char() == 't' || self.current_char() == 'f' {
      JsonValue::Boolean(self.parse_bool())
    } else {
      unimplemented!()
    }
  }

  fn parse_string(&self) -> &'input str {
    self.expect('"');
    println!("remaining '{}'", self.remaining_data.get());

    let idx = self.remaining_data.get().chars().take_while(|c| *c != '"').count();
    let string = self.read_chars(idx);

    println!("idx: {}, current_idx: {}", idx, self.current_idx.get());
    println!("current char: {:?}, next char: {:?}", self.current_char(), self.next_char());

    self.expect('"');

    string
  }

  fn parse_number(&self) -> f64 {
    let idx = self.remaining_data.get().chars().take_while(|c| *c != ',').count();

    let string = self.read_chars(idx);
    string.parse().unwrap()
  }

  fn parse_bool(&self) -> bool {
    match self.current_char() {
      't' => {
        self.expect('t');
        self.expect('r');
        self.expect('u');
        self.expect('e');
        true
      },
      'f' => {
        self.expect('f');
        self.expect('a');
        self.expect('l');
        self.expect('s');
        self.expect('e');
        false
      },
      _ => unreachable!()
    }
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
  Object(HashMap<&'a str, JsonValue<'a>>)
}

#[test]
fn test_just_one_string() {
  let input = r##"{"a_string":"Hello world!"}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));

  assert_eq!(parse_json(input), expected);
}

#[test]
fn test_just_one_string_beautified() {
  let input = r##"{
    "a_string": "Hello world!"
}}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));

  assert_eq!(parse_json(input), expected);
}

// #[test]
// fn test_just_one_string_trailing_comma() {
//   let input = r##"{
//     "a_string": "Hello world!",
// }}"##;
//
//   let mut expected = HashMap::new();
//   expected.insert("a_string", JsonValue::String("Hello world!"));
//
//   assert_eq!(parse_json(input), expected);
// }




#[test]
fn test_bigger_object() {
  let input = r##"{"a_string":"Hello world!","an_integer":17,"a_float":3.14,"a_true_bool":true,"a_false_bool":false}"##;

  let mut expected = HashMap::new();
  expected.insert("a_string", JsonValue::String("Hello world!"));
  expected.insert("an_integer", JsonValue::Number(17.0));
  expected.insert("a_float", JsonValue::Number(3.14));
  expected.insert("a_true_bool", JsonValue::Boolean(true));
  expected.insert("a_false_bool", JsonValue::Boolean(false));

  assert_eq!(parse_json(input), expected);
}
