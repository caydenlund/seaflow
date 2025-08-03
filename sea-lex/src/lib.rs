//! sea-lex - Seaflow lexer component

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    missing_docs,
    rustdoc::all
)]

mod error;
mod lexer;
mod token;
mod token_parser;

pub use error::*;
pub use lexer::*;
pub use token::*;
pub use token_parser::*;

#[cfg(feature = "derive")]
pub use sea_lex_derive::Token;
