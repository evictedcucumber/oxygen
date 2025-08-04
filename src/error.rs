use thiserror::Error;

use crate::{
    lexer::token::TokenType,
    parser::node::{Statement, Term},
};

/// The general error used for the top level program.
///
/// All errors will be able to become an [`thiserror::Error`] using [`std::convert::From`].
#[derive(Error, Debug)]
pub enum Error {
    /// Contains the [`LexerError`]
    #[error("{0}")]
    Lexer(#[from] LexerError),
    /// Contains the [`ParserError`]
    #[error("{0}")]
    Parser(#[from] ParserError),
    /// Contains the [`std::io::Error`]
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

/// The [`crate::lexer`] errors.
#[derive(Error, Debug)]
pub enum LexerError {
    /// Error representing an unknown character.
    #[error("unknown character '{the_char}'")]
    UnknownCharacter {
        /// Contains the unknown character.
        the_char: char,
        /// Contains the line number of the unknown character.
        at_line: usize,
        /// Contains the column number of the unknown character.
        at_column: usize,
    },
}

#[derive(Error, Debug)]
pub enum ParserError {
    /// Contains the [`StatementError`].
    #[error("{0}")]
    Statement(#[from] StatementError),
    /// Contains the [`TermError`].
    #[error("{0}")]
    Term(#[from] TermError),
}

pub type ParserResult = Result<(), ParserError>;

#[derive(Error, Debug)]
pub enum StatementError {
    #[error("{0}")]
    Term(#[from] TermError),
    #[error("{0}")]
    TokenType(#[from] TokenTypeError),
    #[error("function declaration missing final return statement")]
    MissingReturn,
}

pub type StatementResult = Result<Statement, StatementError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TermError {
    #[error("{0}")]
    TokenType(#[from] TokenTypeError),
    #[error("no term found to parse")]
    NoTerm,
}

pub type TermResult = Result<Term, TermError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TokenTypeError {
    #[error("expected {expected:?} but got {got:?}")]
    Expected { expected: TokenType, got: TokenType },
    #[error("expected {expected:?} but got none")]
    ExpectedGotNone { expected: TokenType },
    #[error("expected some token but got none")]
    ExpectedSomeGotNone,
}
