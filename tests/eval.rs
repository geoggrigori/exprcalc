//! Integration tests for the public `eval` API.

use exprcalc::{eval, Error};

#[test]
fn precedence_multiply_before_add() {
    assert_eq!(eval("2 + 3 * 4").unwrap(), 14.0);
}

#[test]
fn parentheses_override_precedence() {
    assert_eq!(eval("(2 + 3) * 4").unwrap(), 20.0);
}

#[test]
fn unary_minus() {
    assert_eq!(eval("-5 + 2").unwrap(), -3.0);
}

#[test]
fn double_unary_minus() {
    assert_eq!(eval("--5").unwrap(), 5.0);
}

#[test]
fn modulo() {
    assert_eq!(eval("10 % 3").unwrap(), 1.0);
    assert_eq!(eval("10 % 4").unwrap(), 2.0);
}

#[test]
fn division() {
    assert_eq!(eval("9 / 2").unwrap(), 4.5);
}

#[test]
fn division_by_zero_is_error() {
    assert_eq!(eval("1 / 0"), Err(Error::DivisionByZero));
}

#[test]
fn modulo_by_zero_is_error() {
    assert_eq!(eval("5 % 0"), Err(Error::DivisionByZero));
}

#[test]
fn invalid_syntax_is_parse_error() {
    match eval("2 +") {
        Err(Error::ParseError { .. }) => {}
        other => panic!("expected ParseError, got {other:?}"),
    }
}

#[test]
fn unbalanced_parenthesis_is_parse_error() {
    match eval("(2 + 3") {
        Err(Error::ParseError { .. }) => {}
        other => panic!("expected ParseError, got {other:?}"),
    }
}

#[test]
fn trailing_input_is_parse_error() {
    match eval("2 3") {
        Err(Error::ParseError { .. }) => {}
        other => panic!("expected ParseError, got {other:?}"),
    }
}

#[test]
fn invalid_char_is_lex_error() {
    match eval("2 $ 3") {
        Err(Error::LexError { ch: '$', .. }) => {}
        other => panic!("expected LexError, got {other:?}"),
    }
}

#[test]
fn float_math() {
    let result = eval("0.1 + 0.2").unwrap();
    assert!((result - 0.3).abs() < 1e-9);
}

#[test]
fn nested_and_combined() {
    // -(2 + 3) * (4 - 1) % 7  =>  -5 * 3 % 7  =>  -15 % 7  =>  -1
    assert_eq!(eval("-(2 + 3) * (4 - 1) % 7").unwrap(), -1.0);
}

#[test]
fn whitespace_is_ignored() {
    assert_eq!(eval("   7\t+  \n 8 ").unwrap(), 15.0);
}
