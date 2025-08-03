use sea_lex::Token;

#[derive(Debug, Clone, PartialEq, Token)]
#[skip(r"\s+")]
enum MathToken {
    #[token(r"\d+", str::parse)]
    Number(i64),

    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*", String::from)]
    Identifier(String),

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

#[test]
fn test_math_lexer() {
    let tokens: Vec<_> = MathToken::tokenize("foo - 12 + 34 * (56 - 78) / bar123")
        .collect()
        .expect("unable to tokenize input");

    let token_kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();

    use MathToken::*;
    assert_eq!(
        token_kinds,
        [
            Identifier("foo".into()),
            Minus,
            Number(12),
            Plus,
            Number(34),
            Multiply,
            LeftParen,
            Number(56),
            Minus,
            Number(78),
            RightParen,
            Divide,
            Identifier("bar123".into()),
        ]
    );
}

#[test]
fn test_token_positions() {
    let lexer = MathToken::lexer("12+34");

    let tokens: Vec<_> = lexer.collect().unwrap();

    assert_eq!(tokens[0].start, 0);
    assert_eq!(tokens[0].end, 2);
    assert_eq!(tokens[0].text, "12");

    assert_eq!(tokens[1].start, 2);
    assert_eq!(tokens[1].end, 3);
    assert_eq!(tokens[1].text, "+");

    assert_eq!(tokens[2].start, 3);
    assert_eq!(tokens[2].end, 5);
    assert_eq!(tokens[2].text, "34");
}
