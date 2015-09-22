use std::collections::HashMap;
use std::cell::Cell;
use std::borrow::Cow;
use std::str;
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

  fn current_char(&self) -> Option<char> {
    if self.current_idx() < self.input.len() {
      Some(self.input.char_at(self.current_idx()))
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

  fn expect(&self, expected: char) -> ParseResult<()> {
    let found = self.current_char().unwrap();

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

  fn parse_object(&self) -> ParseResult<HashMap<Cow<'input, str>, JsonValue<'input>>> {
    let mut output = HashMap::new();

    self.expect_optional_whitespace();
    try!(self.expect('{'));
    self.expect_optional_whitespace();

    loop {

      let (property, value) = try!(self.parse_key_value_pair());
      output.insert(property, value);

      self.expect_optional_whitespace();

      match self.current_byte().unwrap() {
        b',' => {
          try!(self.expect(','));
          self.expect_optional_whitespace();

          match self.current_byte().unwrap() {
            b'}' => {
              try!(self.expect('}'));
              break;
            },
            _ => {
              continue;
            },
          }
        },
        b'}' => {
          try!(self.expect('}'));
          break;
        },
        _ => {
          return Err(
            ParseError::new(
              self.input,
              self.current_idx(),
              ParseErrorKind::UnexpectedCharacter(self.current_char().unwrap(), vec![',', '}'])
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
    try!(self.expect('['));
    self.expect_optional_whitespace();

    loop {
      let value = try!(self.parse_value());
      output.push(value);

      self.expect_optional_whitespace();

      match self.current_byte().unwrap() {
        b',' => {
          try!(self.expect(','));
          self.expect_optional_whitespace();

          match self.current_byte().unwrap() {
            b']' => {
              try!(self.expect(']'));
              break;
            },
            _ => {
              continue;
            },
          }
        },
        b']' => {
          try!(self.expect(']'));
          break;
        },
        _ => {
          return Err(
            ParseError::new(
              self.input,
              self.current_idx(),
              ParseErrorKind::UnexpectedCharacter(self.current_char().unwrap(), vec![',', ']'])
            )
          );
        },
      }
    }

    Ok(output)
  }

  fn parse_key_value_pair(&self) -> ParseResult<(Cow<'input, str>, JsonValue<'input>)> {
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
      match self.current_byte().unwrap() {
        b'"' => JsonValue::String(try!(self.parse_string())),
        b'{' => JsonValue::Object(try!(self.parse_object())),
        b'[' => JsonValue::Array(try!(self.parse_array())),
        b'0'...b'9' | b'-' => JsonValue::Number(try!(self.parse_number())),
        b't' | b'f' => JsonValue::Boolean(try!(self.parse_bool())),
        b'n' => {
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

  fn parse_string(&self) -> ParseResult<Cow<'input, str>> {
    try!(self.expect('"'));

    let mut byte_buf: Option<Vec<u8>> = None;

    let string_start_idx = self.current_idx();
    let mut string_end_idx;
    let mut byte_slice;

    loop {
      string_end_idx = self.current_idx();
      byte_slice = &self.input.as_bytes()[string_start_idx..string_end_idx];

      let b = self.current_byte().unwrap();

      match b {
        b'"' => break,

        b'\\' => {
          self.next(1);
          let b = self.current_byte().unwrap();

          let mut byte_buf_unwrapped = byte_buf.unwrap_or(byte_slice.to_vec());

          match b {
            b'"' => byte_buf_unwrapped.push(b'"'),
            b'\\' => byte_buf_unwrapped.push(b'\\'),
            b'n' => byte_buf_unwrapped.push(b'\n'),
            b'r' => byte_buf_unwrapped.push(b'\r'),
            b't' => byte_buf_unwrapped.push(b'\t'),
            c => panic!("{}", ::std::char::from_u32(c as u32).unwrap())
          }
          byte_buf = Some(byte_buf_unwrapped);
        },
        c => {
          match byte_buf {
            Some(ref mut buf) => buf.push(c),
            None => {}
          }
        }
      }

      self.next(1);
    }

    let string_slice = unsafe { str::from_utf8_unchecked(byte_slice) };

    try!(self.expect('"'));

    Ok(
      match byte_buf {
        Some(buf) => Cow::Owned(
          unsafe { String::from_utf8_unchecked(buf) }
        ),
        None => Cow::Borrowed(string_slice)
      }
    )
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
        try!(self.expect('t'));
        try!(self.expect('r'));
        try!(self.expect('u'));
        try!(self.expect('e'));
        Ok(true)
      },
      b'f' => {
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
