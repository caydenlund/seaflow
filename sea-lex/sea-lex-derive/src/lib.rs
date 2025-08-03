use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Meta, Variant};

#[proc_macro_derive(Token, attributes(token, skip))]
pub fn derive_token(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(&input, "Token can only be derived for enums")
            .to_compile_error()
            .into();
    };

    // Parse enum-level attributes for skip patterns
    let skip_patterns = extract_skip_patterns(&input.attrs);

    // Parse variant-level token patterns
    let mut token_matchers = Vec::new();
    for variant in &data_enum.variants {
        if let Some(matcher) = extract_token_matcher(variant) {
            token_matchers.push(matcher);
        }
    }

    let matcher_implementations = token_matchers.iter().map(|matcher| {
        let pattern = &matcher.pattern;
        let is_regex = matcher.is_regex;
        match &matcher.creator {
            TokenCreatorType::Unit(variant_name) => {
                quote! {
                    (::sea_lex::TokenCreator::Unit(Self::#variant_name), #pattern, #is_regex)
                }
            }
            TokenCreatorType::Function(variant_name, func_name) => {
                // Try to parse as a path first, if that fails, parse as an expression
                // Handle special case for String::from
                if func_name == "String::from" {
                    quote! {
                        (::sea_lex::TokenCreator::Parser(std::sync::Arc::new(move |text, _position| {
                            Ok(Self::#variant_name(String::from(text)))
                        })), #pattern, #is_regex)
                    }
                } else if let Ok(func_path) = syn::parse_str::<syn::Path>(func_name) {
                    quote! {
                        (::sea_lex::TokenCreator::Parser(std::sync::Arc::new(move |text, position| {
                            use ::sea_lex::TokenParser;
                            let parser = #func_path;
                            parser.parse(text, position).map(Self::#variant_name)
                        })), #pattern, #is_regex)
                    }
                } else if let Ok(func_expr) = syn::parse_str::<syn::Expr>(func_name) {
                    quote! {
                        (::sea_lex::TokenCreator::Parser(std::sync::Arc::new(move |text, position| {
                            use ::sea_lex::TokenParser;
                            let parser = #func_expr;
                            parser.parse(text, position).map(Self::#variant_name)
                        })), #pattern, #is_regex)
                    }
                } else {
                    // Fallback: treat as raw tokens
                    let func_tokens: proc_macro2::TokenStream = func_name.parse().unwrap();
                    quote! {
                        (::sea_lex::TokenCreator::Parser(std::sync::Arc::new(move |text, position| {
                            use ::sea_lex::TokenParser;
                            let parser = #func_tokens;
                            parser.parse(text, position).map(Self::#variant_name)
                        })), #pattern, #is_regex)
                    }
                }
            }
        }
    });

    let skip_pattern_strs = skip_patterns
        .iter()
        .map(|(pattern, is_regex)| quote! { (#pattern, #is_regex) });

    let expanded = quote! {
        impl #impl_generics #enum_name #ty_generics #where_clause {
            /// Create a new lexer for this token type
            pub fn lexer(input: impl Into<String>) -> ::sea_lex::Lexer<Self> {
                let matchers = vec![
                    #(#matcher_implementations),*
                ];
                let skip_patterns = vec![
                    #(#skip_pattern_strs),*
                ];
                ::sea_lex::Lexer::new(input, matchers, skip_patterns).unwrap()
            }
            
            /// Create a tokenizing iterator for this token type
            pub fn tokenize(input: impl Into<String>) -> ::sea_lex::Lexer<Self> {
                Self::lexer(input)
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
enum TokenCreatorType {
    Unit(Ident),
    Function(Ident, String),
}

#[derive(Debug)]
struct TokenMatcherInfo {
    pattern: String,
    creator: TokenCreatorType,
    is_regex: bool,
}

fn extract_skip_patterns(attrs: &[Attribute]) -> Vec<(String, bool)> {
    let mut skip_patterns = Vec::new();

    for attr in attrs {
        // Handle #[skip(pattern)] syntax only
        if attr.path().is_ident("skip") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens_str = meta_list.tokens.to_string();
                let pattern_with_quotes = tokens_str.trim();
                
                if let Some((pattern, is_regex)) = parse_pattern_string(pattern_with_quotes) {
                    skip_patterns.push((pattern, is_regex));
                }
            }
        }
    }

    skip_patterns
}

fn parse_pattern_string(pattern_with_quotes: &str) -> Option<(String, bool)> {
    if pattern_with_quotes.starts_with("r\"")
        && pattern_with_quotes.ends_with('"')
    {
        // Raw string literal: r"pattern" - this is a regex
        let pattern = &pattern_with_quotes[2..pattern_with_quotes.len() - 1];
        Some((pattern.to_string(), true))
    } else if pattern_with_quotes.starts_with('"')
        && pattern_with_quotes.ends_with('"')
    {
        // Regular string literal: "pattern" - this is a literal
        if let Ok(lit) = syn::parse_str::<syn::LitStr>(pattern_with_quotes) {
            Some((lit.value(), false))
        } else {
            None
        }
    } else {
        None
    }
}

fn extract_token_matcher(variant: &Variant) -> Option<TokenMatcherInfo> {
    for attr in &variant.attrs {
        if attr.path().is_ident("token") {
            return parse_token_attribute(attr, variant);
        }
    }
    None
}

fn parse_token_attribute(attr: &Attribute, variant: &Variant) -> Option<TokenMatcherInfo> {
    if let Meta::List(meta_list) = &attr.meta {
        // Simple string parsing approach
        let tokens_str = meta_list.tokens.to_string();
        let parts: Vec<&str> = tokens_str.split(',').map(|s| s.trim()).collect();

        match parts.len() {
            1 => {
                // #[token("pattern")] or #[token(r"pattern")]
                let pattern_with_quotes = parts[0].trim();
                if pattern_with_quotes.starts_with("r\"") && pattern_with_quotes.ends_with('"') {
                    // Raw string literal: r"pattern" - this is a regex
                    let pattern = &pattern_with_quotes[2..pattern_with_quotes.len() - 1];
                    let creator = match &variant.fields {
                        Fields::Unit => TokenCreatorType::Unit(variant.ident.clone()),
                        _ => return None,
                    };
                    return Some(TokenMatcherInfo {
                        pattern: pattern.to_string(),
                        creator,
                        is_regex: true,
                    });
                } else if pattern_with_quotes.starts_with('"') && pattern_with_quotes.ends_with('"')
                {
                    // Regular string literal: "pattern" - this is a literal
                    // Need to parse as a string literal to handle escapes properly
                    if let Ok(lit) = syn::parse_str::<syn::LitStr>(pattern_with_quotes) {
                        let creator = match &variant.fields {
                            Fields::Unit => TokenCreatorType::Unit(variant.ident.clone()),
                            _ => return None,
                        };
                        return Some(TokenMatcherInfo {
                            pattern: lit.value(),
                            creator,
                            is_regex: false,
                        });
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ if parts.len() >= 2 => {
                // #[token("pattern", function)] or #[token(r"pattern", function)]
                // Handle both simple functions and closures
                let pattern_with_quotes = parts[0].trim();
                // Join all parts after the first comma to handle closures with commas
                let func_parts: Vec<&str> = parts[1..].iter().map(|s| s.trim()).collect();
                let func_name = func_parts.join(", ");

                if pattern_with_quotes.starts_with("r\"") && pattern_with_quotes.ends_with('"') {
                    // Raw string literal: r"pattern" - this is a regex
                    let pattern = &pattern_with_quotes[2..pattern_with_quotes.len() - 1];
                    let creator = match &variant.fields {
                        Fields::Unnamed(_) => {
                            TokenCreatorType::Function(variant.ident.clone(), func_name)
                        }
                        _ => return None,
                    };
                    return Some(TokenMatcherInfo {
                        pattern: pattern.to_string(),
                        creator,
                        is_regex: true,
                    });
                } else if pattern_with_quotes.starts_with('"') && pattern_with_quotes.ends_with('"')
                {
                    // Regular string literal: "pattern" - this is a literal
                    // Need to parse as a string literal to handle escapes properly
                    if let Ok(lit) = syn::parse_str::<syn::LitStr>(pattern_with_quotes) {
                        let creator = match &variant.fields {
                            Fields::Unnamed(_) => {
                                TokenCreatorType::Function(variant.ident.clone(), func_name)
                            }
                            _ => return None,
                        };
                        return Some(TokenMatcherInfo {
                            pattern: lit.value(),
                            creator,
                            is_regex: false,
                        });
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    None
}
