pub mod cmdline;
pub mod file;
pub mod lexer;

use std::fmt::Display;

use cmdline::CmdlineError;
use file::FileError;
use lexer::LexerError;

pub type ResultT<T> = Result<T, crate::error::Error>;

#[derive(Debug)]
pub enum Error {
    Cmdline(CmdlineError),
    Lexer(LexerError),
    File(FileError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::Cmdline(cmdline_error) => cmdline_error.to_string(),
            Error::Lexer(lexer_error) => lexer_error.to_string(),
            Error::File(file_error) => file_error.to_string(),
        };

        write!(f, "{msg}")
    }
}

fn error_builder(msg: &str, line_num: usize, the_line: &str, column_num: usize) -> String {
    let mut output = String::new();
    output.push_str(&format!("\x1b[31;1merror:\x1b[0m \x1b[1m{msg}\x1b[0m"));
    output.push_str(&format!("\n  \x1b[34;1m{line_num} |\x1b[0m    {the_line}"));
    output.push_str(&format!(
        "\n  \x1b[34;1m{} |\x1b[0m    {}\x1b[31;1m^\x1b[0m",
        " ".repeat(line_num.to_string().len()),
        " ".repeat(column_num - 1),
    ));

    output
}
