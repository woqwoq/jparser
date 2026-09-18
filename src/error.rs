use crate::lexer::{Position, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedCharacter(Position, char),
    BadEscapedCharacter(Position, char),
    UnescapedControlCharacter(Position, char),
    UnterminatedStringLiteral(Position, String),
    UnterminatedFractionalNumber(Position, String),
    ExponentMissingNumber(Position, String),
    LeadingZeroForbidden(Position),
    IncompleteToken(Position),
    NumberParse(Position, String),

    MissingDelimiter(Position),
    TrailingCommaNotAllowed(Position),
    ExpectedObjectKey(Position),
    ExpectedColon(Position),
    UnterminatedArray(Position),
    UnterminatedObject(Position),
    UnexpectedEndOfInput(Position),
    InvalidToken(Position, Token),
    TrailingTokens(Position, Token),
}
