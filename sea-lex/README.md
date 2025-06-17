# `sea-lex`

SeaFlow lexer component.

Part of the [SeaFlow](https://github.com/caydenlund/seaflow) compiler toolkit.

The lexer is one of the less-interesting components of the compiler, so most of my attention will be spent elsewhere.
This means that it may be less efficient than other lexing crates, but because lexing is a small fraction of the overall work of the compiler, inefficiency is not a large concern here.
I intend to come back to this and implement a state machine generator that will be much more efficient.

## Design Overview

Given a source program, the lexer identifies the most primitive building blocks ("tokens") that will be parsed into an Abstract Syntax Tree.
These tokens include things like keywords, punctuation, and identifiers.

The overall lexer is composed of pattern matchers, ordered by priority.
Each pattern matcher will read from the source source program at a given position, and determine whether to emit a valid token type from that position.

## API Overview

Implement the `TokenType` trait for an `enum` of your token types by defining the ordered pattern matchers.
Each pattern matcher is a pair of a `TokenCreator` and a regular or expression or string literal to match.
This trait will make it possible to construct a new `Lexer` for this set of token types.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Integer(i64),
    Identifier(String),
    LParen,
    RParen,
    Asterisk,
    Slash,
    Plus,
    Minus,
}

impl TokenType for Token {
    fn matchers() -> Vec<Matcher<Self>> {
        vec![
            // Integer literals
            (
                TokenCreator::Fn(Box::new(|c| Self::Integer(c.parse().unwrap()))),
                Regex::new(r"^\d+").unwrap(),
            )
                .into(),
            // Variable names
            (
                TokenCreator::Fn(Box::new(|c| Self::Identifier(c.to_string()))),
                Regex::new(r"^[a-zA-Z_]+").unwrap(),
            )
                .into(),
            // Punctuation
            (Self::LParen, "(").into(),
            (Self::RParen, ")").into(),
            (Self::Asterisk, "*").into(),
            (Self::Slash, "/").into(),
            (Self::Plus, "+").into(),
            (Self::Minus, "-").into(),
            // Ignore whitespace
            (TokenCreator::None, Regex::new(r"^\s+").unwrap()).into(),
        ]
    }
}
```
