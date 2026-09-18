use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

use crate::{
    error::SyntaxError,
    lexer::Lexer,
    parser::{JsonValue, Parser},
};

#[derive(Debug)]
#[allow(unused)]
pub enum JsonDeserializeError {
    FileDoesNotExist(PathBuf),
    UnknownFileInterraction(Error),
    FileRead(Error),
}

pub struct JsonDeserializer {
    input: String,
}

impl From<&str> for JsonDeserializer {
    fn from(input: &str) -> Self {
        JsonDeserializer {
            input: input.to_string(),
        }
    }
}

impl From<String> for JsonDeserializer {
    fn from(input: String) -> Self {
        JsonDeserializer { input }
    }
}

impl TryFrom<PathBuf> for JsonDeserializer {
    type Error = JsonDeserializeError;

    fn try_from(path: PathBuf) -> Result<Self, JsonDeserializeError> {
        if path.exists() {
            match File::open(path) {
                Ok(mut file) => {
                    let mut buf = String::new();
                    file.read_to_string(&mut buf)
                        .map_err(JsonDeserializeError::FileRead)?;
                    Ok(JsonDeserializer { input: buf })
                }
                Err(e) => Err(JsonDeserializeError::UnknownFileInterraction(e)),
            }
        } else {
            Err(JsonDeserializeError::FileDoesNotExist(path))
        }
    }
}

impl JsonDeserializer {
    pub fn deserialize(&mut self) -> Result<JsonValue, SyntaxError> {
        Parser::new(Lexer::new(&self.input).tokenize()?).parse()
    }
}
