use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FhirPathToken {
    // Identifiers and literals
    Identifier(String),
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),

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
    Dollar,  // $
    Percent, // %
    At,      // @

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

impl fmt::Display for FhirPathToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(s) => write!(f, "{s}"),
            Self::String(s) => write!(f, "'{s}'"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Dot => write!(f, "."),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Mod => write!(f, "mod"),
            Self::Equals => write!(f, "="),
            Self::NotEquals => write!(f, "!="),
            Self::LessThan => write!(f, "<"),
            Self::LessThanOrEqual => write!(f, "<="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Not => write!(f, "not"),
            Self::Is => write!(f, "is"),
            Self::As => write!(f, "as"),
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Pipe => write!(f, "|"),
            Self::Dollar => write!(f, "$"),
            Self::Percent => write!(f, "%"),
            Self::At => write!(f, "@"),
            Self::Where => write!(f, "where"),
            Self::Select => write!(f, "select"),
            Self::All => write!(f, "all"),
            Self::Any => write!(f, "any"),
            Self::Empty => write!(f, "empty"),
            Self::Exists => write!(f, "exists"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
