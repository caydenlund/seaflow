use sea_lex::{TokenCreator, Lexer};

#[derive(Debug, Clone, PartialEq)]
enum TestToken {
    Number(i64),
    Plus,
}

fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

fn main() {
    println!("Testing manual lexer creation...");
    
    let matchers = vec![
        (TokenCreator::Function(|s| TestToken::Number(parse_int(s))), r"\d+", true), // regex
        (TokenCreator::Unit(TestToken::Plus), "+", false), // literal
    ];
    let skip_patterns = vec![(r"\s+", true)]; // regex
    
    println!("Creating lexer with patterns: {:?}", matchers.iter().map(|(_, p, is_regex)| (p, is_regex)).collect::<Vec<_>>());
    
    match Lexer::new("12 + 34", matchers, skip_patterns) {
        Ok(lexer) => {
            println!("Lexer created successfully");
            match lexer.collect() {
                Ok(tokens) => println!("Tokens: {:?}", tokens),
                Err(e) => println!("Lex error: {:?}", e),
            }
        }
        Err(e) => println!("Failed to create lexer: {:?}", e),
    }
}