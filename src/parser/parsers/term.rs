use crate::{
    error::{TermError, TermResult, TokenTypeError},
    lexer::token::{Literals, TokenType},
    parser::{Parser, node::Term},
};

pub fn parse_term(parser: &mut Parser) -> TermResult {
    if parser
        .peek(0)
        .is_some_and(|t| matches!(t.token_type, TokenType::Literal(Literals::Integer(_))))
    {
        return parse_term_literal_int(parser);
    }

    Err(TermError::NoTerm)
}

fn parse_term_literal_int(parser: &mut Parser) -> TermResult {
    let value: String = match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedSomeGotNone)?
        .token_type
    {
        TokenType::Literal(Literals::Integer(int)) => Ok(int),
        t => Err(TokenTypeError::Expected {
            expected: Literals::Integer("any".to_string()).into(),
            got: t,
        }),
    }?;

    Ok(Term::LiteralInteger(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::token::Token, parser::node::Statement};

    mod test_parse_term {
        use super::*;

        #[test]
        fn should_parse() {
            let tokens: Vec<Token> = vec![Token::new(Literals::Integer("0".to_string()), 1, 7)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_term(&mut parser);

            assert!(res.is_ok());
        }

        #[test]
        fn should_error() {
            let tokens: Vec<Token> = Vec::new();
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_term(&mut parser);

            assert!(res.is_err());
            assert_eq!(res.err().unwrap(), TermError::NoTerm);
        }
    }

    mod test_parse_term_literal_int {
        use crate::lexer::token::Symbols;

        use super::*;

        #[test]
        fn should_parse() {
            let tokens: Vec<Token> = vec![Token::new(Literals::Integer("0".to_string()), 1, 7)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_term_literal_int(&mut parser);

            assert!(res.is_ok());
            assert_eq!(res.unwrap(), Term::LiteralInteger("0".to_string()));
        }

        #[test]
        fn should_error_none() {
            let tokens: Vec<Token> = Vec::new();
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_term_literal_int(&mut parser);

            assert!(res.is_err());
            assert_eq!(
                res.err().unwrap(),
                TokenTypeError::ExpectedSomeGotNone.into()
            )
        }

        #[test]
        fn should_error_expected() {
            let tokens: Vec<Token> = vec![Token::new(Symbols::OpenParen, 0, 0)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_term_literal_int(&mut parser);

            assert!(res.is_err());
            assert_eq!(
                res.err().unwrap(),
                TokenTypeError::Expected {
                    expected: Literals::Integer("any".to_string()).into(),
                    got: Symbols::OpenParen.into(),
                }
                .into()
            )
        }
    }
}
