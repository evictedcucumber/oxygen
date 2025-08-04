use crate::{
    error::{StatementError, StatementResult, TokenTypeError},
    lexer::token::{Keywords, Symbols, TokenType, Types},
    parser::{Parser, node::Statement, parsers::term::parse_term},
};

pub fn parse_statement(parser: &mut Parser) -> StatementResult {
    if parser
        .peek(0)
        .is_some_and(|t| matches!(t.token_type, TokenType::Type(_)))
        && parser
            .peek(1)
            .is_some_and(|t| matches!(t.token_type, TokenType::SomeName(_)))
        && parser
            .peek(2)
            .is_some_and(|t| t.token_type == Symbols::OpenParen.into())
    {
        return parse_statement_function_declare(parser);
    } else if parser
        .peek(0)
        .is_some_and(|t| t.token_type == Keywords::Return.into())
    {
        return parse_statement_return(parser);
    }

    // TODO: add Err return if no statement can be parsed.
    unreachable!("Shouldn't get to statement parser unreachable");
}

/// Parse some tokens into a function declaration.
fn parse_statement_function_declare(parser: &mut Parser) -> StatementResult {
    // int main() {...}
    // ^^^
    let return_type: Types = match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: Keywords::Return.into(),
        })?
        .token_type
    {
        TokenType::Type(t) => Ok(t),
        t => Err(TokenTypeError::Expected {
            expected: Keywords::Return.into(),
            got: t,
        }),
    }?;
    // int main() {...}
    //     ^^^^
    let name: String = match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: TokenType::SomeName("any".to_string()),
        })?
        .token_type
    {
        TokenType::SomeName(name) => Ok(name),
        t => Err(TokenTypeError::Expected {
            expected: TokenType::SomeName("any".to_string()),
            got: t,
        }),
    }?;
    // int main() {...}
    //         ^
    match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: Symbols::OpenParen.into(),
        })?
        .token_type
    {
        TokenType::Symbol(Symbols::OpenParen) => Ok(()),
        t => Err(TokenTypeError::Expected {
            expected: Symbols::OpenParen.into(),
            got: t,
        }),
    }?;
    // int main() {...}
    //          ^
    match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: Symbols::CloseParen.into(),
        })?
        .token_type
    {
        TokenType::Symbol(Symbols::CloseParen) => Ok(()),
        t => Err(TokenTypeError::Expected {
            expected: Symbols::CloseParen.into(),
            got: t,
        }),
    }?;
    // int main() {...}
    //            ^
    match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: Symbols::OpenCurly.into(),
        })?
        .token_type
    {
        TokenType::Symbol(Symbols::OpenCurly) => Ok(()),
        t => Err(TokenTypeError::Expected {
            expected: Symbols::OpenCurly.into(),
            got: t,
        }),
    }?;
    // int main() {...}
    //             ^^^
    let mut body: Vec<Statement> = Vec::new();
    while parser
        .peek(0)
        .ok_or(TokenTypeError::ExpectedSomeGotNone)?
        .token_type
        != Symbols::CloseCurly.into()
    {
        body.push(parse_statement(parser)?);
    }
    if !body
        .last()
        .is_some_and(|t| matches!(t, Statement::Return { .. }))
    {
        return Err(StatementError::MissingReturn);
    }

    // No need to check as the while loop above handles until the '}'
    // int main() {...}
    //                ^
    parser.consume();

    Ok(Statement::FunctionDeclare {
        name,
        return_type,
        body,
    })
}

fn parse_statement_return(parser: &mut Parser) -> StatementResult {
    // return ...;
    // ^^^^^^
    match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: Keywords::Return.into(),
        })?
        .token_type
    {
        TokenType::Keyword(Keywords::Return) => Ok(()),
        t => Err(TokenTypeError::Expected {
            expected: Keywords::Return.into(),
            got: t,
        }),
    }?;
    // return ...;
    //        ^^^
    let term = parse_term(parser)?;
    // return ...;
    //           ^
    match parser
        .consume()
        .ok_or(TokenTypeError::ExpectedGotNone {
            expected: Symbols::SemiColon.into(),
        })?
        .token_type
    {
        TokenType::Symbol(Symbols::SemiColon) => Ok(()),
        t => Err(TokenTypeError::Expected {
            expected: Symbols::SemiColon.into(),
            got: t,
        }),
    }?;

    Ok(Statement::Return { term })
}

#[cfg(test)]
mod tests {
    use crate::lexer::token::{Literals, Token, Types};

    use super::*;

    mod test_parse_statement {
        use crate::parser::node::Term;

        use super::*;

        #[test]
        fn should_parse_function_declare() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
                Token::new(Symbols::CloseParen, 1, 10),
                Token::new(Symbols::OpenCurly, 1, 12),
                Token::new(Keywords::Return, 2, 5),
                Token::new(Literals::Integer("0".to_string()), 2, 7),
                Token::new(Symbols::SemiColon, 2, 8),
                Token::new(Symbols::CloseCurly, 3, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement(&mut parser);

            assert!(res.is_ok());
            assert_eq!(
                res.ok().unwrap(),
                Statement::FunctionDeclare {
                    name: "main".to_string(),
                    return_type: Types::Int,
                    body: vec![Statement::Return {
                        term: Term::LiteralInteger("0".to_string())
                    }],
                }
            );
        }

