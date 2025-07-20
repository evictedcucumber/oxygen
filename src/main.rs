mod cmdline;
mod error;
mod lexer;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use cmdline::{help_text, parse_args};
use error::{Error, file::FileError, lexer::LexerError};
use lexer::{token::Token, tokenize, LexerState};

pub fn main() {
    let argv = std::env::args().skip(1).collect::<Vec<String>>();
    let config = match parse_args(&argv) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    };

    if config.display_help {
        println!("{}", help_text());
        exit(0);
    }

    let oxygen_file = match File::open("examples/basic.o2") {
        Ok(f) => f,
        Err(_) => {
            let e = FileError::UnableToOpen("basic.o2".to_string());
            eprintln!("{e}");
            exit(1);
        }
    };

    let mut lexer_state = LexerState::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer_errors: Vec<LexerError> = Vec::new();

    let file_reader = BufReader::new(oxygen_file);
    for line in file_reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                let e = FileError::UnableToRead("basic.o2".to_string());
                eprintln!("{e}");
                exit(1);
            }
        };
        tokenize(&line, &mut tokens, &mut lexer_state, &mut lexer_errors);
    }

    if !lexer_errors.is_empty() {
        let errors: Vec<Error> = lexer_errors.iter().map(Into::into).collect();
        for error in errors {
            println!("{error}")
        }
        exit(1);
    }

    if config.display_tokens {
        for token in &tokens {
            println!("{token}");
        }
    }
}
