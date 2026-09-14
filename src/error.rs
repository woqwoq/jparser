use crate::lexer::{Position, PositionalToken};

#[derive(Clone, Debug, PartialEq)]
pub enum JsonError {
    General(Position),
    UnexpectedToken(Position, char),
    IncompleteToken(Position),
}
