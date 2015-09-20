use std::collections::HashMap;
use std::cell::Cell;
use JsonValue;

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
  pub fn parse(&self) -> JsonValue<'input> {
    self.parse_value()
  }
}

// Private methods
impl<'input> JsonParser<'input> {
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

  fn parse_error(&self) -> ! {
    panic!("At {}:{}", self.current_line.get(), self.current_column.get());
  }

  fn expect(&self, expected: char) {
    let current_char = self.current_char();

    if current_char != expected {

      self.parse_error();
    }

    self.next();
  }

  fn expect_optional_whitespace(&self) {
    while self.current_char().is_whitespace() {
      self.next();
    }
  }

  fn parse_object(&self) -> HashMap<&'input str, JsonValue<'input>> {
    let mut output: HashMap<&str, JsonValue> = HashMap::new();

    self.expect_optional_whitespace();
    self.expect('{');
    self.expect_optional_whitespace();

    loop {

      let (property, value) = self.parse_key_value_pair();
      output.insert(property, value);

      self.expect_optional_whitespace();

      match self.current_char() {
        ',' => {
          self.expect(',');
          self.expect_optional_whitespace();

          match self.current_char() {
            '}' => {
              self.expect('}');
              break;
            },
            _ => {
              continue;
            },
          }
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

  fn parse_array(&self) -> Vec<JsonValue<'input>> {
    let mut output = Vec::with_capacity(2);

    self.expect_optional_whitespace();
    self.expect('[');
    self.expect_optional_whitespace();

    loop {
      let value = self.parse_value();
      output.push(value);

      self.expect_optional_whitespace();

      match self.current_char() {
        ',' => {
          self.expect(',');
          self.expect_optional_whitespace();

          match self.current_char() {
            ']' => {
              self.expect(']');
              break;
            },
            _ => {
              continue;
            },
          }
        },
        ']' => {
          self.expect(']');
          break;
        },
        c => panic!("Unexpected character '{}' at {}", c, self.current_idx.get()),
      }
    }

    output
  }

  fn parse_key_value_pair(&self) -> (&'input str, JsonValue<'input>) {
    let property_name = self.parse_string();


    self.expect_optional_whitespace();
    self.expect(':');
    self.expect_optional_whitespace();
    let value = self.parse_value();
    self.expect_optional_whitespace();


    (property_name, value)
  }

  fn parse_value(&self) -> JsonValue<'input> {
    match self.current_char() {
      '"' => JsonValue::String(self.parse_string()),
      c if c.is_digit(10) || c == '-' => JsonValue::Number(self.parse_number()),
      't' | 'f' => JsonValue::Boolean(self.parse_bool()),
      '{' => JsonValue::Object(self.parse_object()),
      '[' => JsonValue::Array(self.parse_array()),
      _ => unimplemented!()
    }
  }

  fn parse_string(&self) -> &'input str {
    self.expect('"');


    let idx = self.remaining_data.get().chars().take_while(|c| *c != '"').count();
    let string = self.consume(idx).unwrap();




    self.expect('"');

    string
  }

  fn parse_number(&self) -> f64 {
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
