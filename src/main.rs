mod cmdline;
mod error;
mod lexer;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;
use error::Error;
use lexer::{LexerState, token::Token, tokenize};

pub fn wrapper() -> Result<(), Error> {
    let cli = cmdline::Cli::parse();

    let oxygen_file = File::open(cli.oxygen_file)?;
    dbg!(&oxygen_file);

    let mut lexer_state = LexerState::new();
    let mut tokens: Vec<Token> = Vec::new();

    let file_reader = BufReader::new(oxygen_file);
    for line in file_reader.lines() {
        tokenize(&line?, &mut tokens, &mut lexer_state)?;
    }

    if cli.display_tokens {
        for token in &tokens {
            println!("{token}");
        }
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = wrapper() {
        eprintln!("\x1b[31;1merror:\x1b[0m {e}");
        std::process::exit(1);
    }
}
