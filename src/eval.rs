use crate::{
    errors::EvaluatorError,
    token::{OperatorKind, Token},
};

#[derive(Debug)]
pub struct Evaluator {
    tokens: Vec<Token>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn eval(&mut self, token: Token) -> Result<Option<Token>, EvaluatorError> {
        match token {
            Token::Number(_) => self.push(token),
            Token::Operator(kind) => match self.pop2() {
                Some((t1, t2)) => match (t1, t2) {
                    (Token::Number(v1), Token::Number(v2)) => {
                        let token = Token::Number(self.calculate(v1, v2, kind));
                        self.push(token)
                    }
                    _ => return Err(EvaluatorError::InvalidOperand),
                },
                None => return Err(EvaluatorError::StackUnderflow),
            },
            Token::Period => match self.pop() {
                Some(t) => return Ok(Some(t)),
                None => return Err(EvaluatorError::StackUnderflow),
            },
        }

        Ok(None)
    }

    pub fn eval_all(&mut self, tokens: &[Token]) -> Result<Option<Token>, EvaluatorError> {
        let mut last = None;

        for token in tokens {
            last = self.eval(*token)?;
        }

        Ok(last)
    }

    fn pop2(&mut self) -> Option<(Token, Token)> {
        Some((self.pop()?, self.pop()?))
    }

    fn pop(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn calculate(&self, v1: isize, v2: isize, kind: OperatorKind) -> isize {
        use OperatorKind as K;
        match kind {
            K::Add => v1 + v2,
            K::Subtract => v2 - v1,
            K::Divide => v2 / v1,
            K::Multiply => v1 * v2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        let mut evaluator = Evaluator::new();
        let tokens = vec![
            Token::Number(1),
            Token::Number(2),
            Token::Operator(OperatorKind::Add),
            Token::Period,
        ];

        assert_eq!(
            Token::Number(3),
            evaluator.eval_all(&tokens).unwrap().unwrap()
        );
    }

    #[test]
    fn subtract_two_numbers() {
        let mut evaluator = Evaluator::new();
        let tokens = vec![
            Token::Number(1),
            Token::Number(2),
            Token::Operator(OperatorKind::Subtract),
            Token::Period,
        ];

        assert_eq!(
            Token::Number(-1),
            evaluator.eval_all(&tokens).unwrap().unwrap()
        );
    }

    #[test]
    fn multiply_two_numbers() {
        let mut evaluator = Evaluator::new();
        let tokens = vec![
            Token::Number(2),
            Token::Number(2),
            Token::Operator(OperatorKind::Multiply),
            Token::Period,
        ];

        assert_eq!(
            Token::Number(4),
            evaluator.eval_all(&tokens).unwrap().unwrap()
        );
    }

    #[test]
    fn divide_two_numbers() {
        let mut evaluator = Evaluator::new();
        let tokens = vec![
            Token::Number(1),
            Token::Number(2),
            Token::Operator(OperatorKind::Divide),
            Token::Period,
        ];

        assert_eq!(
            Token::Number(0),
            evaluator.eval_all(&tokens).unwrap().unwrap()
        );
    }

    #[test]
    fn binary_operation_not_enough_operands_stack_underflow() {
        use EvaluatorError as E;
        let mut evaluator = Evaluator::new();
        let add_tokens = vec![Token::Number(1), Token::Operator(OperatorKind::Divide)];
        let subtract_tokens = vec![Token::Number(1), Token::Operator(OperatorKind::Divide)];
        let multiply_tokens = vec![Token::Number(1), Token::Operator(OperatorKind::Divide)];
        let divide_tokens = vec![Token::Number(1), Token::Operator(OperatorKind::Divide)];

        assert_eq!(
            E::StackUnderflow,
            evaluator.eval_all(&add_tokens).unwrap_err()
        );
        assert_eq!(
            E::StackUnderflow,
            evaluator.eval_all(&subtract_tokens).unwrap_err()
        );
        assert_eq!(
            E::StackUnderflow,
            evaluator.eval_all(&multiply_tokens).unwrap_err()
        );
        assert_eq!(
            E::StackUnderflow,
            evaluator.eval_all(&divide_tokens).unwrap_err()
        );
    }

    #[test]
    fn period_not_enough_operands_stack_underflow() {
        let mut evaluator = Evaluator::new();
        let tokens = vec![Token::Period];

        assert_eq!(
            EvaluatorError::StackUnderflow,
            evaluator.eval_all(&tokens).unwrap_err()
        );
    }
}
