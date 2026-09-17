use std::collections::HashMap;

use crate::{
    JsonValue,
    error::SyntaxError,
    lexer::{Position, PositionalToken, Token},
};

pub struct Parser {
    tokens: Vec<PositionalToken>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<PositionalToken>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn peek(&self) -> Option<PositionalToken> {
        self.tokens.get(self.cursor).cloned()
    }

    pub fn peek_next(&self) -> Option<PositionalToken> {
        self.tokens.get(self.cursor + 1).cloned()
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn parse_array(&mut self) -> Result<JsonValue, SyntaxError> {
        todo!()
    }

    pub fn parse_object(&mut self) -> Result<JsonValue, SyntaxError> {
        todo!()
    }

    pub fn parse(&mut self) -> Result<JsonValue, SyntaxError> {
        while let Some(token) = self.peek() {
            match token.token {
                Token::LeftBracket => self.parse_array()?,
                Token::LeftBrace => self.parse_object()?,
                _ => return Err(SyntaxError::IncompleteToken(Position::new())),
            };
        }

        Err(SyntaxError::IncompleteToken(Position::new()))
    }
}
