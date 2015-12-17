pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
struct ParseErrorContext {
    pub line: usize,
    pub column: usize,
}

impl ParseErrorContext {
    pub fn new(input: &str, idx: usize) -> ParseErrorContext {
        // Iterate over `input` to compute the line and column lazily.

        let mut line = 1;
        let mut column = 1;

        for (i, c) in input.chars().enumerate() {
            if i == idx {
                break;
            }

            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        ParseErrorContext {
            line: line,
            column: column,
        }
    }
}

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedEndOfInput,
    UnexpectedCharacter(char, Vec<char>),
}

#[derive(Debug)]
pub struct ParseError {
    ctx: ParseErrorContext,
    kind: ParseErrorKind,
}

impl ParseError {
    pub fn new(input: &str, idx: usize, kind: ParseErrorKind) -> ParseError {
        ParseError {
            ctx: ParseErrorContext::new(input, idx),
            kind: kind,
        }
    }
}
