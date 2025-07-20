pub mod token;

use token::{Token, TokenType};

use crate::error::lexer::LexerError;

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
    ///
    /// # Examples
    ///
    /// ```
    /// let s = LexerState::new();
    /// assert_eq!(LexerState { line: 1, column: 1}, s);
    /// ```
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }
}

/// Tokenizes a given input assumving the input is a string representation of
/// a line in a o2 file.
///
/// # Examples
///
/// ```
/// let content = "func();";
/// let mut tokens: Vec<Token> = Vec::new();
/// let mut state = LexerState::new();
/// let mut errors: Vec<LexerError> = Vec::new();
/// tokenize(content, &mut tokens, &mut state, &mut errors);
///
/// assert_eq!(tokens.len(), 4);
/// assert_eq!(
///     tokens.first().unwrap(),
///     &Token::new(TokenType::new_some_name("func"), 1, 1)
/// );
/// assert_eq!(
///     tokens.last().unwrap(),
///     &Token::new(TokenType::SymbolSemiColon, 1, 1)
/// );
/// assert!(errors.is_empty());
/// assert_eq!(state.line, 2);
/// assert_eq!(state.column, 1);
/// ```
pub fn tokenize(
    content: &str,
    tokens: &mut Vec<Token>,
    state: &mut LexerState,
    errors: &mut Vec<LexerError>,
) {
    let mut index: usize = 0;
    let mut buffer = String::new();
    let content_vec: Vec<_> = content.chars().collect();
    let content_size: usize = content_vec.len();

    while index < content_size {
        let mut c = content_vec[index];

        match c {
            '(' => push_inc_col(tokens, state, TokenType::SymbolOpenParen),
            ')' => push_inc_col(tokens, state, TokenType::SymbolCloseParen),
            '{' => push_inc_col(tokens, state, TokenType::SymbolOpenCurly),
            '}' => push_inc_col(tokens, state, TokenType::SymbolCloseCurly),
            ';' => push_inc_col(tokens, state, TokenType::SymbolSemiColon),
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
                        "return" => {
                            push_col_offset(tokens, state, TokenType::KeywordReturn, "return".len())
                        }
                        "int" => push_col_offset(tokens, state, TokenType::KeywordInt, "int".len()),
                        some => push_col_offset(
                            tokens,
                            state,
                            TokenType::new_some_name(some),
                            some.len(),
                        ),
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
                    push_col_offset(tokens, state, TokenType::new_lit_int(&buffer), buffer.len());
                } else {
                    errors.push(LexerError::UnknownCharacter {
                        the_char: c,
                        the_line: content.to_string(),
                        line: state.line,
                        column: state.column,
                    });
                    state.column += 1;
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
}

/// Push a [`Token`] with the given [`TokenType`] into the `tokens` vec and
/// increment the column by 1
///
/// # Examples
///
/// ```
/// let mut tokens: Vec<Token> = Vec::new();
/// let mut state = LexerState::new();
///
/// push_inc_col(&mut tokens, &mut state, TokenType::SymbolOpenParen);
///
/// assert_eq!(
///     tokens.first().unwrap(),
///     &Token::new(TokenType::SymbolOpenParen, 1, 1)
/// );
/// ```
fn push_inc_col(tokens: &mut Vec<Token>, state: &mut LexerState, token_type: TokenType) {
    tokens.push(Token::new(token_type, state.line, state.column));
    state.column += 1;
}

/// Push a [`Token`] with a given [`TokenType`] into the `tokens` vec and
/// increment offset the set column by the given offset.
///
/// # Examples
///
/// ```
/// let mut tokens: Vec<Token> = Vec::new();
/// let mut state = LexerState::new();
///
/// push_col_offset(&mut tokens, &mut state, TokenType::KeywordInt, "int".len());
///
/// assert_eq!(
///     tokens.first().unwrap(), 
///     &Token::new(TokenType::KeywordInt, 1, 1)
/// );
/// ```
fn push_col_offset(
    tokens: &mut Vec<Token>,
    state: &mut LexerState,
    token_type: TokenType,
    offset: usize,
) {
    tokens.push(Token::new(token_type, state.line, state.column - offset))
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

        push_inc_col(&mut tokens, &mut state, TokenType::SymbolOpenParen);

        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::SymbolOpenParen, 1, 1)
        );
        assert_eq!(state.line, 1);
        assert_eq!(state.column, 2);
    }

    #[test]
    fn should_push_col_offset() {
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState { line: 1, column: 7 };

        push_col_offset(
            &mut tokens,
            &mut state,
            TokenType::KeywordReturn,
            "return".len(),
        );

        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::KeywordReturn, 1, 1)
        );
        assert_eq!(state.line, 1);
        assert_eq!(state.column, 7);
    }

    #[test]
    fn should_tokenize_open_paren() {
        let content = "(";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
    }

    #[test]
    fn should_tokenize_close_paren() {
        let content = ")";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
    }

    #[test]
    fn should_tokenize_open_curly() {
        let content = "{";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
    }

    #[test]
    fn should_tokenize_close_curly() {
        let content = "}";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
    }

    #[test]
    fn should_tokenize_semi_colon() {
        let content = ";";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
    }

    #[test]
    fn should_tokenize_space() {
        let content = " ";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 0);
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_fill_errors_vec() {
        let content = "⫯";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(!errors.is_empty());
        assert_eq!(tokens.len(), 0);
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_keyword_return() {
        let content = "return";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::KeywordReturn, 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_keyword_int() {
        let content = "int";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::KeywordInt, 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name() {
        let content = "name";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::new_some_name("name"), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name_digit() {
        let content = "n9me";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::new_some_name("n9me"), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name_underscore_prefix() {
        let content = "_name";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::new_some_name("_name"), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_some_name_underscore() {
        let content = "n_ame";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::new_some_name("n_ame"), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize_digit() {
        let content = "99";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        tokenize(content, &mut tokens, &mut state, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::new_lit_int("99"), 1, 1)
        );
        assert_eq!(state.line, 2);
        assert_eq!(state.column, 1);
    }

    #[test]
    fn should_tokenize() {
        let content = "int main() {\n    return 0;\n}";
        let mut tokens: Vec<Token> = Vec::new();
        let mut state = LexerState::new();
        let mut errors: Vec<LexerError> = Vec::new();

        for line in content.split('\n') {
            tokenize(line, &mut tokens, &mut state, &mut errors);
        }

        assert!(errors.is_empty());
        assert_eq!(tokens.len(), 9);
        assert_eq!(
            tokens.first().unwrap(),
            &Token::new(TokenType::KeywordInt, 1, 1)
        );
        assert_eq!(
            tokens.last().unwrap(),
            &Token::new(TokenType::SymbolCloseCurly, 3, 1)
        );
        assert_eq!(state.line, 4);
        assert_eq!(state.column, 1);
    }
}
