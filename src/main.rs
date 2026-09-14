use std::collections::HashMap;

mod error;
mod lexer;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    List(Vec<JsonValue>),

    String(String),
    Number(f64),
    Bool(bool),

    Null,
}

fn main() {
    println!("Hello, world!");
}
