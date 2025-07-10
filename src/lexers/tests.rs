use super::*;
use crate::lexers::{Lexer, Token};

fn lex_all(input: &str) -> Vec<Token> {
    Lexer::new(input).collect()
}

#[test]
fn test_single_operators() {
    let tokens = lex_all("+-*/=();");
    assert_eq!(
        tokens,
        vec![
            Token::Add,
            Token::Sub,
            Token::Mult,
            Token::Div,
            Token::Assign,
            Token::OpenParen,
            Token::CloseParen,
            Token::Semicolon,
        ]
    );
}

#[test]
fn test_number_parsing() {
    let tokens = lex_all("123 456");
    assert_eq!(
        tokens,
        vec![
            Token::Number(123),
            Token::Number(456),
        ]
    );
}

#[test]
fn test_identifier_parsing() {
    let tokens = lex_all("hello world");
    assert_eq!(
        tokens,
        vec![
            Token::Ident("hello".into()),
            Token::Ident("world".into()),
        ]
    );
}

#[test]
fn test_reserved_keywords() {
    let tokens = lex_all("func loop custom");
    assert_eq!(
        tokens,
        vec![
            Token::Function("func".into()),
            Token::Loop,
            Token::Ident("custom".into()),
        ]
    );
}

#[test]
fn test_mixed_input() {
    let tokens = lex_all("x = 3 + 4; loop");
    assert_eq!(
        tokens,
        vec![
            Token::Ident("x".into()),
            Token::Assign,
            Token::Number(3),
            Token::Add,
            Token::Number(4),
            Token::Semicolon,
            Token::Loop,
        ]
    );
}

#[test]
fn test_complex_expression() {
    let tokens = lex_all("a = (1 + 2) * func;");
    assert_eq!(
        tokens,
        vec![
            Token::Ident("a".into()),
            Token::Assign,
            Token::OpenParen,
            Token::Number(1),
            Token::Add,
            Token::Number(2),
            Token::CloseParen,
            Token::Mult,
            Token::Function("func".into()),
            Token::Semicolon,
        ]
    );
}

#[test]
fn test_iterator_usage() {
    let mut lexer = Lexer::new("1 + 2");
    assert_eq!(lexer.next(), Some(Token::Number(1)));
    assert_eq!(lexer.next(), Some(Token::Add));
    assert_eq!(lexer.next(), Some(Token::Number(2)));
    assert_eq!(lexer.next(), None);
}
