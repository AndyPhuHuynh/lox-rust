use crate::interpreter::expression::Evaluate;
use crate::runtime::RuntimeResult;
use crate::syntax_tree::expression::Expr;
use crate::syntax_tree::statement::{Print, Stmt};

pub trait Execute {
    fn execute(&self) -> RuntimeResult<()>;
}

impl Execute for Stmt {
    fn execute(&self) -> RuntimeResult<()> {
        match self {
            Stmt::Expr(stmt) => stmt.execute(),
            Stmt::Print(stmt) => stmt.execute(),
        }
    }
}

impl Execute for Expr {
    fn execute(&self) -> RuntimeResult<()> {
        self.evaluate()?;
        Ok(())
    }
}

impl Execute for Print {
    fn execute(&self) -> RuntimeResult<()> {
        let value = self.expr.evaluate()?;
        println!("{}", value);
        Ok(())
    }
}
