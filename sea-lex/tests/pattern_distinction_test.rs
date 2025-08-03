use sea_lex::Token;

/// Parser function for integer tokens
fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

/// Parser function for string tokens
fn parse_string(s: &str) -> String {
    s.to_string()
}

#[derive(Debug, Clone, PartialEq, Token)]
#[skip(r"\s+")]  // Skip whitespace (regex)
enum PatternTestToken {
    // Put specific literals first to take precedence over general regex patterns
    #[token("\\d")]
    LiteralBackslashD,
    
    #[token("+")]
    Plus,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("*")]
    Asterisk,
    
    #[token(".")]
    Dot,
    
    #[token("[")]
    LeftBracket,
    
    #[token("]")]
    RightBracket,
    
    // Regex patterns - should match multiple characters with special meaning
    #[token(r"\d+", parse_int)]
    Number(i64),
    
    #[token(r"[a-zA-Z]+", parse_string)]
    Word(String),
    
    #[token(r"\w+", parse_string)]
    Identifier(String),
}

#[test]
fn test_regex_digit_pattern() {
    // r"\d+" should match one or more digits as a regex
    let lexer = PatternTestToken::lexer("123 456 7");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, PatternTestToken::Number(123));
    assert_eq!(tokens[1].kind, PatternTestToken::Number(456));
    assert_eq!(tokens[2].kind, PatternTestToken::Number(7));
}

#[test]
fn test_literal_backslash_d() {
    // "\\d" should match exactly the literal string "\d"
    let input = "\\d";  // This is a backslash followed by 'd'
    let lexer = PatternTestToken::lexer(input);
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, PatternTestToken::LiteralBackslashD);
    assert_eq!(tokens[0].text, "\\d");
}

#[test] 
fn test_regex_vs_literal_distinction() {
    // Test that "123" matches r"\d+" (regex) but not "\\d" (literal)
    let lexer = PatternTestToken::lexer("123");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    // Should match the regex pattern r"\d+", not the literal "\\d"
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, PatternTestToken::Number(123));
}

#[test]
fn test_literal_plus_vs_regex_meaning() {
    // "+" should match exactly one plus character
    // If it were regex, it would be invalid (nothing to repeat)
    let lexer = PatternTestToken::lexer("+ ++ +++");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    
    // Should match each + as a separate literal token
    // Input: "+ ++ +++" = 6 plus characters total
    assert_eq!(tokens.len(), 6);
    for token in &tokens {
        assert_eq!(token.kind, PatternTestToken::Plus);
        assert_eq!(token.text, "+");
    }
}

#[test]
fn test_literal_dot_vs_regex_meaning() {
    // "." should match exactly the dot character
    // If it were regex, it would match any character
    let lexer = PatternTestToken::lexer(". a b");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    // Should only match the literal dot, not the letters
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, PatternTestToken::Dot);
    assert_eq!(tokens[0].text, ".");
    // The letters 'a' and 'b' should match as words via r"[a-zA-Z]+" (comes first)
    assert_eq!(tokens[1].kind, PatternTestToken::Word("a".to_string()));
    assert_eq!(tokens[2].kind, PatternTestToken::Word("b".to_string()));
}

#[test]
fn test_literal_brackets_vs_regex_meaning() {
    // "[" and "]" should match exactly those characters
    // If they were regex, "[" would start a character class
    let lexer = PatternTestToken::lexer("[ ] [abc]");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].kind, PatternTestToken::LeftBracket);
    assert_eq!(tokens[0].text, "[");
    assert_eq!(tokens[1].kind, PatternTestToken::RightBracket);
    assert_eq!(tokens[1].text, "]");
    assert_eq!(tokens[2].kind, PatternTestToken::LeftBracket);
    assert_eq!(tokens[3].kind, PatternTestToken::Word("abc".to_string()));
    assert_eq!(tokens[4].kind, PatternTestToken::RightBracket);
}

#[test]
fn test_literal_asterisk_vs_regex_meaning() {
    // "*" should match exactly the asterisk character
    // If it were regex, it would be invalid (nothing to repeat)
    let lexer = PatternTestToken::lexer("* ** ***");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    // Should match each * as a separate literal token
    assert_eq!(tokens.len(), 6);
    for token in &tokens {
        assert_eq!(token.kind, PatternTestToken::Asterisk);
        assert_eq!(token.text, "*");
    }
}

#[test]
fn test_regex_character_classes() {
    // r"[a-zA-Z]+" should match sequences of letters
    let lexer = PatternTestToken::lexer("hello WORLD test123");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    
    // "hello" and "WORLD" should match r"[a-zA-Z]+" (Word)
    // "test123" should be split: "test" matches r"[a-zA-Z]+" and "123" matches r"\d+"
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].kind, PatternTestToken::Word("hello".to_string()));
    assert_eq!(tokens[1].kind, PatternTestToken::Word("WORLD".to_string()));
    assert_eq!(tokens[2].kind, PatternTestToken::Word("test".to_string()));
    assert_eq!(tokens[3].kind, PatternTestToken::Number(123));
}

#[test]
fn test_complex_literal_vs_regex() {
    // Test input that would behave very differently if patterns were misinterpreted
    let lexer = PatternTestToken::lexer("42 + \\d * [test]");
    let tokens: Vec<_> = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].kind, PatternTestToken::Number(42));
    assert_eq!(tokens[1].kind, PatternTestToken::Plus);
    assert_eq!(tokens[2].kind, PatternTestToken::LiteralBackslashD);
    assert_eq!(tokens[3].kind, PatternTestToken::Asterisk);
    assert_eq!(tokens[4].kind, PatternTestToken::LeftBracket);
    assert_eq!(tokens[5].kind, PatternTestToken::Word("test".to_string()));
    assert_eq!(tokens[6].kind, PatternTestToken::RightBracket);
}