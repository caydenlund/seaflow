# `sea-lex`

A derive-based lexer for the [SeaFlow](https://github.com/caydenlund/seaflow) compiler toolkit.

## Overview

`sea-lex` provides an ergonomic, derive-based API for building lexers.
Define your tokens as an enum with attributes, and the derive macro generates a complete lexer implementation.

## Quick Start

```rust
use sea_lex::Token;

#[derive(Debug, Clone, PartialEq, Token)]
#[token(skip = r"\s+")]  // Skip whitespace
enum MathToken {
    #[token(r"\d+", parse_int)]
    Number(i64),
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Multiply,
    
    #[token("/")]
    Divide,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
}

fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

fn main() {
    let mut lexer = MathToken::lexer("12 + 34 * (56 - 78)");
    
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
    }
}
```

## Token Attributes

### Pattern Types

**Literal Patterns** - Use regular quoted strings `"pattern"`:
```rust
#[token("if")]           // Matches exactly "if"
Keyword,

#[token("+")]            // Matches exactly "+"
Plus,
```

**Regex Patterns** - Use raw strings `r"pattern"`:
```rust
#[token(r"\d+")]         // Matches one or more digits
Number,

#[token(r"[a-zA-Z_]\w*")] // Matches identifiers
Identifier,
```

### `#[token(pattern)]`
Simple token without data:
```rust
#[token("if")]           // Literal
Keyword,

#[token(r"\d+")]         // Regex
Number,
```

### `#[token(pattern, parser)]`
Token with data, using a parser function:
```rust
#[token(r"\d+", parse_int)]
Number(i64),

#[token(r"[a-zA-Z_]\w*", String::from)]
Identifier(String),

#[token("true", |_| true)]
Boolean(bool),
```

### `#[token(skip = pattern)]` (on enum)
Automatically skips matched patterns:
```rust
#[derive(Token)]
#[token(skip = r"\s+")]        // Skip whitespace (regex)
#[token(skip = "//")]          // Skip literal "//" (literal)
enum MyToken { ... }
```

You can specify multiple skip patterns:
```rust
#[derive(Token)]
#[token(skip = r"\s+")]        // Skip whitespace
#[token(skip = r"//[^\n]*")]   // Skip line comments
#[token(skip = r"/\*[^*]*\*/")]// Skip block comments
enum MyToken { ... }
```

## Advanced Examples

### Programming Language Lexer
```rust
#[derive(Debug, Clone, PartialEq, Token)]
#[token(skip = r"\s+")]
#[token(skip = r"//[^\n]*")]
enum LangToken {
    // Literals
    #[token(r"\d+", parse_int)]
    Integer(i64),
    
    #[token(r#""([^"\\]|\\.)*""#, parse_string)]
    String(String),
    
    // Keywords
    #[token("fn")]
    Function,
    
    #[token("let")]
    Let,
    
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    // Identifiers
    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(String),
    
    // Operators
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    #[token("=")]
    Assign,
    
    // Punctuation
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("{")]
    LeftBrace,
    
    #[token("}")]
    RightBrace,
    
    #[token(";")]
    Semicolon,
}

fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

fn parse_string(s: &str) -> String {
    // Remove quotes and handle escape sequences
    s[1..s.len()-1].replace(r#"\""#, r#"""#)
}
```

## Token Information

Each token includes position information:
```rust
let mut lexer = MyToken::lexer("hello world");
while let Some(token) = lexer.next() {
    println!("Token: {:?}", token.kind);
    println!("Text: '{}'", token.text);
    println!("Position: {}..{}", token.start, token.end);
}
```

## Error Handling

The lexer returns `LexError` for unrecognized input:
```rust
match MyToken::lexer("invalid @#$").collect() {
    Ok(tokens) => println!("Tokens: {:?}", tokens),
    Err(error) => println!("Lex error at position {}: {}", error.position, error.message),
}
```

## Performance

The derive macro generates efficient lexers using the following:
- Compile-time regular expression compilation
- Ordered pattern matching (longest match wins)
- Zero-copy string slicing where possible
- Minimal allocation for token values

## Design Philosophy

`sea-lex` prioritizes the following:
1. **Ergonomics**—Minimal boilerplate, declarative syntax
2. **Safety**—Compile-time validation, type-safe parsing
3. **Performance**—Efficient generated code, minimal runtime overhead
4. **Simplicity**—Clear, predictable behavior

## License

Apache-2.0 or MIT, at your option
