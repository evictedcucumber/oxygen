/// A representation of any literals.
#[derive(Debug, PartialEq, Eq)]
pub enum Literals {
    /// Contains the value of the integer literal.
    Integer(String),
}

impl Literals {
    /// Returns the col offset of the current literal.
    pub fn to_col_offset(&self) -> usize {
        match self {
            Literals::Integer(int) => int.len(),
        }
    }
}

impl std::fmt::Display for Literals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Literals::Integer(int) => format!("LiteralInteger({int})"),
        };

        write!(f, "{s}")
    }
}

/// A representation of any symbols.
#[derive(Debug, PartialEq, Eq)]
pub enum Symbols {
    /// Represents an `(`.
    OpenParen,
    /// Represents an `)`.
    CloseParen,
    /// Represents an `{`.
    OpenCurly,
    /// Represents an `}`.
    CloseCurly,
    /// Represents an `;`.
    SemiColon,
}

impl std::fmt::Display for Symbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Symbols::OpenParen => "SymbolOpenParen",
            Symbols::CloseParen => "SymbolCloseParen",
            Symbols::OpenCurly => "SymbolOpenCurly",
            Symbols::CloseCurly => "SymbolCloseCurly",
            Symbols::SemiColon => "SymbolSemiColon",
        };

        write!(f, "{s}")
    }
}

/// A representation of any keywords.
#[derive(Debug, PartialEq, Eq)]
pub enum Keywords {
    /// Represents the keyword `return`.
    Return,
    /// Represents the keyword `int`.
    Int,
}

impl Keywords {
    /// Returns the column offset of the keyword.
    pub fn to_col_offset(&self) -> usize {
        match self {
            Keywords::Return => "return".len(),
            Keywords::Int => "int".len(),
        }
    }
}

impl std::fmt::Display for Keywords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Keywords::Return => "KeywordReturn",
            Keywords::Int => "KeywordInt",
        };

        write!(f, "{s}")
    }
}

/// A representation of a type of token.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    /// Contains a literal from [`Literals`].
    Literal(Literals),
    /// Contains a symbol from [`Symbols`].
    Symbol(Symbols),
    /// Contains a keyword from [`Keywords`].
    Keyword(Keywords),
    /// Contains the name of some item as a [`String`].
    SomeName(String),
}

impl TokenType {
    /// Returns the column offset of the [`TokenType`].
    pub fn to_col_offset(&self) -> usize {
        match self {
            TokenType::Literal(literal) => literal.to_col_offset(),
            TokenType::Symbol(_) => 0,
            TokenType::Keyword(keyword) => keyword.to_col_offset(),
            TokenType::SomeName(name) => name.len(),
        }
    }
}

impl std::convert::From<Literals> for TokenType {
    fn from(value: Literals) -> Self {
        TokenType::Literal(value)
    }
}

impl std::convert::From<Symbols> for TokenType {
    fn from(value: Symbols) -> Self {
        TokenType::Symbol(value)
    }
}

impl std::convert::From<Keywords> for TokenType {
    fn from(value: Keywords) -> Self {
        TokenType::Keyword(value)
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenType::Literal(literal) => format!("{literal}"),
            TokenType::Symbol(symbol) => format!("{symbol}"),
            TokenType::Keyword(keyword) => format!("{keyword}"),
            TokenType::SomeName(name) => format!("SomeName({name})"),
        };

        write!(f, "{s}")
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
    pub fn new(token_type: impl std::convert::Into<TokenType>, line: usize, column: usize) -> Self {
        Self {
            token_type: token_type.into(),
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
            Token::new(Keywords::Return, 1, 1),
            Token {
                token_type: Keywords::Return.into(),
                line: 1,
                column: 1
            }
        );
    }

    #[test]
    fn should_display_literals() {
        assert_eq!(
            Literals::Integer("99".to_string()).to_string(),
            "LiteralInteger(99)"
        )
    }

    #[test]
    fn should_display_symbols() {
        assert_eq!(Symbols::OpenParen.to_string(), "SymbolOpenParen");
        assert_eq!(Symbols::CloseParen.to_string(), "SymbolCloseParen");
        assert_eq!(Symbols::OpenCurly.to_string(), "SymbolOpenCurly");
        assert_eq!(Symbols::CloseCurly.to_string(), "SymbolCloseCurly");
        assert_eq!(Symbols::SemiColon.to_string(), "SymbolSemiColon");
    }

    #[test]
    fn should_display_keywords() {
        assert_eq!(Keywords::Return.to_string(), "KeywordReturn");
        assert_eq!(Keywords::Int.to_string(), "KeywordInt");
    }

    #[test]
    fn should_display_token_type() {
        assert_eq!(
            TokenType::from(Literals::Integer("99".to_string())).to_string(),
            "LiteralInteger(99)"
        );
        assert_eq!(
            TokenType::from(Symbols::OpenParen).to_string(),
            "SymbolOpenParen"
        );
        assert_eq!(
            TokenType::from(Keywords::Return).to_string(),
            "KeywordReturn"
        );
        assert_eq!(
            TokenType::SomeName("name".to_string()).to_string(),
            "SomeName(name)"
        );
    }

    #[test]
    fn should_get_col_offset_token_type() {
        assert_eq!(
            TokenType::from(Literals::Integer("99".to_string())).to_col_offset(),
            2
        );
        assert_eq!(TokenType::from(Symbols::OpenParen).to_col_offset(), 0);
        assert_eq!(TokenType::from(Keywords::Return).to_col_offset(), 6);
        assert_eq!(TokenType::SomeName("name".to_string()).to_col_offset(), 4);
    }

    #[test]
    fn should_get_col_offset_literals() {
        assert_eq!(Literals::Integer("99".to_string()).to_col_offset(), 2);
    }

    #[test]
    fn should_get_col_offset_keywords() {
        assert_eq!(Keywords::Return.to_col_offset(), 6);
        assert_eq!(Keywords::Int.to_col_offset(), 3)
    }

    #[test]
    fn should_display_token() {
        assert_eq!(
            Token::new(Symbols::OpenParen, 1, 1).to_string(),
            "SymbolOpenParen:1:1"
        );
    }
}
