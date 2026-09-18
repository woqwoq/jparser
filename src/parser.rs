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

    pub fn parse_value(&mut self) -> Result<JsonValue, SyntaxError> {
        if let Some(token) = self.peek() {
            match token.token {
                Token::Null => {
                    self.advance();
                    Ok(JsonValue::Null)
                }
                Token::Bool(b) => {
                    self.advance();
                    Ok(JsonValue::Bool(b))
                }
                Token::Number(n) => {
                    self.advance();
                    Ok(JsonValue::Number(n))
                }
                Token::String(s) => {
                    self.advance();
                    Ok(JsonValue::String(s))
                }

                // Delegate array/object handling, do not consume token, because child functions will
                Token::LeftBracket => self.parse_array(),
                Token::LeftBrace => self.parse_object(),

                _ => Err(SyntaxError::ParseErrorPlaceHolder),
            }
        } else {
            Err(SyntaxError::ParseErrorPlaceHolder)
        }
    }

    // [[1], 1, 2]
    pub fn parse_array(&mut self) -> Result<JsonValue, SyntaxError> {
        let mut array: Vec<JsonValue> = Vec::new();

        // First token has to be Token::LeftBracket [ or we error
        if let Some(token) = self.peek()
            && token.token == Token::LeftBracket
        {
            self.advance();
        } else {
            return Err(SyntaxError::ParseErrorPlaceHolder);
        }

        while let Some(token) = self.peek() {
            match token.token {
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
                            Token::Null
                                | Token::Bool(_)
                                | Token::String(_)
                                | Token::Number(_)
                                | Token::LeftBracket
                                | Token::LeftBrace
                        )
                    {
                        self.advance();
                    } else {
                        return Err(SyntaxError::ParseErrorPlaceHolder);
                    }
                }
                _ => {
                    // Delgate the actual handling of values to parse_value and handle commans
                    array.push(self.parse_value()?);

                    if let Some(token) = self.peek()
                        && !matches!(token.token, Token::Comma | Token::RightBracket)
                    {
                        return Err(SyntaxError::MissingDelimiter(token.position.clone()));
                    }
                }
            };
        }

        Err(SyntaxError::ParseErrorPlaceHolder)
    }

    pub fn parse_object(&mut self) -> Result<JsonValue, SyntaxError> {
        let mut map: HashMap<String, JsonValue> = HashMap::new();

        // First token has to be Token::LeftBracket [ or we error
        if let Some(token) = self.peek()
            && token.token == Token::LeftBrace
        {
            self.advance();
        } else {
            return Err(SyntaxError::ParseErrorPlaceHolder);
        }

        // { "string1" : parse_value(), "string2" : parse_value()}

        let mut key: Option<String> = None;
        let mut colon = false;
        let mut value: Option<JsonValue> = None;

        while let Some(token) = self.peek() {
            match token.token {
                Token::RightBrace => {
                    self.advance();
                    return Ok(JsonValue::Object(map));
                }
                Token::String(s) => {
                    if key.is_none() {
                        key = Some(s);
                        self.advance();
                    } else if colon && value.is_none() {
                        value = Some(self.parse_value()?);

                        if let Some(token) = self.peek()
                            && !matches!(token.token, Token::Comma | Token::RightBrace)
                        {
                            return Err(SyntaxError::MissingDelimiter(token.position.clone()));
                        }
                    } else {
                        return Err(SyntaxError::ParseErrorPlaceHolder);
                    }
                }
                Token::Comma => {
                    if let Some(next) = self.peek_next()
                        && matches!(
                            next.token,
                            Token::Null
                                | Token::Bool(_)
                                | Token::String(_)
                                | Token::Number(_)
                                | Token::LeftBracket
                                | Token::LeftBrace
                        )
                    {
                        self.advance();
                    } else {
                        return Err(SyntaxError::ParseErrorPlaceHolder);
                    }
                }
                Token::Colon => {
                    if key.is_some()
                        && !colon
                        && let Some(next) = self.peek_next()
                        && matches!(
                            next.token,
                            Token::Null
                                | Token::Bool(_)
                                | Token::String(_)
                                | Token::Number(_)
                                | Token::LeftBracket
                                | Token::LeftBrace
                        )
                    {
                        self.advance();
                        colon = true;
                    } else {
                        return Err(SyntaxError::ParseErrorPlaceHolder);
                    }
                }
                _ => {
                    if colon && value.is_none() {
                        value = Some(self.parse_value()?);

                        if let Some(token) = self.peek()
                            && !matches!(token.token, Token::Comma | Token::RightBrace)
                        {
                            return Err(SyntaxError::MissingDelimiter(token.position.clone()));
                        }
                    } else {
                        return Err(SyntaxError::ParseErrorPlaceHolder);
                    }
                }
            }
            if let Some(k) = key.clone()
                && colon
                && let Some(v) = value.clone()
            {
                map.insert(k, v);
                key = None;
                colon = false;
                value = None;
            }
        }
        Err(SyntaxError::ParseErrorPlaceHolder)
    }

    pub fn parse(&mut self) -> Result<JsonValue, SyntaxError> {
        while let Some(token) = self.peek() {
            match token.token {
                Token::LeftBracket => self.parse_array()?,
                Token::LeftBrace => self.parse_object()?,
                _ => self.parse_object()?,
            };
        }

        Err(SyntaxError::ParseErrorPlaceHolder)
    }
}

