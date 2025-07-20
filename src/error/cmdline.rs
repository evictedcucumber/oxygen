use crate::error::Error;

#[derive(Debug)]
pub enum CmdlineError {
    UnknownArgument(String),
}

impl std::fmt::Display for CmdlineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CmdlineError::*;

        match self {
            UnknownArgument(arg) => write!(f, "unknown argument '{arg}'"),
        }
    }
}

impl std::convert::From<CmdlineError> for Error {
    fn from(e: CmdlineError) -> Self {
        Error::Cmdline(e)
    }
}

