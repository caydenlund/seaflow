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
#[skip(r"\s+")]  // Skip whitespace using regex
enum KeyDistinctionToken {
    // REGEX PATTERNS: Use r"..." syntax
    #[token(r"\d+", parse_int)]
    RegexDigits(i64),       // Matches sequences of digits: 123, 456, etc.
    
    #[token(r"[a-zA-Z]+", parse_string)]
    RegexLetters(String),   // Matches sequences of letters: hello, WORLD, etc.
    
    // LITERAL PATTERNS: Use "..." syntax  
    #[token("\\d")]
    LiteralBackslashD,      // Matches exactly "\d" (backslash followed by d)
    
    #[token("+")]
    LiteralPlus,            // Matches exactly "+"
    
    #[token(".")]
    LiteralDot,             // Matches exactly "."
    
    #[token("*")]
    LiteralAsterisk,        // Matches exactly "*"
}

/// Test that demonstrates the key distinction between regex and literal patterns
#[test]
fn test_key_regex_vs_literal_distinction() {
    // Test 1: r"\d+" (regex) vs "\\d" (literal)
    
    // This should match the REGEX pattern r"\d+" 
    let lexer1 = KeyDistinctionToken::lexer("123");
    let tokens1 = lexer1.collect().unwrap();
    assert_eq!(tokens1.len(), 1);
    assert_eq!(tokens1[0].kind, KeyDistinctionToken::RegexDigits(123));
    
    // This should match the LITERAL pattern "\\d"
    let lexer2 = KeyDistinctionToken::lexer("\\d");
    let tokens2 = lexer2.collect().unwrap();
    assert_eq!(tokens2.len(), 1);
    assert_eq!(tokens2[0].kind, KeyDistinctionToken::LiteralBackslashD);
    assert_eq!(tokens2[0].text, "\\d");
    
    // Test 2: Literal characters that have special meaning in regex
    
    // "+" matches exactly one plus (literal), not "one or more" (regex meaning)
    let lexer3 = KeyDistinctionToken::lexer("++");
    let tokens3 = lexer3.collect().unwrap();
    assert_eq!(tokens3.len(), 2); // Two separate + tokens
    assert_eq!(tokens3[0].kind, KeyDistinctionToken::LiteralPlus);
    assert_eq!(tokens3[1].kind, KeyDistinctionToken::LiteralPlus);
    
    // "." matches exactly one dot (literal), not "any character" (regex meaning)
    let lexer4 = KeyDistinctionToken::lexer(". a");
    let tokens4 = lexer4.collect().unwrap();
    assert_eq!(tokens4.len(), 2);
    assert_eq!(tokens4[0].kind, KeyDistinctionToken::LiteralDot);
    assert_eq!(tokens4[0].text, ".");
    assert_eq!(tokens4[1].kind, KeyDistinctionToken::RegexLetters("a".to_string()));
    
    // "*" matches exactly one asterisk (literal), not "zero or more" (regex meaning)
    let lexer5 = KeyDistinctionToken::lexer("**");
    let tokens5 = lexer5.collect().unwrap();
    assert_eq!(tokens5.len(), 2); // Two separate * tokens
    assert_eq!(tokens5[0].kind, KeyDistinctionToken::LiteralAsterisk);
    assert_eq!(tokens5[1].kind, KeyDistinctionToken::LiteralAsterisk);
}

/// Test that shows complex patterns work correctly
#[test]
fn test_complex_pattern_combinations() {
    // Mix of regex and literal patterns in one input
    let input = "hello 123 \\d + . * world";
    let lexer = KeyDistinctionToken::lexer(input);
    let tokens = lexer.collect().unwrap();
    
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].kind, KeyDistinctionToken::RegexLetters("hello".to_string()));
    assert_eq!(tokens[1].kind, KeyDistinctionToken::RegexDigits(123));
    assert_eq!(tokens[2].kind, KeyDistinctionToken::LiteralBackslashD);
    assert_eq!(tokens[3].kind, KeyDistinctionToken::LiteralPlus);
    assert_eq!(tokens[4].kind, KeyDistinctionToken::LiteralDot);
    assert_eq!(tokens[5].kind, KeyDistinctionToken::LiteralAsterisk);
    assert_eq!(tokens[6].kind, KeyDistinctionToken::RegexLetters("world".to_string()));
}

/// Test that shows skip patterns can also be regex or literal
#[test]
fn test_skip_pattern_distinction() {
    #[derive(Debug, Clone, PartialEq, Token)]
    #[skip(r"\s+")]      // Regex: skip any whitespace
    #[skip("//")]        // Literal: skip exactly "//"
    enum SkipTestToken {
        #[token(r"\w+", parse_string)]
        Word(String),
    }
    
    let input = "hello//world   test";
    let lexer = SkipTestToken::lexer(input);
    let tokens = lexer.collect().unwrap();
    
    
    // Should skip the "//" literally and the whitespace via regex  
    // Actually, we should get: "hello", "world", "test" because "//" and whitespace are skipped
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, SkipTestToken::Word("hello".to_string()));
    assert_eq!(tokens[1].kind, SkipTestToken::Word("world".to_string()));
    assert_eq!(tokens[2].kind, SkipTestToken::Word("test".to_string()));
}