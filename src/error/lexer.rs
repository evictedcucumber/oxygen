use crate::error::{Error, error_builder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    UnknownCharacter {
        the_char: char,
        the_line: String,
        line: usize,
        column: usize,
    },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LexerError::*;

        let msg = match self {
            UnknownCharacter {
                the_char,
                the_line,
                line,
                column,
            } => error_builder(
                &format!("unknown character '{the_char}'"),
                *line,
                the_line,
                *column,
            ),
        };

        write!(f, "{msg}")
    }
}

impl std::convert::From<LexerError> for Error {
    fn from(e: LexerError) -> Self {
        Error::Lexer(e)
    }
}


impl std::convert::From<&LexerError> for Error {
    fn from(e: &LexerError) -> Self {
        Error::Lexer(e.clone())
    }
}
