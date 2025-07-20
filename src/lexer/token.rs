/// A representation of a type of token.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    /// Contains the value of the integer literal.
    LitInt(Box<str>),
    /// Represents an `(`.
    SymbolOpenParen,
    /// Represents an `)`.
    SymbolCloseParen,
    /// Represents an `{`.
    SymbolOpenCurly,
    /// Represents an `}`.
    SymbolCloseCurly,
    /// Represents an `;`.
    SymbolSemiColon,
    /// Represents the keyword 'return'.
    KeywordReturn,
    /// Represents the keyword 'int'.
    KeywordInt,
    /// Contains the name of some item.
    SomeName(Box<str>),
}

impl TokenType {
    /// Create new [`TokenType::LitInt`] from a given [`&str`].
    ///
    /// # Examples
    ///
    /// ```
    /// let i = TokenType::new_lit_int("0");
    /// assert_eq!(i, TokenType::LitInt(Box::from("0")));
    /// ```
    pub fn new_lit_int(s: &str) -> Self {
        TokenType::LitInt(Box::from(s))
    }

    /// Create new [`TokenType::SomeName`] from a given [`&str`].
    ///
    /// # Examples
    ///
    /// ```
    /// let s = TokenType::new_some_name("main");
    /// assert_eq!(s, TokenType::SomeName(Box::from("main")));
    /// ```
    pub fn new_some_name(s: &str) -> Self {
        TokenType::SomeName(Box::from(s))
    }

    /// Get the column offset of a [`TokenType`].
    ///
    /// # Examples
    ///
    /// ```
    /// let offset: usize = TokenType::KeywordReturn.to_column_offset();
    /// assert_eq(offset, 6);
    /// ```
    pub fn to_column_offset(&self) -> usize {
        use TokenType::*;

        match self {
            LitInt(s) => s.len(),
            KeywordReturn => "return".len(),
            KeywordInt => "int".len(),
            SomeName(s) => s.len(),
            _ => 0,
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LitInt(int) => write!(f, "LitInt({int})"),
            TokenType::SymbolOpenParen => write!(f, "SymbolOpenParen"),
            TokenType::SymbolCloseParen => write!(f, "SymbolCloseParen"),
            TokenType::SymbolOpenCurly => write!(f, "SymbolOpenCurly"),
            TokenType::SymbolCloseCurly => write!(f, "SymbolCloseCurly"),
            TokenType::SymbolSemiColon => write!(f, "SymbolSemiColon"),
            TokenType::KeywordReturn => write!(f, "KeywordReturn"),
            TokenType::KeywordInt => write!(f, "KeywordInt"),
            TokenType::SomeName(name) => write!(f, "SomeName({name})"),
        }
    }
}

/// A representation of an accepted token from an Oxygen source file.
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    /// Contains the type of the token.
    token_type: TokenType,
    /// Contains the line number where the token appears.
    line: usize,
    /// Contains the column number where the token appears.
    column: usize,
}

impl Token {
    /// Creates a new [`Token`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let token = Token::new(TokenType::Value, 1, 1);
    /// assert_eq(token, Token {
    ///     token_type: TokenType::Value,
    ///     line: 1,
    ///     column: 1,
    /// });
    /// ```
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.token_type, self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_new_token() {
        assert_eq!(
            Token::new(TokenType::KeywordReturn, 1, 1),
            Token {
                token_type: TokenType::KeywordReturn,
                line: 1,
                column: 1
            }
        );
    }

    #[test]
    fn should_return_new_lit_int() {
        assert_eq!(
            TokenType::new_lit_int("99"),
            TokenType::LitInt(Box::from("99"))
        );
    }

    #[test]
    fn should_return_new_some_name() {
        assert_eq!(
            TokenType::new_some_name("some"),
            TokenType::SomeName(Box::from("some"))
        );
    }

    #[test]
    fn should_return_new_int_offset() {
        assert_eq!(TokenType::new_lit_int("99").to_column_offset(), 2);
    }

    #[test]
    fn should_return_some_name_offset() {
        assert_eq!(TokenType::new_some_name("main").to_column_offset(), 4);
    }

    #[test]
    fn should_return_default_offset() {
        assert_eq!(TokenType::SymbolOpenParen.to_column_offset(), 0);
    }

    #[test]
    fn should_return_token_to_string() {
        let token = Token::new(TokenType::SymbolOpenParen, 1, 1);
        assert_eq!(token.to_string(), "SymbolOpenParen:1:1");
    }

    #[test]
    fn should_return_token_type_to_string() {
        assert_eq!(TokenType::LitInt(Box::from("0")).to_string(), "LitInt(0)");
        assert_eq!(TokenType::SymbolOpenParen.to_string(), "SymbolOpenParen");
        assert_eq!(TokenType::SymbolCloseParen.to_string(), "SymbolCloseParen");
        assert_eq!(TokenType::SymbolOpenCurly.to_string(), "SymbolOpenCurly");
        assert_eq!(TokenType::SymbolCloseCurly.to_string(), "SymbolCloseCurly");
        assert_eq!(TokenType::SymbolSemiColon.to_string(), "SymbolSemiColon");
        assert_eq!(TokenType::KeywordReturn.to_string(), "KeywordReturn");
        assert_eq!(TokenType::KeywordInt.to_string(), "KeywordInt");
        assert_eq!(
            TokenType::SomeName(Box::from("name")).to_string(),
            "SomeName(name)"
        );
    }
}
