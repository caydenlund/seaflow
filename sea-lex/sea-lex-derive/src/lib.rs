use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Meta, Variant,
};

#[proc_macro_derive(Token, attributes(token))]
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
                let func_path: syn::Path = syn::parse_str(func_name).unwrap();
                quote! {
                    (::sea_lex::TokenCreator::Function(|s| Self::#variant_name(#func_path(s))), #pattern, #is_regex)
                }
            }
        }
    });

    let skip_pattern_strs = skip_patterns.iter().map(|(pattern, is_regex)| quote! { (#pattern, #is_regex) });

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
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
enum TokenCreatorType {
    Unit(Ident),
    Function(Ident, String), // Changed to String to handle paths like String::from
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
        if attr.path().is_ident("token") {
            if let Meta::List(meta_list) = &attr.meta {
                // Simple approach: parse as string and extract skip patterns manually
                let tokens_str = meta_list.tokens.to_string();
                if tokens_str.contains("skip") {
                    // Extract the string after "skip ="
                    if let Some(start) = tokens_str.find("skip = ") {
                        let remainder = &tokens_str[start + 7..];
                        if let Some(end) = remainder.find(',').or_else(|| Some(remainder.len())) {
                            let pattern_with_quotes = remainder[..end].trim();
                            if pattern_with_quotes.starts_with("r\"") && pattern_with_quotes.ends_with('"') {
                                // Raw string literal: r"pattern" - this is a regex
                                let pattern = &pattern_with_quotes[2..pattern_with_quotes.len()-1];
                                skip_patterns.push((pattern.to_string(), true));
                            } else if pattern_with_quotes.starts_with('"') && pattern_with_quotes.ends_with('"') {
                                // Regular string literal: "pattern" - this is a literal
                                // Need to parse as a string literal to handle escapes properly
                                if let Ok(lit) = syn::parse_str::<syn::LitStr>(pattern_with_quotes) {
                                    skip_patterns.push((lit.value(), false));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    

    skip_patterns
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
                    let pattern = &pattern_with_quotes[2..pattern_with_quotes.len()-1];
                    let creator = match &variant.fields {
                        Fields::Unit => TokenCreatorType::Unit(variant.ident.clone()),
                        _ => return None,
                    };
                    return Some(TokenMatcherInfo { 
                        pattern: pattern.to_string(), 
                        creator,
                        is_regex: true,
                    });
                } else if pattern_with_quotes.starts_with('"') && pattern_with_quotes.ends_with('"') {
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
            2 => {
                // #[token("pattern", function)] or #[token(r"pattern", function)]
                let pattern_with_quotes = parts[0].trim();
                let func_name_str = parts[1].trim();
                let func_name = func_name_str.to_string();
                
                if pattern_with_quotes.starts_with("r\"") && pattern_with_quotes.ends_with('"') {
                    // Raw string literal: r"pattern" - this is a regex
                    let pattern = &pattern_with_quotes[2..pattern_with_quotes.len()-1];
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
                } else if pattern_with_quotes.starts_with('"') && pattern_with_quotes.ends_with('"') {
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