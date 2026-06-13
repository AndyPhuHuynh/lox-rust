pub mod expression;
pub mod statement;

use crate::environment::EnvRef;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::syntax_tree::statement::Stmt;

pub struct Interpreter {
    pub globals: EnvRef,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            globals: EnvRef::with_enclosing(None),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> RuntimeResult<()> {
        let globals = self.globals.clone();
        for stmt in statements {
            stmt.execute(self, &mut globals.clone())?
        }
        Ok(())
    }
}
