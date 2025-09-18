mod errors;
mod eval;
mod lexer;
mod token;

use crate::{eval::Evaluator, lexer::Lexer};

fn main() {
    let source = "1 2 +";
    let lexer = Lexer::new();
    let tokens = lexer.tokenize(source).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval_all(&tokens).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    println!("{}", match result {
        Some(t) => format!("{:?}", t),
        None => "".to_string(),
    });
}
