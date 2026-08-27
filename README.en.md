<!-- ══════════════════════════ TITLE ══════════════════════════ -->
<div align="center">
  <img src="docs/title-banner.svg" width="100%" alt="exprcalc"/>
</div>

<br/>

<!-- ══════════════════════ IDIOMAS / LANGUAGES ══════════════════════ -->
<div align="center">
<a href="README.md"><img src="https://img.shields.io/badge/Português-555555?style=for-the-badge" alt="Português"/></a>
<a href="README.en.md"><img src="https://img.shields.io/badge/English-1987F0?style=for-the-badge" alt="English"/></a>
<a href="README.es.md"><img src="https://img.shields.io/badge/Español-555555?style=for-the-badge" alt="Español"/></a>
</div>

<br/>

[![CI](https://github.com/geoggrigori/exprcalc/actions/workflows/ci.yml/badge.svg)](https://github.com/geoggrigori/exprcalc/actions/workflows/ci.yml)

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Cargo](https://img.shields.io/badge/Cargo-build-DEA584?logo=rust&logoColor=white)](https://doc.rust-lang.org/cargo)
[![License: MIT](https://img.shields.io/badge/License-MIT-6B2FB5)](LICENSE)
[![zero dependencies](https://img.shields.io/badge/dependencies-zero-4A1E86)](Cargo.toml)

A small, fast arithmetic expression evaluator written in Rust. It takes a string
like `2 + 3 * (4 - 1)`, runs it through a hand-written **lexer**, a
**recursive-descent parser**, and an **evaluator**, and gives you back an `f64`.

The whole crate is **standard library only** — no external dependencies.

## Features

- Numbers (integer and floating point), the operators `+ - * / % ^`, unary
  minus, and parentheses for grouping.
- Correct operator precedence: `^` binds tighter than `* / %`, which bind
  tighter than `+ -`.
- Right-associative exponentiation (`2 ^ 3 ^ 2` evaluates to `512`) and
  right-associative unary minus (`--5` evaluates to `5`).
- A clean `Error` enum (`LexError`, `ParseError` with position, `DivisionByZero`)
  implementing `Display` and `std::error::Error`.
- Usable as a **library** (`pub fn eval(input: &str) -> Result<f64, Error>`) or
  as a **CLI** with both one-shot and REPL modes.
- Fully tested: unit, integration, and documentation tests.

## How it works

```mermaid
flowchart LR
    A["input string"] --> B["Lexer"]
    B -->|tokens| C["Parser"]
    C -->|AST| D["Evaluator"]
    D -->|f64 result| E["output"]

    B -.->|invalid character| X["Error::LexError"]
    C -.->|malformed input| Y["Error::ParseError"]
    D -.->|divide by zero| Z["Error::DivisionByZero"]
```

## Grammar

The supported grammar, in EBNF-ish form:

```ebnf
expr    = term , { ( "+" | "-" ) , term } ;
term    = power , { ( "*" | "/" | "%" ) , power } ;
power   = unary , [ "^" , power ] ;
unary   = "-" , unary | primary ;
primary = number | "(" , expr , ")" ;
number  = digit , { digit } , [ "." , { digit } ]
        | "." , digit , { digit } ;
digit   = "0".."9" ;
```

Precedence (lowest to highest): additive (`+ -`) → multiplicative (`* / %`) →
exponentiation (`^`) → unary minus → primary. Exponentiation and unary minus
are right-associative; the additive and multiplicative operators are
left-associative. Because unary minus binds tighter than `^`, `-2 ^ 2` parses
as `(-2) ^ 2 = 4`; use `2 ^ (-1)` for a negative exponent.

## Build & install

```sh
cargo build --release
```

The optimized binary is written to `target/release/exprcalc`. You can also
install it onto your `PATH` with:

```sh
cargo install --path .
```

## Usage

**One-shot** — pass the expression as arguments:

```sh
$ exprcalc "2 + 3 * (4 - 1)"
11

$ exprcalc "-(2 + 3) * 4"
-20

$ exprcalc "10 % 3"
1

$ exprcalc "2 ^ 3 ^ 2"
512

$ exprcalc "2 * 3 ^ 2"
18
```

On an error the message goes to stderr and the process exits with status `1`:

```sh
$ exprcalc "1 / 0"
error: evaluation error: division by zero

$ exprcalc "2 +"
error: parse error at position 3: unexpected end of input, expected a number or '('
```

**REPL** — run with no arguments to evaluate expressions line by line until EOF:

```sh
$ exprcalc
> 2 + 2
4
> (1 + 2) * 3
9
> 5 $ 1
error: lex error: unexpected character '$' at position 2
```

### As a library

```rust
use exprcalc::eval;

fn main() {
    match eval("2 + 3 * 4") {
        Ok(value) => println!("= {value}"),
        Err(err) => eprintln!("error: {err}"),
    }
}
```

## Running tests

```sh
cargo test
```

This runs the unit tests (lexer and parser), the integration tests under
`tests/`, and the documentation tests embedded in the API docs.

## License

Released under the [MIT License](LICENSE). Copyright (c) 2026 Geovana Grigorio.
