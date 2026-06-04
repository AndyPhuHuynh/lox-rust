use crate::environment::Environment;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::syntax_tree::statement::Stmt;

pub mod expression;
pub mod statement;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> RuntimeResult<()> {
        for stmt in statements {
            stmt.execute(&mut self.env)?
        }
        Ok(())
    }
}
