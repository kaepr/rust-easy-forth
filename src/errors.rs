use std::fmt;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum EvaluatorError {
    StackUnderflow,
    InvalidOperand,
}

impl fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluatorError::StackUnderflow => write!(f, "Stack underflow."),
            EvaluatorError::InvalidOperand => write!(f, "Invalid operand."),
        }
    }
}
