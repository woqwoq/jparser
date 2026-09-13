use std::collections::HashMap;

pub enum JsonError {
    General { row: usize, col: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f64),
    List(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
    Null,
}

pub enum Token {
    Space,
    Tab,
    Char,
    Coma,
    CurlyBracketOpen,
    CurlyBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    Number,
    Minus,
    Plus,
    Period,
    E,
}

pub struct Lexer {
    row: usize,
    col: usize,

    input: String,
    structure: Vec<JsonValue>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            row: 0,
            col: 0,
            input: input.to_string(),
            structure: Vec::new(),
        }
    }

    pub fn tokenize() -> Result<(), JsonError> {
        Ok(())
    }
}
fn main() {
    println!("Hello, world!");
}
