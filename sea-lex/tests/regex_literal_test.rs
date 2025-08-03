use sea_lex::Token;

/// Parser function for integer tokens
fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

#[derive(Debug, Clone, PartialEq, Token)]
#[token(skip = r"\s+")]  // Skip whitespace
enum TestToken {
    // Regex patterns use r"..." syntax
    #[token(r"\d+", parse_int)]
    Number(i64),
    
    // Put specific literals before general patterns
    #[token("literal")]
    Literal,
    
    #[token(r"[a-zA-Z_][a-zA-Z0-9_]*", String::from)]
    Identifier(String),
    
    // Literal patterns use "..." syntax  
    #[token("+")]
    Plus,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
}

#[test]
fn test_regex_vs_literal() {
    let lexer = TestToken::lexer("123 + identifier ( literal )");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].kind, TestToken::Number(123));
    assert_eq!(tokens[1].kind, TestToken::Plus);
    assert_eq!(tokens[2].kind, TestToken::Identifier("identifier".to_string()));
    assert_eq!(tokens[3].kind, TestToken::LeftParen);
    assert_eq!(tokens[4].kind, TestToken::Literal);
    assert_eq!(tokens[5].kind, TestToken::RightParen);
}

#[test]
fn test_regex_pattern_parsing() {
    // Test that \d+ regex works correctly
    let lexer = TestToken::lexer("42 + 999");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens[0].kind, TestToken::Number(42));
    assert_eq!(tokens[1].kind, TestToken::Plus);
    assert_eq!(tokens[2].kind, TestToken::Number(999));
}

#[test] 
fn test_literal_pattern_parsing() {
    // Test that literal + matches only the + character
    let lexer = TestToken::lexer("++");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    // Should match two separate + tokens
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TestToken::Plus);
    assert_eq!(tokens[1].kind, TestToken::Plus);
}