use crate::error::JsonError;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    line: usize,
    col: usize,
}

impl Position {
    pub fn new() -> Self {
        Position { line: 1, col: 1 }
    }

    pub fn from(line: usize, col: usize) -> Self {
        Position { line, col }
    }

    pub fn advance_newline(&mut self) {
        self.line += 1;
        self.col = 1;
    }

    pub fn advance(&mut self) {
        self.col += 1;
    }
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

    Null,
    Bool(bool),
    String(String),
    Number(f64),
}

pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,

    position: Position,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect(),
            cursor: 0,
            position: Position::new(),
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.cursor)
    }

    fn advance(&mut self) {
        self.cursor += 1;
        self.position.advance();
    }

    fn parse_null(&mut self) -> Result<(), JsonError> {
        let position = self.position.clone(); // snapshot position before we proceed

        for remaining_char in "null".chars() {
            if let Some(char) = self.peek() {
                if *char == remaining_char {
                    self.advance();
                } else {
                    return Err(JsonError::UnexpectedToken(position, *char));
                }
            } else {
                return Err(JsonError::IncompleteToken(position));
            }
        }

        Ok(())
    }

    fn parse_bool(&mut self) -> Result<bool, JsonError> {
        todo!()
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        todo!()
    }

    fn parse_number(&mut self) -> Result<f64, JsonError> {
        todo!()
    }

    pub fn tokenize(&mut self) -> Result<Vec<PositionalToken>, JsonError> {
        let mut tokens: Vec<PositionalToken> = Vec::new();

        while let Some(char) = self.peek() {
            let position = self.position.clone();
            match char {
                // These characters do not emit a token and should be ignored, so we simply advance
                // over them.
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                // Newline character doesn't emit a token, so we simply advance over it and do
                // corresponding changes to our position tracker.
                '\n' => {
                    self.advance();
                    self.position.advance_newline();
                }

                // Emit a token and advance
                '{' => {
                    self.advance();
                    tokens.push(PositionalToken {
                        token: Token::LeftBrace,
                        position,
                    });
                }
                '}' => {
                    self.advance();
                    tokens.push(PositionalToken {
                        token: Token::RightBrace,
                        position,
                    });
                }
                '[' => {
                    self.advance();
                    tokens.push(PositionalToken {
                        token: Token::LeftBracket,
                        position,
                    });
                }
                ']' => {
                    self.advance();
                    tokens.push(PositionalToken {
                        token: Token::RightBracket,
                        position,
                    });
                }
                ',' => {
                    self.advance();
                    tokens.push(PositionalToken {
                        token: Token::Comma,
                        position,
                    });
                }
                ':' => {
                    self.advance();
                    tokens.push(PositionalToken {
                        token: Token::Colon,
                        position,
                    });
                }

                // Values like null, bool, string, number require further handling as they consist
                // of multiple characters and will require a nested loop.
                'n' => {
                    self.parse_null()?;

                    tokens.push(PositionalToken {
                        token: Token::Null,
                        position,
                    });
                }
                't' | 'f' => {
                    tokens.push(PositionalToken {
                        token: Token::Bool(self.parse_bool()?),
                        position,
                    });
                }
                '"' => {
                    tokens.push(PositionalToken {
                        token: Token::String(self.parse_string()?),
                        position,
                    });
                }
                '-' | '0'..='9' => {
                    tokens.push(PositionalToken {
                        token: Token::Number(self.parse_number()?),
                        position,
                    });
                }
                _ => return Err(JsonError::General(self.position.clone())),
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::{
        error::JsonError,
        lexer::{Lexer, Position},
    };

    fn build_lexer_with_input(input: &str) -> Lexer {
        Lexer::new(input)
    }

    #[test]
    fn parse_null_passes_on_well_formed() {
        let mut lexer = build_lexer_with_input("null");
        assert_eq!(Ok(()), lexer.parse_null());
        assert_eq!(lexer.cursor, 4);
        assert_eq!(lexer.position.col, 5);
    }

    #[test]
    fn parse_null_fails_on_malformed() {
        assert!(matches!(
            build_lexer_with_input("").parse_null(),
            Err(JsonError::IncompleteToken(_))
        ));
        assert!(matches!(
            build_lexer_with_input("nul").parse_null(),
            Err(JsonError::IncompleteToken(_))
        ));
        assert!(matches!(
            build_lexer_with_input("ull").parse_null(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("full").parse_null(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
    }

    #[test]
    fn parse_null_fails_with_valid_position() {
        let expected_position = Position::from(1, 3);
        let mut lexer = build_lexer_with_input("full");
        lexer.position = expected_position.clone();

        assert!(matches!(
            lexer.parse_null(),
            Err(JsonError::UnexpectedToken(
                Position { line: 1, col: 3 },
                'f'
            ))
        ));
    }
}
