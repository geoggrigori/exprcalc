//! Lexer: turns raw input text into a stream of [`Token`]s.

use crate::Error;

/// The kind of a lexical token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A numeric literal, already parsed to `f64`.
    Number(f64),
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// An opening parenthesis `(`.
    LParen,
    /// A closing parenthesis `)`.
    RParen,
    /// End of input.
    Eof,
}

/// A token together with the byte position where it begins in the input.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,
    /// Byte offset of the token's start in the original input.
    pub pos: usize,
}

impl Token {
    fn new(kind: TokenKind, pos: usize) -> Self {
        Token { kind, pos }
    }
}

/// Tokenize `input` into a vector of [`Token`]s, always terminated by
/// [`TokenKind::Eof`].
///
/// Whitespace is skipped. A number is a run of digits with an optional
/// single decimal point.
///
/// # Errors
///
/// Returns [`Error::LexError`] if an unrecognized character is found.
pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    // Work on bytes for stable positions; the grammar is ASCII-only.
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::new(TokenKind::Plus, i));
                i += 1;
            }
            '-' => {
                tokens.push(Token::new(TokenKind::Minus, i));
                i += 1;
            }
            '*' => {
                tokens.push(Token::new(TokenKind::Star, i));
                i += 1;
            }
            '/' => {
                tokens.push(Token::new(TokenKind::Slash, i));
                i += 1;
            }
            '%' => {
                tokens.push(Token::new(TokenKind::Percent, i));
                i += 1;
            }
            '(' => {
                tokens.push(Token::new(TokenKind::LParen, i));
                i += 1;
            }
            ')' => {
                tokens.push(Token::new(TokenKind::RParen, i));
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                let mut seen_dot = false;
                while i < bytes.len() {
                    let d = bytes[i] as char;
                    if d.is_ascii_digit() {
                        i += 1;
                    } else if d == '.' && !seen_dot {
                        seen_dot = true;
                        i += 1;
                    } else {
                        break;
                    }
                }
                let text = &input[start..i];
                // A lone "." is not a valid number.
                let value: f64 = text
                    .parse()
                    .map_err(|_| Error::LexError { ch: '.', pos: start })?;
                tokens.push(Token::new(TokenKind::Number(value), start));
            }
            other => {
                return Err(Error::LexError { ch: other, pos: i });
            }
        }
    }

    tokens.push(Token::new(TokenKind::Eof, bytes.len()));
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_numbers_and_operators() {
        let toks = tokenize("1 + 2.5 * 3").unwrap();
        let kinds: Vec<_> = toks.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::Number(1.0),
                TokenKind::Plus,
                TokenKind::Number(2.5),
                TokenKind::Star,
                TokenKind::Number(3.0),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn reports_invalid_char_position() {
        match tokenize("1 + @") {
            Err(Error::LexError { ch: '@', pos: 4 }) => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn lone_dot_is_lex_error() {
        assert!(tokenize(".").is_err());
    }
}
