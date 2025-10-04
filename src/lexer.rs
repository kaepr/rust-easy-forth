use std::borrow::Cow;

use crate::{
    errors::LexerError,
    token::{OperatorKind, Token},
};

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Lexer {}
    }

    pub fn tokenize<'a>(&self, source: &'a str) -> Result<Vec<Token<'a>>, LexerError> {
        source.split_whitespace().map(|s| self.token(s)).collect()
    }

    fn token<'a>(&self, source: &'a str) -> Result<Token<'a>, LexerError> {
        if let Ok(n) = source.parse::<isize>() {
            return Ok(Token::Number(n));
        }

        match source {
            "+" => Ok(Token::Operator(OperatorKind::Add)),
            "/" => Ok(Token::Operator(OperatorKind::Divide)),
            "-" => Ok(Token::Operator(OperatorKind::Subtract)),
            "*" => Ok(Token::Operator(OperatorKind::Multiply)),
            "." => Ok(Token::Period),
            ":" => Ok(Token::Colon),
            ";" => Ok(Token::Semicolon),
            _ => {
                if source.chars().all(|c| c.is_ascii_alphanumeric()) {
                    Ok(Token::Word(Cow::Borrowed(source)))
                } else {
                    Err(LexerError::InvalidToken(source.to_string()))
                }
            }
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

    #[test]
    fn test_word() {
        let dup = "dup";
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("dup").unwrap();
        assert_eq!(tokens, vec![Token::Word(Cow::Borrowed(dup))]);
    }

    #[test]
    fn test_multiple_word() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize("dup swap").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Word(Cow::Borrowed("dup")),
                Token::Word(Cow::Owned("swap".to_string()))
            ]
        );
    }

    #[test]
    fn test_function_defition() {
        let lexer = Lexer::new();
        let tokens = lexer.tokenize(": foo  100 + ;").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Colon,
                Token::Word(Cow::Borrowed("foo")),
                Token::Number(100),
                Token::Operator(OperatorKind::Add),
                Token::Semicolon,
            ]
        );
    }
}
