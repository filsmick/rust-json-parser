use std::collections::HashMap;
use std::cell::Cell;
use JsonValue;
use parse_error::*;

pub struct JsonParser<'input> {
  input: &'input str,
  current_idx: Cell<usize>,
}

/* Public interface */
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

/* Utility functions */
fn is_digit(b: u8) -> bool {
  match b {
    b'0'...b'9' => true,
    _ => false
  }
}

fn is_whitespace(b: u8) -> bool {
  match b {
    b' ' | b'\n' | b'\r' | b'\t' => true,
    _ => false
  }
}

/* Private methods */
impl<'input> JsonParser<'input> {
  fn current_byte(&self) -> Option<u8> {
    if self.current_idx() < self.input.len() {
      Some(self.input.as_bytes()[self.current_idx()])
    } else {
      None
    }
  }

  fn current_idx(&self) -> usize {
    self.current_idx.get()
  }

  fn next(&self, n: usize) {
    let new_idx = self.current_idx() + n;

    self.current_idx.set(new_idx);
  }

  fn expect(&self, expected: u8) -> ParseResult<()> {
    let found = self.current_byte().unwrap();

    if found != expected {
      return Err(
        ParseError::new(
          self.input,
          self.current_idx(),
          ParseErrorKind::UnexpectedCharacter(found, vec![expected])
        )
      );
    }

    self.next(1);
    Ok(())
  }

  fn expect_optional_whitespace(&self) {
    while let Some(b) = self.current_byte() {
      if !is_whitespace(b) { break }
      self.next(1);
    }
  }

  fn parse_object(&self) -> ParseResult<HashMap<&'input str, JsonValue<'input>>> {
    let mut output: HashMap<&str, JsonValue> = HashMap::new();

    self.expect_optional_whitespace();
    try!(self.expect(b'{'));
    self.expect_optional_whitespace();

    loop {

      let (property, value) = try!(self.parse_key_value_pair());
      output.insert(property, value);

      self.expect_optional_whitespace();

      match self.current_byte().unwrap() {
        b',' => {
          try!(self.expect(b','));
          self.expect_optional_whitespace();

          match self.current_byte().unwrap() {
            b'}' => {
              try!(self.expect(b'}'));
              break;
            },
            _ => {
              continue;
            },
          }
        },
        b'}' => {
          try!(self.expect(b'}'));
          break;
        },
        c => {
          return Err(
            ParseError::new(
              self.input,
              self.current_idx(),
              ParseErrorKind::UnexpectedCharacter(c, vec![b',', b'}'])
            )
          );
        },
      }
    }

    self.expect_optional_whitespace();

    Ok(output)
  }

  fn parse_array(&self) -> ParseResult<Vec<JsonValue<'input>>> {
    let mut output = Vec::with_capacity(2);

    self.expect_optional_whitespace();
    try!(self.expect(b'['));
    self.expect_optional_whitespace();

    loop {
      let value = try!(self.parse_value());
      output.push(value);

      self.expect_optional_whitespace();

      match self.current_byte().unwrap() {
        b',' => {
          try!(self.expect(b','));
          self.expect_optional_whitespace();

          match self.current_byte().unwrap() {
            b']' => {
              try!(self.expect(b']'));
              break;
            },
            _ => {
              continue;
            },
          }
        },
        b']' => {
          try!(self.expect(b']'));
          break;
        },
        c => {
          return Err(
            ParseError::new(
              self.input,
              self.current_idx(),
              ParseErrorKind::UnexpectedCharacter(c, vec![b',', b']'])
            )
          );
        },
      }
    }

    Ok(output)
  }

  fn parse_key_value_pair(&self) -> ParseResult<(&'input str, JsonValue<'input>)> {
    let property_name = try!(self.parse_string());

    self.expect_optional_whitespace();
    try!(self.expect(b':'));
    self.expect_optional_whitespace();
    let value = try!(self.parse_value());
    self.expect_optional_whitespace();

    Ok((property_name, value))
  }

  fn parse_value(&self) -> ParseResult<JsonValue<'input>> {
    Ok(
      match self.current_byte().unwrap() {
        b'"' => JsonValue::String(try!(self.parse_string())),
        b'{' => JsonValue::Object(try!(self.parse_object())),
        b'[' => JsonValue::Array(try!(self.parse_array())),
        c if is_digit(c) || c == b'-' => JsonValue::Number(try!(self.parse_number())),
        b't' | b'f' => JsonValue::Boolean(try!(self.parse_bool())),
        b'n' => {
          try!(self.expect(b'n'));
          try!(self.expect(b'u'));
          try!(self.expect(b'l'));
          try!(self.expect(b'l'));
          JsonValue::Null
        }
        _ => unimplemented!()
      }
    )
  }

  fn parse_string(&self) -> ParseResult<&'input str> {
    try!(self.expect(b'"'));

    let string_start_idx = self.current_idx();
    while self.current_byte().unwrap() != b'"' {
      self.next(1);
    }
    let string_end_idx = self.current_idx();

    let string = &self.input[string_start_idx..string_end_idx];

    try!(self.expect(b'"'));

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

    let integer_part_start: usize = self.current_idx();

    if self.current_byte().unwrap() == b'-' {
      self.next(1);
    }

    while is_digit(self.current_byte().unwrap()) {
      self.next(1);
    }

    let mut decimal_part_end: usize = self.current_idx();

    if self.current_byte().unwrap() == b'.' {
      self.next(1);
      decimal_part_end += 1;
      while is_digit(self.current_byte().unwrap()) {
        self.next(1);
        decimal_part_end += 1;
      }
    }

    let string = &self.input[integer_part_start..decimal_part_end];

    Ok(string.parse().unwrap()) // it is unclear whether this can fail
  }

  fn parse_bool(&self) -> ParseResult<bool> {
    match self.current_byte().unwrap() {
      b't' => {
        try!(self.expect(b't'));
        try!(self.expect(b'r'));
        try!(self.expect(b'u'));
        try!(self.expect(b'e'));
        Ok(true)
      },
      b'f' => {
        try!(self.expect(b'f'));
        try!(self.expect(b'a'));
        try!(self.expect(b'l'));
        try!(self.expect(b's'));
        try!(self.expect(b'e'));
        Ok(false)
      },
      _ => unreachable!()
    }
  }
}
