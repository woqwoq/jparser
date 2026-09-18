use crate::lexer::Position;

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedToken(Position, char),
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
    ParseErrorPlaceHolder,
}
