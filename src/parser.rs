//! Recursive-descent parser: turns a token stream into an [`Expr`] AST.
//!
//! The grammar (see the project README for the full EBNF) encodes the
//! usual arithmetic precedence: `+` and `-` bind looser than `*`, `/`,
//! and `%`, which in turn bind looser than `^` (exponentiation), and
//! unary minus binds tightest of all. Both `^` and unary minus are
//! right-associative.

use crate::lexer::{Token, TokenKind};
use crate::Error;

/// A binary arithmetic operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Division.
    Div,
    /// Remainder (modulo).
    Rem,
    /// Exponentiation (`^`), right-associative.
    Pow,
}

/// An abstract syntax tree node for an arithmetic expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal.
    Number(f64),
    /// A unary negation, e.g. `-x`.
    Neg(Box<Expr>),
    /// A binary operation, e.g. `a + b`.
    Binary {
        /// The operator.
        op: BinOp,
        /// Left-hand operand.
        left: Box<Expr>,
        /// Right-hand operand.
        right: Box<Expr>,
    },
}

/// Parse a token stream into an [`Expr`].
///
/// The stream must be terminated by [`TokenKind::Eof`] (as produced by
/// [`crate::lexer::tokenize`]).
///
/// # Errors
///
/// Returns [`Error::ParseError`] if the tokens do not form a complete,
/// well-formed expression.
pub fn parse(tokens: Vec<Token>) -> Result<Expr, Error> {
    let mut p = Parser { tokens, pos: 0 };
    let expr = p.parse_expr()?;
    p.expect_eof()?;
    Ok(expr)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> &Token {
        // tokenize always appends an Eof token, so indexing is safe.
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn expect_eof(&self) -> Result<(), Error> {
        match self.peek().kind {
            TokenKind::Eof => Ok(()),
            _ => Err(Error::ParseError {
                message: "unexpected trailing input".to_string(),
                pos: self.peek().pos,
            }),
        }
    }

    /// expr := term ( ("+" | "-") term )*
    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_term()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// term := power ( ("*" | "/" | "%") power )*
    fn parse_term(&mut self) -> Result<Expr, Error> {
        let mut left = self.parse_power()?;
        loop {
            let op = match self.peek().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Rem,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    /// power := unary ( "^" power )?
    ///
    /// Exponentiation is right-associative, so `2 ^ 3 ^ 2` parses as
    /// `2 ^ (3 ^ 2)`. It binds tighter than `* / %` but looser than
    /// unary minus, so `-2 ^ 2` parses as `(-2) ^ 2`.
    fn parse_power(&mut self) -> Result<Expr, Error> {
        let base = self.parse_unary()?;
        if self.peek().kind == TokenKind::Caret {
            self.advance();
            let exponent = self.parse_power()?;
            return Ok(Expr::Binary {
                op: BinOp::Pow,
                left: Box::new(base),
                right: Box::new(exponent),
            });
        }
        Ok(base)
    }

    /// unary := "-" unary | primary
    ///
    /// Unary minus is right-associative: `--5` parses as `-(-5)`.
    fn parse_unary(&mut self) -> Result<Expr, Error> {
        if self.peek().kind == TokenKind::Minus {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expr::Neg(Box::new(operand)));
        }
        self.parse_primary()
    }

    /// primary := NUMBER | "(" expr ")"
    fn parse_primary(&mut self) -> Result<Expr, Error> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenKind::LParen => {
                self.advance();
                let inner = self.parse_expr()?;
                match self.peek().kind {
                    TokenKind::RParen => {
                        self.advance();
                        Ok(inner)
                    }
                    _ => Err(Error::ParseError {
                        message: "expected ')'".to_string(),
                        pos: self.peek().pos,
                    }),
                }
            }
            TokenKind::Eof => Err(Error::ParseError {
                message: "unexpected end of input, expected a number or '('".to_string(),
                pos: tok.pos,
            }),
            _ => Err(Error::ParseError {
                message: "expected a number or '('".to_string(),
                pos: tok.pos,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    fn ast(input: &str) -> Expr {
        parse(tokenize(input).unwrap()).unwrap()
    }

    #[test]
    fn precedence_builds_correct_tree() {
        // 2 + 3 * 4  =>  Add(2, Mul(3, 4))
        let expr = ast("2 + 3 * 4");
        match expr {
            Expr::Binary { op: BinOp::Add, right, .. } => {
                assert!(matches!(*right, Expr::Binary { op: BinOp::Mul, .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn unary_minus_is_right_associative() {
        // --5 => Neg(Neg(5))
        let expr = ast("--5");
        assert!(matches!(expr, Expr::Neg(_)));
    }

    #[test]
    fn exponent_is_right_associative_in_tree() {
        // 2 ^ 3 ^ 2  =>  Pow(2, Pow(3, 2))
        let expr = ast("2 ^ 3 ^ 2");
        match expr {
            Expr::Binary { op: BinOp::Pow, right, .. } => {
                assert!(matches!(*right, Expr::Binary { op: BinOp::Pow, .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn exponent_binds_tighter_than_multiply_in_tree() {
        // 2 * 3 ^ 2  =>  Mul(2, Pow(3, 2))
        let expr = ast("2 * 3 ^ 2");
        match expr {
            Expr::Binary { op: BinOp::Mul, right, .. } => {
                assert!(matches!(*right, Expr::Binary { op: BinOp::Pow, .. }));
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn missing_operand_is_parse_error() {
        assert!(parse(tokenize("2 +").unwrap()).is_err());
    }
}
