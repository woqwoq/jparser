# jparser

A handwritten, pure-Rust JSON parser and deserializer built from scratch with zero runtime dependencies.

## Features

- Recursive descent-based with two-pass pipeline separating tokenization (`Lexer`) from AST construction (`Parser`).
- Rich `SyntaxError` reporting with exact line and column positions (`Position { line, col }`) for syntax violations.
- RFC 8259 Compliant: Complete support for JSON objects, arrays, strings (with escape sequences), numbers (fractions, exponents, signs, leading zero validation), booleans, and null.
---

## Usage

Add `jparser` to your project and parse JSON from a string or file:

```rust
use jparser::JsonDeserializer;

fn main() {
    let json = r#"{"name": "Alice", "age": 30, "active": true}"#;

    match JsonDeserializer::from(json).deserialize() {
        Ok(value) => println!("Parsed AST: {:#?}", value),
        Err(err) => eprintln!("Syntax error: {:?}", err),
    }
}
```

### Parsing from a File

```rust
use std::path::PathBuf;
use jparser::JsonDeserializer;

let path = PathBuf::from("config.json");
let parsed = JsonDeserializer::try_from(path)
    .expect("File not found")
    .deserialize()
    .expect("Invalid JSON syntax");
```

---

## Architecture & Project Layout

```
src/
├── lib.rs           # Crate root exposing public API & re-exports
├── main.rs          # CLI runner & usage examples
├── lexer.rs         # Tokenizer scanning input into PositionalToken stream
├── parser.rs        # Recursive descent parser building JsonValue AST
├── deserializer.rs  # High-level entrypoint (JsonDeserializer)
└── error.rs         # Granular SyntaxError enum and Position tracking
```

---

## Development

### Run Unit Tests
```bash
cargo test
```

### Check Test Coverage
Requires [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov):
```bash
cargo llvm-cov
```
