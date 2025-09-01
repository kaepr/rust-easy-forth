#[derive(Debug, PartialEq, Eq)]
pub enum OperatorKind {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(isize),
    Operator(OperatorKind),
}
