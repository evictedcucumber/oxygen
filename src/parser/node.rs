use crate::lexer::token::Types;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    FunctionDeclare {
        name: String,
        return_type: Types,
        body: Vec<Statement>,
    },
    Return {
        term: Term,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    LiteralInteger(String),
}
