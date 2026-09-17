use crate::lexer::Position;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonError {
    General(Position),
    UnexpectedToken(Position, char),
    UnterminatedStringLiteral(Position, String),
    IncompleteToken(Position),
    NumberParse(Position, String),
}
