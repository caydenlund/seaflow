//! sea-lex/error - Lexer module for errors that can occur during lexing.
//!
//! This module provides a [`LexerError`] struct that can be used to inform
//! the user that there was an illegal character found in the input source.

use std::fmt::Display;
use thiserror::Error;

/// An error emitted when an input character can't be matched by the lexer.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub struct LexerError {
    /// The position of the illegal character in the input file.
    pub position: usize,
    /// The illegal character that couldn't be lexed.
    pub unmatched: char,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unexpected token at position {}: '{}'",
            self.position, self.unmatched
        )
    }
}
