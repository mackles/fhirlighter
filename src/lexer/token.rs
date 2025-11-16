use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Identifiers and literals
    Identifier,
    String,
    Number(f64),
    Integer(i64),
    Boolean(bool),
    ISODate,
    ISODateTime,

    // Operators
    Dot,                // .
    Plus,               // +
    Minus,              // -
    Multiply,           // *
    Divide,             // /
    Mod,                // mod
    Equals,             // =
    NotEquals,          // !=
    LessThan,           // <
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=
    And,                // and
    Or,                 // or
    Xor,                // xor
    Not,                // not
    Is,                 // is
    As,                 // as

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Pipe,         // |

    // Special
    Dollar,   // $
    Percent,  // %
    At,       // @
    BackTick, // `

    // Keywords
    Where,  // where
    Select, // select
    All,    // all
    Any,    // any
    Empty,  // empty
    Exists, // exists

    // End of input
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    #[must_use]
    pub const fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }

    /// Get the text for this token from the original input
    #[must_use]
    pub fn text<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start..self.end]
    }

    /// Get the length of this token
    #[must_use]
    pub const fn length(&self) -> usize {
        self.end - self.start
    }
}

// Note: FhirPathToken alias removed as the migration to Token is complete

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::String => write!(f, "string"),
            TokenKind::Number(n) => write!(f, "{n}"),
            TokenKind::Integer(i) => write!(f, "{i}"),
            TokenKind::Boolean(b) => write!(f, "{b}"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Mod => write!(f, "mod"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::NotEquals => write!(f, "!="),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::LessThanOrEqual => write!(f, "<="),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::GreaterThanOrEqual => write!(f, ">="),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Xor => write!(f, "xor"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::Is => write!(f, "is"),
            TokenKind::As => write!(f, "as"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Dollar => write!(f, "$"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::ISODateTime => write!(f, "@ISODateTime"),
            TokenKind::ISODate => write!(f, "@ISODate"),
            TokenKind::At => write!(f, "@"),
            TokenKind::Where => write!(f, "where"),
            TokenKind::Select => write!(f, "select"),
            TokenKind::All => write!(f, "all"),
            TokenKind::Any => write!(f, "any"),
            TokenKind::Empty => write!(f, "empty"),
            TokenKind::Exists => write!(f, "exists"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::BackTick => write!(f, "`"),
        }
    }
}
