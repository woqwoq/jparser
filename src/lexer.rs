use std::panic;

use crate::error::JsonError;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    line: usize,
    col: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PositionalToken {
    token: Token,
    position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Comma,
    Colon,

    String(String),
    Bool(bool),
    Number(f64),
    Null,
}

pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,

    position: Position,
}
