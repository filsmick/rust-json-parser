use std::collections::HashMap;
use std::cell::Cell;
use JsonValue;
use parse_error::*;

pub struct JsonParser<'input> {
  input: &'input str,
  remaining_data: Cell<&'input str>,
  current_idx: Cell<usize>,
  current_line: Cell<usize>,
  current_column: Cell<usize>
}

// Public interface
impl<'input> JsonParser<'input> {
  /// Create a new `JsonParser` with the given input.
  pub fn new(input: &str) -> JsonParser {
    JsonParser {
      input: input,
      remaining_data: Cell::new(input),
      current_idx: Cell::new(0),
      current_line: Cell::new(1), // lines and columns are 1-indexed
      current_column: Cell::new(1)
    }
  }

  /// Parse the input as an array or an object.
  pub fn parse(&self) -> ParseResult<JsonValue<'input>> {
    Ok(try!(self.parse_value()))
  }
}

// Private methods
impl<'input> JsonParser<'input> {
  fn current_line(&self) -> usize {
    self.current_line.get()
  }

  fn current_column(&self) -> usize {
    self.current_column.get()
  }

  fn current_char(&self) -> char {
    self.input.char_at(self.current_idx.get())
  }

  fn next(&self) {
    self.consume(1);
  }

  fn consume(&self, n: usize) -> Option<&'input str> {
    let new_idx = self.current_idx.get() + n;

    if new_idx < self.input.len() {
      self.current_idx.set(new_idx);

      for c in (&self.remaining_data.get()[..n]).chars() {
        if c == '\n' {
          self.current_line.set(self.current_line.get() + 1);
          self.current_column.set(1);
        } else {
          self.current_column.set(self.current_column.get() + 1);
        }
      }

      let ret = Some(&self.remaining_data.get()[..n]);
      self.remaining_data.set(
        &self.input[self.current_idx.get()..]
      );
      ret
    } else {
      None
    }
  }

  fn expect(&self, expected: char) -> ParseResult<()> {
    let found = self.current_char();

    if found != expected {
      return Err(
        ParseError::new(
          ParseErrorKind::UnexpectedCharacter(found, expected),
          self.current_line(),
          self.current_column()
        )
      );
    }

    self.next();
    Ok(())
  }

  fn expect_optional_whitespace(&self) {
    while self.current_char().is_whitespace() {
      self.next();
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
        c if c.is_digit(10) || c == '-' => JsonValue::Number(try!(self.parse_number())),
        't' | 'f' => JsonValue::Boolean(try!(self.parse_bool())),
        '{' => JsonValue::Object(try!(self.parse_object())),
        '[' => JsonValue::Array(try!(self.parse_array())),
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

    let idx = self.remaining_data.get().chars().take_while(|c| *c != '"').count();
    let string = self.consume(idx).unwrap();

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
      self.next();
    }

    while self.current_char().is_digit(10) {
      self.next();
    }

    let mut decimal_part_end: usize = self.current_idx.get();

    if self.current_char() == '.' {
      self.next();
      decimal_part_end += 1;
      while self.current_char().is_digit(10) {
        self.next();
        decimal_part_end += 1;
      }
    }

    let string = &self.input[integer_part_start..decimal_part_end];

    Ok(string.parse().unwrap())
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
