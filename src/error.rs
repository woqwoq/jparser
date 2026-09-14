use crate::lexer::Position;

pub enum JsonError {
    General(Position),
}
