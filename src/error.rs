use thiserror::Error;

/// The general error used for the top level program.
///
/// All errors will be able to become an [`thiserror::Error`] using [`std::convert::From`].
#[derive(Error, Debug)]
pub enum Error {
    /// Contains the [`LexerError`]
    #[error("{0}")]
    Lexer(#[from] LexerError),
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
