use std::{borrow::Cow, fmt};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum OperatorKind {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl fmt::Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OperatorKind::Add => write!(f, "+"),
            OperatorKind::Subtract => write!(f, "-"),
            OperatorKind::Divide => write!(f, "/"),
            OperatorKind::Multiply => write!(f, "*"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token<'a> {
    Number(isize),
    Operator(OperatorKind),
    Word(Cow<'a, str>),
    Colon,
    Semicolon,
    Period,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Number(v) => write!(f, "{}", v),
            Token::Period => write!(f, "."),
            Token::Operator(o) => write!(f, "{:?}", o),
            Token::Word(w) => write!(f, "{}", w.to_string()),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
        }
    }
}