#[cfg(test)]
mod parser_tests {
    use std::collections::HashMap;

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
            build_parser_with_pos_token_vec_from_input("[1, 2]").parse_array(),
            Ok(JsonValue::List(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0)
            ]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[[1], 1, 2]").parse_array(),
            Ok(JsonValue::List(vec![
                JsonValue::List(vec![JsonValue::Number(1.0)]),
                JsonValue::Number(1.0),
                JsonValue::Number(2.0)
            ]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[3, [1], 1, 2]").parse_array(),
            Ok(JsonValue::List(vec![
                JsonValue::Number(3.0),
                JsonValue::List(vec![JsonValue::Number(1.0)]),
                JsonValue::Number(1.0),
                JsonValue::Number(2.0)
            ]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[3, [1, 1, 2]]").parse_array(),
            Ok(JsonValue::List(vec![
                JsonValue::Number(3.0),
                JsonValue::List(vec![
                    JsonValue::Number(1.0),
                    JsonValue::Number(1.0),
                    JsonValue::Number(2.0)
                ])
            ]))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("[[1], [1]]").parse_array(),
            Ok(JsonValue::List(vec![
                JsonValue::List(vec![JsonValue::Number(1.0)]),
                JsonValue::List(vec![JsonValue::Number(1.0)]),
            ]))
        );
    }
    #[test]
    fn parse_array_fails_on_malformed() {
        assert!(
            build_parser_with_pos_token_vec_from_input("[")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[,]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1,]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1,")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1 2]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[[] 1, 2]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1, [] 2, 3]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1, [1,] 2, 3]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1, 1,] 2, 3]")
                .parse_array()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("[1, [1, 2, 3]")
                .parse_array()
                .is_err()
        );
    }

    macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
    }

    #[test]
    fn parse_object_passes_on_well_formed() {
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("{}").parse_object(),
            Ok(JsonValue::Object(HashMap::new()))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("{\"a\" : 1 }").parse_object(),
            Ok(JsonValue::Object(
                hashmap!(String::from("a") => JsonValue::Number(1.0))
            ))
        );
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("{\"a\" : 1, \"b\" : 2 }").parse_object(),
            Ok(JsonValue::Object(
                hashmap!(String::from("a") => JsonValue::Number(1.0), String::from("b") => JsonValue::Number(2.0))
            ))
        );

        let inner = JsonValue::Object(hashmap!(
                String::from("a") => JsonValue::Number(1.0),
                String::from("b") => JsonValue::Number(2.0)));
        let outer = JsonValue::Object(hashmap!(String::from("c") => JsonValue::List(vec![inner])));
        assert_eq!(
            build_parser_with_pos_token_vec_from_input("{\"c\" : [{\"a\" : 1, \"b\" : 2 }]}")
                .parse_object(),
            Ok(outer)
        );

        assert_eq!(
            build_parser_with_pos_token_vec_from_input("{\"name\": \"Vlad\"}").parse_object(),
            Ok(JsonValue::Object(hashmap!(
                String::from("name") => JsonValue::String(String::from("Vlad"))
            )))
        );

        assert_eq!(
            build_parser_with_pos_token_vec_from_input(
                "{\"active\": true, \"deleted\": false, \"data\": null}"
            )
            .parse_object(),
            Ok(JsonValue::Object(hashmap!(
                String::from("active") => JsonValue::Bool(true),
                String::from("deleted") => JsonValue::Bool(false),
                String::from("data") => JsonValue::Null
            )))
        );

        let nested_expected = JsonValue::Object(hashmap!(
            String::from("user") => JsonValue::Object(hashmap!(
                String::from("id") => JsonValue::Number(1.0),
                String::from("role") => JsonValue::String(String::from("admin"))
            ))
        ));
        assert_eq!(
            build_parser_with_pos_token_vec_from_input(
                "{\"user\": {\"id\": 1, \"role\": \"admin\"}}"
            )
            .parse_object(),
            Ok(nested_expected)
        );
    }

    #[test]
    fn parse_object_fails_on_malformed() {
        assert!(
            build_parser_with_pos_token_vec_from_input("{")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\"")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\":")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\" 1}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{ 1 : \"a\"}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{true: \"a\"}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{null: 1}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{[1]: 1}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{,}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\": 1,}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\": 1, \"b\": 2,}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\": 1 \"b\": 2}")
                .parse_object()
                .is_err()
        );
        assert!(
            build_parser_with_pos_token_vec_from_input("{\"a\": : 1}")
                .parse_object()
                .is_err()
        );
    }
}
