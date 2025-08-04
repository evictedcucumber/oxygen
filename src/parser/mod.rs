pub mod node;
pub mod parsers;

use node::Statement;
use parsers::statement::parse_statement;

use crate::{error::ParserResult, lexer::token::Token};

#[derive(Debug, PartialEq, Eq)]
pub struct Parser<'a> {
    tokens: Box<[Token]>,
    prog: &'a mut Vec<Statement>,
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, prog: &'a mut Vec<Statement>) -> Self {
        Self {
            tokens: tokens.into_boxed_slice(),
            prog,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> ParserResult {
        while self.index < self.tokens.len() {
            let statement = parse_statement(self)?;
            self.prog.push(statement);
        }

        Ok(())
    }

    fn peek(&self, offset: usize) -> Option<Token> {
        self.tokens.get(self.index + offset).cloned()
    }

    fn consume(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.index).cloned();
        self.index += 1;
        t
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::Keywords;

    use super::*;

    #[test]
    fn should_peek_some() {
        let tokens: Vec<Token> = vec![Token::new(Keywords::Return, 1, 1)];
        let mut prog: Vec<Statement> = Vec::new();
        let parser = Parser::new(tokens, &mut prog);

        assert!(parser.peek(0).is_some());
        assert_eq!(parser.peek(0).unwrap(), Token::new(Keywords::Return, 1, 1));
    }

    #[test]
    fn should_peek_none() {
        let tokens: Vec<Token> = Vec::new();
        let mut prog: Vec<Statement> = Vec::new();
        let parser = Parser::new(tokens, &mut prog);

        assert!(parser.peek(0).is_none());
    }

    #[test]
    fn should_consume_some() {
        let tokens: Vec<Token> = vec![Token::new(Keywords::Return, 1, 1)];
        let mut prog: Vec<Statement> = Vec::new();
        let mut parser = Parser::new(tokens, &mut prog);

        let c = parser.consume();
        assert!(c.is_some());
        assert_eq!(c.unwrap(), Token::new(Keywords::Return, 1, 1));
        assert_eq!(parser.index, 1);
    }

    #[test]
    fn should_consume_none() {
        let tokens: Vec<Token> = Vec::new();
        let mut prog: Vec<Statement> = Vec::new();
        let mut parser = Parser::new(tokens, &mut prog);

        let c = parser.consume();
        assert!(c.is_none());
    }
}
