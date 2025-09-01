use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String),
    UnexpectedEof,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidToken(tok) => write!(f, "Invalid token: {}", tok),
            LexerError::UnexpectedEof => write!(f, "Unexpected end of input."),
        }
    }
}
