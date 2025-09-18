use crate::{
    errors::LexerError,
    token::{OperatorKind, Token},
};

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Lexer {}
    }

    pub fn tokenize(&self, source: &str) -> Result<Vec<Token>, LexerError> {
        source.split_whitespace().map(|s| self.token(s)).collect()
    }

    fn token(&self, source: &str) -> Result<Token, LexerError> {
        if let Ok(n) = source.parse::<isize>() {
            return Ok(Token::Number(n));
        }

        match source {
            "+" => Ok(Token::Operator(OperatorKind::Add)),
            "/" => Ok(Token::Operator(OperatorKind::Divide)),
            "-" => Ok(Token::Operator(OperatorKind::Subtract)),
            "*" => Ok(Token::Operator(OperatorKind::Multiply)),
            "." => Ok(Token::Period),
            _ => Err(LexerError::InvalidToken(source.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_token() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("42").unwrap();
        assert_eq!(tokens, vec![Token::Number(42)]);
    }

    #[test]
    fn test_operator_tokens() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("+ - / *").unwrap();
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
        let tokens = lexer.tokenize("+ 42").unwrap();
        assert_eq!(
            tokens,
            vec![Token::Operator(OperatorKind::Add), Token::Number(42),]
        );
    }

    #[test]
    fn test_period() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(".").unwrap();
        assert_eq!(tokens, vec![Token::Period]);
    }
}
