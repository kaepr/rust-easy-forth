use crate::{
    errors::EvalError,
    token::{OperatorKind, Token},
};
use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, PartialEq)]
enum Mode {
    Compile,
    Interpret,
}

#[derive(Debug)]
pub struct Evaluator<'a> {
    tokens: Vec<Token<'a>>,
    mode: Mode,
    definition: Vec<Token<'a>>,
    dictionary: HashMap<Cow<'a, str>, Vec<Token<'a>>>,
}

impl<'a> Evaluator<'a> {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            definition: vec![],
            mode: Mode::Interpret,
            dictionary: Evaluator::empty_dictionary(),
        }
    }

    fn empty_dictionary() -> HashMap<Cow<'a, str>, Vec<Token<'a>>> {
        HashMap::new()
    }

    pub fn eval(&mut self, token: Token<'a>) -> Result<Option<Token<'a>>, EvalError> {
        match self.mode {
            Mode::Compile => match token {
                Token::Semicolon => {
                    self.mode = Mode::Interpret;

                    if let Some(name) = self.definition.first() {
                        match name {
                            Token::Word(word) => {
                                let key = Cow::Owned(word.clone().into_owned());
                                let value: Vec<Token<'a>> = self
                                    .definition
                                    .iter()
                                    .skip(1)
                                    .map(|t| t.to_owned())
                                    .collect();

                                self.dictionary.insert(key, value);
                                return Ok(None);
                            }

                            _other => return Err(EvalError::InvalidDefinition),
                        }
                    }

                    return Err(EvalError::InvalidDefinition);
                }
                Token::Colon => return Err(EvalError::UnexpectedColon),
                _ => self.add_to_definition(token),
            },
            Mode::Interpret => match token {
                Token::Number(_) => self.push(token),
                Token::Operator(kind) => match self.pop2() {
                    Some((t1, t2)) => match (t1, t2) {
                        (Token::Number(v1), Token::Number(v2)) => {
                            let token = Token::Number(self.calculate(v1, v2, kind));
                            self.push(token)
                        }
                        _ => return Err(EvalError::InvalidOperand),
                    },
                    None => return Err(EvalError::StackUnderflow),
                },
                Token::Period => match self.pop() {
                    Some(t) => return Ok(Some(t.clone())),
                    None => return Err(EvalError::StackUnderflow),
                },
                Token::Colon => match self.mode {
                    Mode::Interpret => self.mode = Mode::Compile,
                    Mode::Compile => return Err(EvalError::UnexpectedSemicolon),
                },
                Token::Semicolon => return Err(EvalError::UnexpectedSemicolon),
                Token::Word(ref word) => match self.mode {
                    Mode::Compile => self.push(token),
                    Mode::Interpret => match self.dictionary.get(word).cloned() {
                        Some(definition) => return self.eval_all(&definition),
                        None => return Err(EvalError::NoDefinition),
                    },
                },
            },
        }

        Ok(None)
    }

    pub fn eval_all(&mut self, tokens: &[Token<'a>]) -> Result<Option<Token<'a>>, EvalError> {
        let mut last: Option<Token<'a>> = None;

        for token in tokens.iter() {
            last = self.eval(token.clone())?;
        }

        Ok(last)
    }

    fn pop2(&mut self) -> Option<(Token<'a>, Token<'a>)> {
        let t1 = self.pop()?;
        let t2 = self.pop()?;
        Some((t1, t2))
    }

    fn pop(&mut self) -> Option<Token<'a>> {
        self.tokens.pop()
    }

    fn push(&mut self, token: Token<'a>) {
        self.tokens.push(token);
    }

    fn add_to_definition(&mut self, token: Token<'a>) {
        self.definition.push(token);
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
        use EvalError as E;
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
            EvalError::StackUnderflow,
            evaluator.eval_all(&tokens).unwrap_err()
        );
    }

    #[test]
    fn dictionary_definition() {
        let mut evaluator = Evaluator::new();
        let tokens = vec![
            Token::Colon,
            Token::Word(Cow::Borrowed("foo")),
            Token::Number(100),
            Token::Operator(OperatorKind::Add),
            Token::Semicolon,
            Token::Number(1000),
            Token::Word(Cow::Borrowed("foo")),
            Token::Word(Cow::Borrowed("foo")),
            Token::Word(Cow::Borrowed("foo")),
            Token::Word(Cow::Borrowed("foo")),
            Token::Period,
        ];

        assert_eq!(
            Token::Number(1400),
            evaluator.eval_all(&tokens).unwrap().unwrap()
        );
    }
}
