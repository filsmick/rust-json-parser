# json-parser
**Warning: this is a learning project.
Use [serde](https://github.com/serde-rs/serde) instead.**

A JSON parser written in Rust as a learning project.
Inspired by [Douglas Crockford's JSON parser](https://github.com/douglascrockford/JSON-js/blob/master/json_parse.js).

I'm doing this for fun, and to teach myself parser design.

## Things left to implement:
- exponentials
- (optional) handle floats and integers separately
- escape code handling
- unicode code points

## Benchmark
    test bench_large ... bench:       7,006 ns/iter (+/- 617)
