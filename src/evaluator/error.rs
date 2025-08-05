
#[derive(Debug)]
pub enum Error {
    Parse(String),
    Unrecoverable(String),
    IntegerConversion(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "Parse error: {msg}"),
            Self::Unrecoverable(msg) => write!(f, "Unrecoverable error: {msg}"),
            Self::IntegerConversion(msg) => write!(f, "Unparseable index: {msg}"),
        }
    }
}

impl std::error::Error for Error {}