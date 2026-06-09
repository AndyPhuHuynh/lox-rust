use crate::environment::EnvRef;
use crate::interpreter::expression::Evaluate;
use crate::runtime::RuntimeResult;
use crate::runtime::value::RuntimeValue;
use crate::syntax_tree::expression::Expr;
use crate::syntax_tree::statement::{Block, If, Print, Stmt, Var};

pub trait Execute {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()>;
}

impl Execute for Stmt {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()> {
        match self {
            Stmt::Expr(stmt) => stmt.execute(env),
            Stmt::If(stmt) => stmt.execute(env),
            Stmt::Print(stmt) => stmt.execute(env),
            Stmt::Var(stmt) => stmt.execute(env),
            Stmt::Block(stmt) => stmt.execute(env),
        }
    }
}

impl Execute for Expr {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()> {
        self.evaluate(env)?;
        Ok(())
    }
}

impl Execute for If {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()> {
        if self.condition.evaluate(env)?.is_truthy() {
            return self.then_branch.execute(env)
        } else if let Some(else_branch) = &self.else_branch {
            return else_branch.execute(env)
        }
        Ok(())
    }
}

impl Execute for Print {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()> {
        let value = self.expr.evaluate(env)?;
        println!("{}", value);
        Ok(())
    }
}

impl Execute for Var {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()> {
        let mut value = RuntimeValue::Nil;

        if let Some(expr) = &self.initializer {
            value = expr.evaluate(env)?;
        }

        env.define(self.name.clone(), value);
        Ok(())
    }
}

impl Execute for Block {
    fn execute(&self, env: &mut EnvRef) -> RuntimeResult<()> {
        let mut nested_env = EnvRef::with_enclosing(Some(env.clone()));

        for stmt in &self.stmts {
            stmt.execute(&mut nested_env)?;
        }

        Ok(())
    }
}