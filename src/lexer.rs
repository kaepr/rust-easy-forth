use crate::token::{OperatorKind, Token};

pub struct Lexer {}

impl Lexer {
    fn new() -> Self {
        Lexer {}
    }

    fn tokenize(&self, source: &str) -> Vec<Token> {
        source.split_whitespace().map(|s| self.token(s)).collect()
    }

    fn token(&self, source: &str) -> Token {
        if let Ok(n) = source.parse::<isize>() {
            return Token::Number(n);
        }

        if source == "+" {
            return Token::Operator(OperatorKind::Add);
        }

        if source == "-" {
            return Token::Operator(OperatorKind::Subtract);
        }

        if source == "/" {
            return Token::Operator(OperatorKind::Divide);
        }

        if source == "*" {
            return Token::Operator(OperatorKind::Multiply);
        }

        panic!("Error in lexer.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_token() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("42");
        assert_eq!(tokens, vec![Token::Number(42)]);
    }

    #[test]
    fn test_operator_tokens() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("+ - / *");
        assert_eq!(
            tokens,
            vec![
                Token::Operator(OperatorKind::Add),
                Token::Operator(OperatorKind::Subtract),
                Token::Operator(OperatorKind::Divide),
                Token::Operator(OperatorKind::Multiply),
            ]
        );
    }

    #[test]
    fn test_operator_number_tokens() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("+ 42");
        assert_eq!(
            tokens,
            vec![
                Token::Operator(OperatorKind::Add),
                Token::Number(42),
            ]
        );
    }
}
