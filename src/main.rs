mod errors;
mod eval;
mod lexer;
mod token;

use crate::{eval::Evaluator, lexer::Lexer};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file-path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let source: Vec<String> = fs::read_to_string(file_path)
        .unwrap_or_else(|err| {
            eprintln!("Error reading {}: {}", file_path, err);
            process::exit(1);
        })
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let lexer = Lexer::new();
    let mut evaluator = Evaluator::new();

    for line in source {
        let tokens = lexer.tokenize(&line).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

        let result = evaluator.eval_all(&tokens).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

        println!(
            "{}",
            match result {
                Some(t) => format!("{:?}", t),
                None => "".to_string(),
            }
        );
    }
}
