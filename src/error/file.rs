use crate::error::Error;

#[derive(Debug)]
pub enum FileError {
    UnableToOpen(String),
    UnableToRead(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileError::*;

        let msg = match self {
            UnableToOpen(file_name) => format!("unable to open file '{file_name}'"),
            UnableToRead(file_name) => format!("unable to read file '{file_name}'"),
        };

        write!(f, "{msg}")
    }
}

impl std::convert::From<FileError> for Error {
    fn from(e: FileError) -> Self {
        Error::File(e)
    }
}

