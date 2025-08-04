pub trait ColumnOffset {
    fn to_col_offset(&self) -> usize;
}

/// A representation of any literals.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literals {
    /// Contains the value of the integer literal.
    Integer(String),
}

impl ColumnOffset for Literals {
    fn to_col_offset(&self) -> usize {
        use Literals::*;

        match self {
            Integer(int) => int.len(),
        }
    }
}

/// A representation of any symbols.
#[derive(Debug, PartialEq, Eq, Clone)]
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

impl ColumnOffset for Symbols {
    fn to_col_offset(&self) -> usize {
        1
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Types {
    /// Represents the keyword `int`.
    Int,
}

impl ColumnOffset for Types {
    fn to_col_offset(&self) -> usize {
        use Types::*;

        match self {
            Int => 3,
        }
    }
}

/// A representation of any keywords.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keywords {
    /// Represents the keyword `return`.
    Return,
}

impl ColumnOffset for Keywords {
    fn to_col_offset(&self) -> usize {
        use Keywords::*;

        match self {
            Return => 6,
        }
    }
}

/// A representation of a type of token.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    /// Contains a literal from [`Literals`].
    Literal(Literals),
    /// Contains a symbol from [`Symbols`].
    Symbol(Symbols),
    /// Contains a keyword from [`Keywords`].
    Keyword(Keywords),
    /// Contains a type from [`Types`].
    Type(Types),
    /// Contains the name of some item as a [`String`].
    SomeName(String),
}

impl ColumnOffset for TokenType {
    fn to_col_offset(&self) -> usize {
        use TokenType::*;

        match self {
            Literal(literal) => literal.to_col_offset(),
            Symbol(symbol) => symbol.to_col_offset(),
            Keyword(keyword) => keyword.to_col_offset(),
            Type(t) => t.to_col_offset(),
            SomeName(name) => name.len(),
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

impl std::convert::From<Types> for TokenType {
    fn from(value: Types) -> Self {
        TokenType::Type(value)
    }
}

impl std::convert::From<Keywords> for TokenType {
    fn from(value: Keywords) -> Self {
        TokenType::Keyword(value)
    }
}

/// A representation of an accepted token from an Oxygen source file.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    /// Contains the type of the token.
    pub token_type: TokenType,
    /// Contains the line number where the token appears.
    pub line: usize,
    /// Contains the column number where the token appears.
    pub column: usize,
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
    fn should_get_col_offset_token_type() {
        assert_eq!(
            TokenType::from(Literals::Integer("99".to_string())).to_col_offset(),
            2
        );
        assert_eq!(TokenType::from(Symbols::OpenParen).to_col_offset(), 1);
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
    }

    #[test]
    fn should_get_col_offset_types() {
        assert_eq!(Types::Int.to_col_offset(), 3);
    }
}
