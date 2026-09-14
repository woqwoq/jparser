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
            position: Position { line: 1, col: 1 },
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.cursor)
    }

    fn advance(&mut self) {
        self.cursor += 1;
        self.position.col += 1;
    }

    fn parse_null(&mut self) -> Result<(), JsonError> {
        todo!()
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
                    self.position.line += 1;
                    self.position.col = 1;
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
        Err(JsonError::General(self.position.clone()))
    }
}
