mod error;
mod interpreter;
mod parser;
mod runtime;
mod scanner;
mod syntax_tree;
mod token;

use crate::interpreter::statement::Execute;
use crate::parser::Parser;
use crate::scanner::Scanner;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn run(source: &str, exit_on_error: bool) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.clone());
    let statements = match parser.parse() {
        Ok(stmt) => stmt,
        Err(_) => {
            println!("Parse error");
            if exit_on_error {
                process::exit(65);
            } else {
                return;
            }
        }
    };

    for stmt in statements {
        match stmt.execute() {
            Ok(_) => {},
            Err(err) => {
                match err.line {
                    Some(line) => {
                        println!("Runtime error at line {}, {}", line, err.message);
                    }
                    None => {
                        println!("Runtime error: {}", err.message);
                    }
                };
                if exit_on_error {
                    process::exit(70);
                } else {
                    return;
                }
            }
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    run(&contents, true);
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

        run(&input, false);
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
