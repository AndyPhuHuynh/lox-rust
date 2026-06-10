pub mod expression;
pub mod statement;

use crate::environment::EnvRef;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::syntax_tree::statement::Stmt;

pub struct Interpreter {
    env: EnvRef,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: EnvRef::with_enclosing(None),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> RuntimeResult<()> {
        for stmt in statements {
            stmt.execute(&mut self.env)?
        }
        Ok(())
    }
}
