mod analysis;
mod environment;
mod error;
mod interpreter;
mod parser;
mod runtime;
mod scanner;
mod syntax_tree;
mod token;

use crate::analysis::resolver::Resolver;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::runtime::error::RuntimeException;
use crate::scanner::Scanner;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn display_error(error: RuntimeException) -> String {
    match error {
        RuntimeException::RuntimeError { message, line } => {
            if let Some(line) = line {
                format!("Runtime error at line {}: {}", line, message)
            } else {
                format!("Runtime error: {}", message)
            }
        }
        RuntimeException::Return {
            value: _value,
            line,
        } => {
            if let Some(line) = line {
                format!(
                    "Runtime error at line {}: return statement encountered outside of function or method",
                    line
                )
            } else {
                "Runtime error: return statement encountered outside of function or method"
                    .to_string()
            }
        }
    }
}

fn run(interpreter: &mut Interpreter, source: &str, exit_on_error: bool) {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.clone());
    let mut statements = match parser.parse() {
        Ok(stmt) => stmt,
        Err(_) => {
            println!("Parse error");
            if exit_on_error {
                exit(65);
            } else {
                return;
            }
        }
    };

    let mut resolver = Resolver::new();
    resolver.resolve_statements(&mut statements);
    if resolver.has_encountered_errors() {
        if exit_on_error {
            exit(65);
        } else {
            return;
        }
    }
    
    match interpreter.interpret(&statements) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", display_error(err));
            if exit_on_error {
                exit(70);
            } else {
                return;
            }
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;

    let mut interpreter = Interpreter::new();
    run(&mut interpreter, &contents, true);

    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut interpreter = Interpreter::new();

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;

        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            return Ok(());
        }

        run(&mut interpreter, &input, false);
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
