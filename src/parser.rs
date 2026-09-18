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

// TODO:
// 1. Add expect() for handling commas
// 2. Add parse_value() for handling regular values
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
        let mut array: Vec<JsonValue> = Vec::new();

        // First token has to be Token::LeftBracket [ or we error
        if let Some(token) = self.peek()
            && token.token == Token::LeftBracket
        {
            self.advance();
        } else {
            return Err(SyntaxError::IncompleteToken(Position::new()));
        }

        while let Some(token) = self.peek() {
            match token.token {
                // Seeing [ means that we are facing a nested array, so we go in recursively
                Token::LeftBracket => {
                    array.push(self.parse_array()?);
                }
                // Right bracket here means that we are at the end of the current array, so we
                // return the current array representation (vector)
                Token::RightBracket => {
                    self.advance();
                    return Ok(JsonValue::List(array));
                }
                Token::Comma => {
                    if let Some(next) = self.peek_next()
                        && matches!(
                            next.token,
                            Token::Null | Token::Bool(_) | Token::String(_) | Token::Number(_)
                        )
                    {
                        self.advance();
                    } else {
                        return Err(SyntaxError::UnexpectedToken(token.position, '#'));
                    }
                }
                Token::Null => {
                    self.advance();
                    array.push(JsonValue::Null);
                }
                Token::Bool(b) => {
                    self.advance();
                    array.push(JsonValue::Bool(b));
                }
                Token::Number(n) => {
                    self.advance();
                    array.push(JsonValue::Number(n));
                }
                Token::String(s) => {
                    self.advance();
                    array.push(JsonValue::String(s));
                }
                _ => return Err(SyntaxError::IncompleteToken(Position::new())),
            };
        }

        Err(SyntaxError::IncompleteToken(Position::new()))
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

#[cfg(test)]
mod parser_tests {
    use crate::{
        JsonValue,
        lexer::{Lexer, PositionalToken},
        parser::Parser,
    };

    fn build_pos_token_vec_from_input(input: &str) -> Vec<PositionalToken> {
        Lexer::new(input).tokenize().unwrap()
    }

    fn build_parser_with_pos_token_vec_from_input(input: &str) -> Parser {
        Parser::new(build_pos_token_vec_from_input(input))
    }

    #[test]
    fn parse_array_passes_on_well_formed() {
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[]").parse_array(),
            Ok(JsonValue::List(vec![]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[null]").parse_array(),
            Ok(JsonValue::List(vec![JsonValue::Null]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[true]").parse_array(),
            Ok(JsonValue::List(vec![JsonValue::Bool(true)]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[false]").parse_array(),
            Ok(JsonValue::List(vec![JsonValue::Bool(false)]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[1]").parse_array(),
            Ok(JsonValue::List(vec![JsonValue::Number(1.0)]))
        );

        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[1 2]").parse_array(),
            Ok(JsonValue::List(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0)
            ]))
        );
    }
}
