use std::fmt;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum OperatorKind {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl fmt::Debug for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OperatorKind::Add => write!(f, "+"),
            OperatorKind::Subtract => write!(f, "-"),
            OperatorKind::Divide => write!(f, "/"),
            OperatorKind::Multiply => write!(f, "*"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Token {
    Number(isize),
    Operator(OperatorKind),
    Period,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Token::Number(v) => write!(f, "{}", v),
            Token::Period => write!(f, "."),
            Token::Operator(o) => write!(f, "{:?}", o),
        }
    }
}
