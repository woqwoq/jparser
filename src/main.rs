use std::collections::HashMap;

mod error;
mod lexer;
mod parser;

#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>), // { }
    List(Vec<JsonValue>),               // [ ]
    String(String),                     // "hello world"
    Number(f64),                        // -3.14159e-1
    Bool(bool),                         // true / false
    Null,                               // null
}

fn main() {
    println!("Hello, world!");
}
