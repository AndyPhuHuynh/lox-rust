mod error;
mod interpreter;
mod parser;
mod runtime;
mod scanner;
mod syntax_tree;
mod token;

use crate::parser::Parser;
use crate::scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.clone());
    match parser.parse() {
        Ok(expr) => {
            interpreter::interpret(expr);
        }
        Err(_) => {
            println!("Unable to parse. Encountered parse error");
        }
    };
}

fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(&contents);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;

        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            return Ok(());
        }

        run(&input);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        eprintln!("Usage: rlox [script]")
    } else if args.len() == 1 {
        run_file(&args[0])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}
