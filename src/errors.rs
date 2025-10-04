use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LexerError {
    #[error("invalid token: {0}")]
    InvalidToken(String),
    #[error("unexpected end of input")]
    UnexpectedEof,
}

#[derive(Error, Debug, PartialEq)]
pub enum EvalError {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("invalid operand")]
    InvalidOperand,
    #[error("not a number")]
    NotANumber,
    #[error("unexpected semicolon")]
    UnexpectedSemicolon,
    #[error("unexpected colon")]
    UnexpectedColon,
    #[error("definition did not start with word")]
    InvalidDefinition,
    #[error("no definition found")]
    NoDefinition,
}
