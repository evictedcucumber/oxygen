mod error;
mod lexer;
mod parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use clap::Parser;
use error::Error;
use lexer::{LexerState, token::Token, tokenize};
use parser::node::Statement;

pub fn wrapper() -> Result<(), Error> {
    let cli = cmdline::O2CCli::parse();

    let oxygen_file = File::open(cli.oxygen_file)?;

    let mut lexer_state = LexerState::new();
    let mut tokens: Vec<Token> = Vec::new();

    let file_reader = BufReader::new(oxygen_file);
    for line in file_reader.lines() {
        tokenize(&line?, &mut tokens, &mut lexer_state)?;
    }

    if cli.display_tokens {
        for token in &tokens {
            println!("{token:#?}");
        }
        return Ok(());
    }

    let mut prog: Vec<Statement> = Vec::new();
    let mut parser = parser::Parser::new(tokens, &mut prog);
    parser.parse()?;

    if cli.display_ast {
        println!("{prog:#?}");
        return Ok(());
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = wrapper() {
        eprintln!("\x1b[31;1merror:\x1b[0m {e}");
        exit(1);
    }
}
