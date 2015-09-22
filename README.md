# json-parser
**Warning: this is a learning project.
Use [serde](https://github.com/serde-rs/serde) instead.**

A JSON parser written in Rust as a learning project.
Inspired by [Douglas Crockford's JSON parser](https://github.com/douglascrockford/JSON-js/blob/master/json_parse.js).

I'm doing this for fun, and to teach myself parser design.

## Things left to implement:
- exponentials
- (optional) handle floats and integers separately
- backslash escapes handling
  - escape characters: \n, \r, \f, \b, \t
  - escaped characters: \, / (https://stackoverflow.com/questions/4264877/why-is-the-slash-an-escapable-character-in-json), "
  - unicode code points: \uXXXX (4 hex digits)

## Benchmark
    test json_parser_large ... bench:       5,450 ns/iter (+/- 337)
    test serde_json_large  ... bench:      17,030 ns/iter (+/- 494)

Note: considering `json_parser` still misses important features like
exponentials and escapes, it's not very surprising that it's faster than
`serde_json`.
