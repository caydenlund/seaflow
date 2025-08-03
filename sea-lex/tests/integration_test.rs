use sea_lex::Token;

/// Parser function for integer tokens
fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

#[derive(Debug, Clone, PartialEq, Token)]
#[token(skip = r"\s+")]
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

#[test]
fn test_math_lexer() {
    let lexer = MathToken::lexer("12 + 34 * (56 - 78)");
    
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[0].kind, MathToken::Number(12));
    assert_eq!(tokens[1].kind, MathToken::Plus);
    assert_eq!(tokens[2].kind, MathToken::Number(34));
    assert_eq!(tokens[3].kind, MathToken::Multiply);
    assert_eq!(tokens[4].kind, MathToken::LeftParen);
    assert_eq!(tokens[5].kind, MathToken::Number(56));
    assert_eq!(tokens[6].kind, MathToken::Minus);
    assert_eq!(tokens[7].kind, MathToken::Number(78));
    assert_eq!(tokens[8].kind, MathToken::RightParen);
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