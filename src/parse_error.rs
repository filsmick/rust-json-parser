pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
struct ParseErrorContext {
  pub line: usize,
  pub column: usize
}

#[derive(Debug)]
pub enum ParseErrorKind {
  UnexpectedEndOfInput,
  UnexpectedCharacter(char, char)
}

#[derive(Debug)]
pub struct ParseError {
  ctx: ParseErrorContext,
  kind: ParseErrorKind
}

impl ParseError {
  pub fn new(kind: ParseErrorKind, line: usize, column: usize) -> ParseError {
    ParseError {
      ctx: ParseErrorContext {
        line: line,
        column: column
      },
      kind: kind
    }
  }
}
