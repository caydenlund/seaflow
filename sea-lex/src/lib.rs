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

mod lexer;
pub use lexer::*;

mod token_creator;
pub use token_creator::*;

mod token_matcher;
pub use token_matcher::*;

mod token_type;
pub use token_type::*;
