use std::collections::HashMap;
use std::cell::Cell;
use JsonValue;
use parse_error::*;

pub struct JsonParser<'input> {
  input: &'input str,
  current_idx: Cell<usize>,
}

// Public interface
impl<'input> JsonParser<'input> {
  /// Create a new `JsonParser` with the given input.
  pub fn new(input: &str) -> JsonParser {
    JsonParser {
      input: input,
      current_idx: Cell::new(0),
    }
  }

  /// Parse the input as an array or an object.
  pub fn parse(&self) -> ParseResult<JsonValue<'input>> {
    Ok(try!(self.parse_value()))
  }
}

// Private methods
impl<'input> JsonParser<'input> {
  fn current_char(&self) -> char {
    self.input.char_at(self.current_idx.get())
  }

  fn current_idx(&self) -> usize {
    self.current_idx.get()
  }

  fn remaining_data(&self) -> &'input str {
    &self.input[self.current_idx()..]
  }

  fn next(&self, n: usize) {
    let new_idx = self.current_idx.get() + n;

    if new_idx < self.input.len() {
      self.current_idx.set(new_idx);
    }
  }

  fn expect(&self, expected: char) -> ParseResult<()> {
    let found = self.current_char();

    if found != expected {
      return Err(
        ParseError::new(
          self.input,
          self.current_idx(),
          ParseErrorKind::UnexpectedCharacter(found, expected)
        )
      );
    }

    self.next(1);
    Ok(())
  }

  fn expect_optional_whitespace(&self) {
    while self.current_char().is_whitespace() {
      self.next(1);
    }
  }

  fn parse_object(&self) -> ParseResult<HashMap<&'input str, JsonValue<'input>>> {
    let mut output: HashMap<&str, JsonValue> = HashMap::new();

    self.expect_optional_whitespace();
    try!(self.expect('{'));
    self.expect_optional_whitespace();

    loop {

      let (property, value) = try!(self.parse_key_value_pair());
      output.insert(property, value);

      self.expect_optional_whitespace();

      match self.current_char() {
        ',' => {
          try!(self.expect(','));
          self.expect_optional_whitespace();

          match self.current_char() {
            '}' => {
              try!(self.expect('}'));
              break;
            },
            _ => {
              continue;
            },
          }
        },
        '}' => {
          try!(self.expect('}'));
          break;
        },
        c => panic!("Unexpected character '{}' at {}", c, self.current_idx.get()),
      }
    }

    self.expect_optional_whitespace();

    Ok(output)
  }

  fn parse_array(&self) -> ParseResult<Vec<JsonValue<'input>>> {
    let mut output = Vec::with_capacity(2);

    self.expect_optional_whitespace();
    try!(self.expect('['));
    self.expect_optional_whitespace();

    loop {
      let value = try!(self.parse_value());
      output.push(value);

      self.expect_optional_whitespace();

      match self.current_char() {
        ',' => {
          try!(self.expect(','));
          self.expect_optional_whitespace();

          match self.current_char() {
            ']' => {
              try!(self.expect(']'));
              break;
            },
            _ => {
              continue;
            },
          }
        },
        ']' => {
          try!(self.expect(']'));
          break;
        },
        c => panic!("Unexpected character '{}' at {}", c, self.current_idx.get()),
      }
    }

    Ok(output)
  }

  fn parse_key_value_pair(&self) -> ParseResult<(&'input str, JsonValue<'input>)> {
    let property_name = try!(self.parse_string());

    self.expect_optional_whitespace();
    try!(self.expect(':'));
    self.expect_optional_whitespace();
    let value = try!(self.parse_value());
    self.expect_optional_whitespace();

    Ok((property_name, value))
  }

  fn parse_value(&self) -> ParseResult<JsonValue<'input>> {
    Ok(
      match self.current_char() {
        '"' => JsonValue::String(try!(self.parse_string())),
        '{' => JsonValue::Object(try!(self.parse_object())),
        '[' => JsonValue::Array(try!(self.parse_array())),
        c if c.is_digit(10) || c == '-' => JsonValue::Number(try!(self.parse_number())),
        't' | 'f' => JsonValue::Boolean(try!(self.parse_bool())),
        'n' => {
          try!(self.expect('n'));
          try!(self.expect('u'));
          try!(self.expect('l'));
          try!(self.expect('l'));
          JsonValue::Null
        }
        _ => unimplemented!()
      }
    )
  }

  fn parse_string(&self) -> ParseResult<&'input str> {
    try!(self.expect('"'));

    let string_start_idx = self.current_idx();
    while self.current_char() != '"' {
      self.next(1);
    }
    let string_end_idx = self.current_idx();

    let string = &self.input[string_start_idx..string_end_idx];

    try!(self.expect('"'));

    Ok(string)
  }

  fn parse_number(&self) -> ParseResult<f64> {
    /*
           end of integer part
           |
    -101654.79
    ^         ^
    |         | end of decimal part
    |
    | start of integer part
    */

    let integer_part_start: usize = self.current_idx.get();

    if self.current_char() == '-' {
      self.next(1);
    }

    while self.current_char().is_digit(10) {
      self.next(1);
    }

    let mut decimal_part_end: usize = self.current_idx.get();

    if self.current_char() == '.' {
      self.next(1);
      decimal_part_end += 1;
      while self.current_char().is_digit(10) {
        self.next(1);
        decimal_part_end += 1;
      }
    }

    let string = &self.input[integer_part_start..decimal_part_end];

    Ok(string.parse().unwrap()) // it is unclear whether this can fail
  }

  fn parse_bool(&self) -> ParseResult<bool> {
    match self.current_char() {
      't' => {
        try!(self.expect('t'));
        try!(self.expect('r'));
        try!(self.expect('u'));
        try!(self.expect('e'));
        Ok(true)
      },
      'f' => {
        try!(self.expect('f'));
        try!(self.expect('a'));
        try!(self.expect('l'));
        try!(self.expect('s'));
        try!(self.expect('e'));
        Ok(false)
      },
      _ => unreachable!()
    }
  }
}
