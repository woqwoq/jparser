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

    fn peek_next(&self) -> Option<&char> {
        self.chars.get(self.cursor + 1)
    }

    fn advance(&mut self) {
        self.cursor += 1;
        self.position.advance();
    }

    fn parse_or_err(&mut self, string: &str) -> Result<(), JsonError> {
        let position = self.position.clone(); // snapshot position before we proceed

        for remaining_char in string.chars() {
            if let Some(char) = self.peek() {
                if *char == remaining_char {
                    self.advance();
                } else {
                    return Err(JsonError::UnexpectedToken(self.position.clone(), *char));
                }
            } else {
                return Err(JsonError::IncompleteToken(position));
            }
        }

        Ok(())
    }

    fn parse_null(&mut self) -> Result<(), JsonError> {
        self.parse_or_err("null")
    }

    fn parse_bool(&mut self) -> Result<bool, JsonError> {
        let position = self.position.clone(); // snapshot position before we proceed

        if let Some(char) = self.peek() {
            match char {
                't' => {
                    self.parse_or_err("true")?;
                    Ok(true)
                }
                'f' => {
                    self.parse_or_err("false")?;
                    Ok(false)
                }
                _ => Err(JsonError::UnexpectedToken(position, *char)), // Not expected, as this
                                                                       // code is ran only when current token is 't' | 'f'
            }
        } else {
            Err(JsonError::IncompleteToken(position))
        }
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        todo!()
    }

    fn parse_number(&mut self) -> Result<f64, JsonError> {
        let position = self.position.clone(); // snapshot position before we proceed

        let mut start = true;
        let mut signed = false;
        let mut fraction = false;

        let mut number_repr = String::new();

        while let Some(char) = self.peek() {
            match char {
                '-' => {
                    if start && !signed {
                        number_repr.push(*char);
                        self.advance();
                        signed = true;
                    } else {
                        return Err(JsonError::UnexpectedToken(self.position.clone(), *char));
                    }
                }
                '0' => {
                    if start
                        && *char == '0'
                        && let Some(next) = self.peek_next()
                        && *next != '.'
                        && (next.is_ascii_digit())
                    {
                        return Err(JsonError::UnexpectedToken(self.position.clone(), *char));
                    } else {
                        number_repr.push(*char);
                        start = false;
                        self.advance();
                    }
                }
                '.' => {
                    if !start
                        && !fraction
                        && let Some(next) = self.peek_next()
                        && next.is_ascii_digit()
                    {
                        number_repr.push(*char);
                        self.advance();
                        fraction = true;
                    } else {
                        return Err(JsonError::UnexpectedToken(self.position.clone(), *char));
                    }
                }
                '1'..='9' => {
                    number_repr.push(*char);
                    start = false;
                    self.advance();
                }
                _ => {
                    if !matches!(char, ' ' | ',' | '}' | ']' | '\n') {
                        return Err(JsonError::UnexpectedToken(self.position.clone(), *char));
                    }
                    break;
                }
            }
        }

        if (signed && number_repr.len() == 1) || start {
            return Err(JsonError::IncompleteToken(position));
        }

        Ok(number_repr.parse().unwrap())
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
    fn parse_or_err_test_ok_base() {
        assert!(
            build_lexer_with_input("hello")
                .parse_or_err("hello")
                .is_ok()
        );
        assert!(build_lexer_with_input("").parse_or_err("").is_ok());
        assert!(build_lexer_with_input("a").parse_or_err("a").is_ok());
        assert!(build_lexer_with_input("ab").parse_or_err("a").is_ok());
        assert!(build_lexer_with_input("AB").parse_or_err("AB").is_ok());
    }

    #[test]
    fn parse_or_err_test_ok_overkill() {
        let mut overkill_lexer = build_lexer_with_input("ABcdEfG");
        assert!(overkill_lexer.parse_or_err("AB").is_ok());
        assert!(overkill_lexer.parse_or_err("c").is_ok());
        assert!(overkill_lexer.parse_or_err("dEf").is_ok());
        assert!(overkill_lexer.parse_or_err("G").is_ok());
    }

    #[test]
    fn parse_or_err_test_err() {
        assert!(build_lexer_with_input("a").parse_or_err("b").is_err());
        assert!(build_lexer_with_input("").parse_or_err("a").is_err());
        assert!(build_lexer_with_input("a").parse_or_err("ab").is_err());
        assert!(build_lexer_with_input("AB").parse_or_err("ab").is_err());
        assert!(build_lexer_with_input("ab").parse_or_err("AB").is_err());
        assert!(build_lexer_with_input("\"ab\"").parse_or_err("ab").is_err());
    }
    #[test]
    fn parse_or_err_test_err_valid_message() {
        assert!(matches!(
            build_lexer_with_input("worl").parse_or_err("world"),
            Err(JsonError::IncompleteToken(_))
        ));
        assert!(matches!(
            build_lexer_with_input("xorld").parse_or_err("world"),
            Err(JsonError::UnexpectedToken(
                Position { line: 1, col: 1 },
                'x'
            ))
        ));
        assert!(matches!(
            build_lexer_with_input("worlx").parse_or_err("world"),
            Err(JsonError::UnexpectedToken(
                Position { line: 1, col: 5 },
                'x'
            ))
        ));
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

    #[test]
    fn parse_bool_passes_on_well_formed() {
        let mut lexer = build_lexer_with_input("true");
        assert_eq!(Ok(true), lexer.parse_bool());
        assert_eq!(Ok(false), build_lexer_with_input("false").parse_bool());
        assert_eq!(lexer.cursor, 4);
        assert_eq!(lexer.position.col, 5);
    }

    #[test]
    fn parse_bool_fails_on_malformed() {
        assert!(matches!(
            build_lexer_with_input("").parse_bool(),
            Err(JsonError::IncompleteToken(_))
        ));
        assert!(matches!(
            build_lexer_with_input("tru").parse_bool(),
            Err(JsonError::IncompleteToken(_))
        ));
        assert!(matches!(
            build_lexer_with_input("trux").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("rue").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("TRUE").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("\"TRUE\"").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("fals").parse_bool(),
            Err(JsonError::IncompleteToken(_))
        ));
        assert!(matches!(
            build_lexer_with_input("faslx").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("alse").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("FALSE").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
        assert!(matches!(
            build_lexer_with_input("\"FALSE\"").parse_bool(),
            Err(JsonError::UnexpectedToken(_, _))
        ));
    }

    #[test]
    fn parse_number_pass_on_well_formed_base() {
        assert_eq!(build_lexer_with_input("0").parse_number(), Ok(0.0));
        assert_eq!(build_lexer_with_input("-1").parse_number(), Ok(-1.0));
        assert_eq!(build_lexer_with_input("123").parse_number(), Ok(123.0));
    }

    #[test]
    fn parse_number_pass_on_well_formed_delimiters() {
        assert_eq!(build_lexer_with_input("123,").parse_number(), Ok(123.0));
        assert_eq!(build_lexer_with_input("123}").parse_number(), Ok(123.0));
        assert_eq!(build_lexer_with_input("123]").parse_number(), Ok(123.0));
        assert_eq!(build_lexer_with_input("123 ").parse_number(), Ok(123.0));
        assert_eq!(build_lexer_with_input("123\n").parse_number(), Ok(123.0));
    }

    #[test]
    fn parse_number_pass_on_well_formed_fraction() {
        assert_eq!(build_lexer_with_input("123.0").parse_number(), Ok(123.0));
        assert_eq!(
            build_lexer_with_input("123.123").parse_number(),
            Ok(123.123)
        );
        assert_eq!(build_lexer_with_input("0.123").parse_number(), Ok(0.123));
        assert_eq!(build_lexer_with_input("-0.123").parse_number(), Ok(-0.123));
        assert_eq!(
            build_lexer_with_input("-0.000123").parse_number(),
            Ok(-0.000123)
        );
    }

    #[test]
    fn parse_number_fails_on_malormed() {
        assert!(build_lexer_with_input("").parse_number().is_err());
        assert!(build_lexer_with_input("NaN").parse_number().is_err());

        assert!(build_lexer_with_input("-").parse_number().is_err());
        assert!(build_lexer_with_input("+1").parse_number().is_err());
        assert!(build_lexer_with_input("-01").parse_number().is_err());
        assert!(build_lexer_with_input("0-1").parse_number().is_err());
        assert!(build_lexer_with_input("--1").parse_number().is_err());

        assert!(build_lexer_with_input("00").parse_number().is_err());
        assert!(build_lexer_with_input("01").parse_number().is_err());
        assert!(build_lexer_with_input("001").parse_number().is_err());

        assert!(build_lexer_with_input("1)").parse_number().is_err());
        assert!(build_lexer_with_input("1f").parse_number().is_err());

        assert!(build_lexer_with_input("0.").parse_number().is_err());
        assert!(build_lexer_with_input("0.n").parse_number().is_err());
        assert!(build_lexer_with_input(".1").parse_number().is_err());
        assert!(build_lexer_with_input("0..1").parse_number().is_err());
    }
}
