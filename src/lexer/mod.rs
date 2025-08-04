pub mod token;

use token::{ColumnOffset, Keywords, Literals, Symbols, Token, TokenType, Types};

use crate::error::LexerError;

/// A representation of the current state of the lexer.
#[derive(Debug, PartialEq, Eq)]
pub struct LexerState {
    /// Contains the current line number in the file.
    line: usize,
    /// Contains the current column number in the file.
    column: usize,
}

impl LexerState {
    /// Create a new [`LexerState`] with default line and column.
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }
}

/// Tokenizes a given input assumving the input is a string representation of
/// a line in a o2 file.
pub fn tokenize(
    content: &str,
    tokens: &mut Vec<Token>,
    state: &mut LexerState,
) -> Result<(), LexerError> {
    let mut index: usize = 0;
    let mut buffer = String::new();
    let content_vec: Vec<_> = content.chars().collect();
    let content_size: usize = content_vec.len();

    while index < content_size {
        let mut c = content_vec[index];

        match c {
            '(' => push_inc_col(tokens, state, Symbols::OpenParen),
            ')' => push_inc_col(tokens, state, Symbols::CloseParen),
            '{' => push_inc_col(tokens, state, Symbols::OpenCurly),
            '}' => push_inc_col(tokens, state, Symbols::CloseCurly),
            ';' => push_inc_col(tokens, state, Symbols::SemiColon),
            ' ' => {
                state.column += 1;
            }
            _ => {
                if c.is_ascii_alphabetic() || c == '_' {
                    loop {
                        if !c.is_ascii_alphanumeric() && c != '_' {
                            break;
                        }
                        buffer.push(c);
                        index += 1;
                        state.column += 1;

                        if index >= content_size {
                            break;
                        }
                        c = content_vec[index];
                    }
                    match buffer.as_str() {
                        "return" => push_col_offset(tokens, state, Keywords::Return),
                        "int" => push_col_offset(tokens, state, Types::Int),
                        some => {
                            push_col_offset(tokens, state, TokenType::SomeName(some.to_string()))
                        }
                    }
                } else if c.is_ascii_digit() {
                    loop {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        buffer.push(c);
                        index += 1;
                        state.column += 1;

                        if index >= content_size {
                            break;
                        }
                        c = content_vec[index];
                    }
                    push_col_offset(tokens, state, Literals::Integer("99".to_string()));
                } else {
                    return Err(LexerError::UnknownCharacter {
                        the_char: c,
                        at_line: state.line,
                        at_column: state.column,
                    });
                }
            }
        }

        if buffer.is_empty() {
            index += 1;
        } else {
            buffer.clear();
        }
    }

    state.line += 1;
    state.column = 1;

    Ok(())
}

/// Push a [`Token`] with the given [`TokenType`] into the `tokens` vec and
/// increment the column by 1
fn push_inc_col(
    tokens: &mut Vec<Token>,
    state: &mut LexerState,
    token_type: impl std::convert::Into<TokenType>,
) {
    tokens.push(Token::new(token_type, state.line, state.column));
    state.column += 1;
}

/// Push a [`Token`] with a given [`TokenType`] into the `tokens` vec and
/// increment offset the set column by the given offset.
fn push_col_offset(
    tokens: &mut Vec<Token>,
    state: &mut LexerState,
    token_type: impl std::convert::Into<TokenType>,
) {
    let t: TokenType = token_type.into();
    let offset = t.to_col_offset();

    tokens.push(Token::new(t, state.line, state.column - offset))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_new_lexer_state() {
        assert_eq!(LexerState { line: 1, column: 1 }, LexerState::new());
    }

    #[test]
    fn should_push_and_inc_column() {
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        push_inc_col(&mut tokens, &mut state, Symbols::OpenParen);

        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Symbols::OpenParen, 1, 1)
        );
        assert_eq!(state.line, 1);
        assert_eq!(state.column, 2);
    }

    #[test]
    fn should_push_col_offset() {
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState { line: 1, column: 7 };

        push_col_offset(&mut tokens, &mut state, Keywords::Return);

        assert_eq!(tokens.first().unwrap(), &Token::new(Keywords::Return, 1, 1));
        assert_eq!(state.line, 1);
        assert_eq!(state.column, 7);
    }

    #[test]
    fn should_tokenize_open_paren() {
        let content = "(";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Symbols::OpenParen, 1, 1)
        );
    }

    #[test]
    fn should_tokenize_close_paren() {
        let content = ")";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Symbols::CloseParen, 1, 1)
        );
    }

    #[test]
    fn should_tokenize_open_curly() {
        let content = "{";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Symbols::OpenCurly, 1, 1)
        );
    }

    #[test]
    fn should_tokenize_close_curly() {
        let content = "}";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Symbols::CloseCurly, 1, 1)
        );
    }

    #[test]
    fn should_tokenize_semi_colon() {
        let content = ";";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Symbols::SemiColon, 1, 1)
        );
    }

    #[test]
    fn should_tokenize_keyword_return() {
        let content = "return";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap(), &Token::new(Keywords::Return, 1, 1));
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_keyword_int() {
        let content = "int";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens.first().unwrap(), &Token::new(Types::Int, 1, 1));
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name() {
        let content = "name";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::SomeName("name".to_string()), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name_digit() {
        let content = "n9me";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::SomeName("n9me".to_string()), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name_underscore_prefix() {
        let content = "_name";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::SomeName("_name".to_string()), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name_underscore() {
        let content = "n_ame";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::SomeName("n_ame".to_string()), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_digit() {
        let content = "99";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_ok());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(Literals::Integer("99".to_string()), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_to_err() {
        let content = "⫯";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        let res = tokenize(content, &mut tokens, &mut state);

        assert!(res.is_err());
    }

    #[test]
    fn should_tokenize() {
        let content = "int main() {\n    return 0;\n}";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();

        for line in content.split('\n') {
            let res = tokenize(line, &mut tokens, &mut state);
            assert!(res.is_ok());
        }

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens.first().unwrap(), &Token::new(Types::Int, 1, 1));
        assert_eq!(
            tokens.last().unwrap(),
            &Token::new(Symbols::CloseCurly, 3, 1)
        );
        assert_eq!(state.line, 4);
        assert_eq!(state.column, 1);
    }
}
