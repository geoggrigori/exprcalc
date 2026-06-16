//! exprcalc — a small arithmetic expression evaluator.
//!
//! The crate is organized as a classic three-stage pipeline:
//!
//! 1. [`lexer`] turns an input string into a flat stream of [`Token`]s.
//! 2. [`parser`] consumes those tokens and builds an [`Expr`] abstract
//!    syntax tree honoring operator precedence and associativity.
//! 3. [`eval`] walks the tree and produces an [`f64`] result.
//!
//! The single entry point most callers need is [`eval`]:
//!
//! ```
//! use exprcalc::eval;
//!
//! assert_eq!(eval("2 + 3 * 4").unwrap(), 14.0);
//! assert_eq!(eval("(2 + 3) * 4").unwrap(), 20.0);
//! assert_eq!(eval("-5 + 2").unwrap(), -3.0);
//! ```
//!
//! All errors are reported through the [`Error`] enum, which implements
//! both [`std::fmt::Display`] and [`std::error::Error`].

use std::fmt;

pub mod lexer;
pub mod parser;
pub mod eval;

pub use lexer::{Token, TokenKind};
pub use parser::Expr;

/// Errors that can occur while lexing, parsing, or evaluating an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An invalid character was encountered during lexing.
    ///
    /// Carries the offending character and its byte position in the input.
    LexError { ch: char, pos: usize },
    /// The token stream did not form a valid expression.
    ///
    /// Carries a human-readable message and the byte position where the
    /// problem was detected.
    ParseError { message: String, pos: usize },
    /// Evaluation attempted to divide (or take a remainder) by zero.
    DivisionByZero,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LexError { ch, pos } => {
                write!(f, "lex error: unexpected character '{ch}' at position {pos}")
            }
            Error::ParseError { message, pos } => {
                write!(f, "parse error at position {pos}: {message}")
            }
            Error::DivisionByZero => write!(f, "evaluation error: division by zero"),
        }
    }
}

impl std::error::Error for Error {}

/// Evaluate an arithmetic expression and return its `f64` value.
///
/// Supported syntax: numbers (integer or floating point), the binary
/// operators `+ - * / %`, unary minus, and parentheses for grouping.
///
/// # Errors
///
/// Returns an [`Error`] if the input contains an invalid character
/// ([`Error::LexError`]), is not a well-formed expression
/// ([`Error::ParseError`]), or divides by zero ([`Error::DivisionByZero`]).
///
/// # Examples
///
/// ```
/// use exprcalc::eval;
///
/// assert_eq!(eval("10 % 3").unwrap(), 1.0);
/// assert!(eval("1 / 0").is_err());
/// ```
pub fn eval(input: &str) -> Result<f64, Error> {
    let tokens = lexer::tokenize(input)?;
    let expr = parser::parse(tokens)?;
    eval::eval_expr(&expr)
}