        #[test]
        fn should_parse_return() {
            let tokens: Vec<Token> = vec![
                Token::new(Keywords::Return, 2, 5),
                Token::new(Literals::Integer("0".to_string()), 2, 7),
                Token::new(Symbols::SemiColon, 2, 8),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement(&mut parser);

            assert!(res.is_ok());
            assert_eq!(
                res.ok().unwrap(),
                Statement::Return {
                    term: Term::LiteralInteger("0".to_string())
                }
            );
        }
    }

    mod test_parse_statement_function_declare {
        use crate::parser::node::Term;

        use super::*;

        #[test]
        fn should_parse() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
                Token::new(Symbols::CloseParen, 1, 10),
                Token::new(Symbols::OpenCurly, 1, 12),
                Token::new(Keywords::Return, 2, 5),
                Token::new(Literals::Integer("0".to_string()), 2, 7),
                Token::new(Symbols::SemiColon, 2, 8),
                Token::new(Symbols::CloseCurly, 3, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_ok());
            assert_eq!(
                res.ok().unwrap(),
                Statement::FunctionDeclare {
                    name: "main".to_string(),
                    return_type: Types::Int,
                    body: vec![Statement::Return {
                        term: Term::LiteralInteger("0".to_string())
                    }],
                }
            );
        }

        #[test]
        fn should_error_try_type_but_none() {
            let tokens: Vec<Token> = Vec::new();
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::ExpectedGotNone { .. })
            )))
        }
        #[test]
        fn should_error_try_type_expected() {
            let tokens: Vec<Token> = vec![Token::new(Keywords::Return, 2, 5)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::Expected { .. })
            )))
        }

        #[test]
        fn should_error_try_name_but_none() {
            let tokens: Vec<Token> = vec![Token::new(Types::Int, 1, 1)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::ExpectedGotNone { .. })
            )))
        }
        #[test]
        fn should_error_try_name_expected() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(Keywords::Return, 1, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::Expected { .. })
            )))
        }

        #[test]
        fn should_error_try_open_paren_but_none() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::ExpectedGotNone { .. })
            )))
        }
        #[test]
        fn should_error_try_open_paren_expected() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Keywords::Return, 1, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::Expected { .. })
            )))
        }

        #[test]
        fn should_error_try_close_paren_but_none() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::ExpectedGotNone { .. })
            )))
        }
        #[test]
        fn should_error_try_close_paren_expected() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
                Token::new(Keywords::Return, 1, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::Expected { .. })
            )))
        }

        #[test]
        fn should_error_try_open_curly_but_none() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
                Token::new(Symbols::CloseParen, 1, 10),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::ExpectedGotNone { .. })
            )))
        }
        #[test]
        fn should_error_try_open_curly_expected() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
                Token::new(Symbols::CloseParen, 1, 10),
                Token::new(Keywords::Return, 1, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(
                e,
                StatementError::TokenType(TokenTypeError::Expected { .. })
            )))
        }

        #[test]
        fn should_error_missing_final_return() {
            let tokens: Vec<Token> = vec![
                Token::new(Types::Int, 1, 1),
                Token::new(TokenType::SomeName("main".to_string()), 1, 5),
                Token::new(Symbols::OpenParen, 1, 9),
                Token::new(Symbols::CloseParen, 1, 10),
                Token::new(Symbols::OpenCurly, 1, 12),
                Token::new(Symbols::CloseCurly, 3, 1),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_function_declare(&mut parser);

            assert!(res.is_err_and(|e| matches!(e, StatementError::MissingReturn)));
        }
    }

    mod test_parse_statement_return {

        use crate::parser::node::Term;

        use super::*;

        #[test]
        fn should_parse() {
            let tokens: Vec<Token> = vec![
                Token::new(Keywords::Return, 1, 5),
                Token::new(Literals::Integer("0".to_string()), 1, 7),
                Token::new(Symbols::SemiColon, 1, 8),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_ok());
            assert_eq!(
                res.ok().unwrap(),
                Statement::Return {
                    term: Term::LiteralInteger("0".to_string())
                }
            );
        }

        #[test]
        fn should_error_try_return_but_none() {
            let tokens: Vec<Token> = Vec::new();
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_err());
        }

        #[test]
        fn should_error_try_return_expected() {
            let tokens: Vec<Token> = vec![Token::new(Symbols::SemiColon, 0, 0)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_err());
        }

        #[test]
        fn should_error_try_term_but_none() {
            let tokens: Vec<Token> = vec![Token::new(Keywords::Return, 1, 5)];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_err());
        }

        #[test]
        fn should_error_try_term_expected() {
            let tokens: Vec<Token> = vec![
                Token::new(Keywords::Return, 1, 5),
                Token::new(Symbols::OpenParen, 0, 0),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_err());
        }

        #[test]
        fn should_error_try_semi_colon_but_none() {
            let tokens: Vec<Token> = vec![
                Token::new(Keywords::Return, 1, 5),
                Token::new(Literals::Integer("0".to_string()), 1, 7),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_err());
        }

        #[test]
        fn should_error_try_semi_colon_expected() {
            let tokens: Vec<Token> = vec![
                Token::new(Keywords::Return, 1, 5),
                Token::new(Literals::Integer("0".to_string()), 1, 7),
                Token::new(Symbols::OpenParen, 1, 8),
            ];
            let mut prog: Vec<Statement> = Vec::new();
            let mut parser = Parser::new(tokens, &mut prog);

            let res = parse_statement_return(&mut parser);

            assert!(res.is_err());
        }
    }
}
