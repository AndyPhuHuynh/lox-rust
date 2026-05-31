mod error;
mod scanner;
mod syntax_tree;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};

use crate::scanner::Scanner;
use crate::syntax_tree::expressions::{BinaryOp, Expr, Literal, UnaryOp};

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    println!("{:#?}", tokens);
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
    let expr = Expr::binary(
        Expr::unary(
            UnaryOp::Negation,
            Expr::literal_num(123f64)
        ),
        BinaryOp::Mul,
        Expr::grouping(
            Expr::literal_num(45.67)
        )
    );

    println!("{}", expr.print());

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
