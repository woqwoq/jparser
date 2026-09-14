use crate::JsonError;

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
    row: usize,
    col: usize,

    input: String,
    structure: Vec<Token>,
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
