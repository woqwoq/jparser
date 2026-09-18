use crate::{
    error::SyntaxError,
    lexer::Lexer,
    parser::{JsonValue, Parser},
};

pub struct JsonDeserializer {
    input: String,
}

impl JsonDeserializer {
    pub fn new(input: &str) -> Self {
        JsonDeserializer {
            input: input.to_string(),
        }
    }

    pub fn deserialize(&mut self) -> Result<JsonValue, SyntaxError> {
        Parser::new(Lexer::new(&self.input).tokenize()?).parse()
    }
}
