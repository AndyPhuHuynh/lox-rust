use crate::environment::Environment;
use crate::interpreter::expression::Evaluate;
use crate::runtime::RuntimeResult;
use crate::runtime::value::RuntimeValue;
use crate::syntax_tree::expression::Expr;
use crate::syntax_tree::statement::{Print, Stmt, Var};

pub trait Execute {
    fn execute(&self, env: &mut Environment) -> RuntimeResult<()>;
}

impl Execute for Stmt {
    fn execute(&self, env: &mut Environment) -> RuntimeResult<()> {
        match self {
            Stmt::Expr(stmt) => stmt.execute(env),
            Stmt::Print(stmt) => stmt.execute(env),
            Stmt::Var(stmt) => stmt.execute(env),
        }
    }
}

impl Execute for Expr {
    fn execute(&self, env: &mut Environment) -> RuntimeResult<()> {
        self.evaluate(env)?;
        Ok(())
    }
}

impl Execute for Print {
    fn execute(&self, env: &mut Environment) -> RuntimeResult<()> {
        let value = self.expr.evaluate(env)?;
        println!("{}", value);
        Ok(())
    }
}

impl Execute for Var {
    fn execute(&self, env: &mut Environment) -> RuntimeResult<()> {
        let mut value = RuntimeValue::Nil;

        if let Some(expr) = &self.initializer {
            value = expr.evaluate(env)?;
        }

        env.define(self.name.clone(), value);
        Ok(())
    }
}
