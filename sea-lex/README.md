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
pub enum EquationToken {
    Integer(i64),
    Asterisk,
    Slash,
    Plus,
    Minus,
}

impl TokenType for EquationToken {
    fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
        vec![
            (
                TokenCreator::Fn(Box::new(|c| Self::Integer(c.parse().unwrap()))).into(),
                Regex::new(r"^\d+").unwrap().into(),
            ),
            (Self::Asterisk.into(), "*".into()),
            (Self::Slash.into(), "/".into()),
            (Self::Plus.into(), "+".into()),
            (Self::Minus.into(), "-".into()),
            (TokenCreator::None, " ".into()), // skip spaces
        ]
    }
}

let lexer = EquationToken::lexer();
let tokens = lexer.lex("1 + 23 - 45 * 6 / 789");
let expected = vec![
    Token::new(EquationToken::Integer(1), "1", 0),
    Token::new(EquationToken::Plus, "+", 2),
    Token::new(EquationToken::Integer(23), "23", 4),
    Token::new(EquationToken::Minus, "-", 7),
    Token::new(EquationToken::Integer(45), "45", 9),
    Token::new(EquationToken::Asterisk, "*", 12),
    Token::new(EquationToken::Integer(6), "6", 14),
    Token::new(EquationToken::Slash, "/", 16),
    Token::new(EquationToken::Integer(789), "789", 18),
];
assert_eq!(tokens, Ok(expected));
```
