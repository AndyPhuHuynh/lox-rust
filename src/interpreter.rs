pub mod interpret;

use crate::interpreter::interpret::Interpret;
use crate::syntax_tree::expressions::Expr;

pub fn interpret(expr: Expr) {
    match expr.interpret() {
        Ok(value) => {
            println!("{}", value);
        }
        Err(err) => match err.line {
            Some(line) => {
                println!("Runtime error at line {}, {}", line, err.message);
            }
            None => {
                println!("Runtime error: {}", err.message);
            }
        },
    }
}
