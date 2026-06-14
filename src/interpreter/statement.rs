use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::interpreter::expression::Evaluate;
use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::{FunctionRef, RuntimeValue};
use crate::syntax_tree::expression::Expr;
use crate::syntax_tree::statement::{
    Block, ClassDecl, FunctionDecl, If, Print, Return, Stmt, Var, While,
};

pub trait Execute {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()>;
}

impl Execute for Stmt {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        match self {
            Stmt::Block(stmt) => stmt.execute(interpreter, env),
            Stmt::Class(stmt) => stmt.execute(interpreter, env),
            Stmt::Expr(stmt) => stmt.execute(interpreter, env),
            Stmt::Function(stmt) => stmt.execute(interpreter, env),
            Stmt::If(stmt) => stmt.execute(interpreter, env),
            Stmt::Print(stmt) => stmt.execute(interpreter, env),
            Stmt::Return(stmt) => stmt.execute(interpreter, env),
            Stmt::While(stmt) => stmt.execute(interpreter, env),
            Stmt::Var(stmt) => stmt.execute(interpreter, env),
        }
    }
}

impl Execute for Block {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        let mut nested_env = EnvRef::with_enclosing(Some(env.clone()));

        for stmt in &self.stmts {
            stmt.execute(interpreter, &mut nested_env)?;
        }

        Ok(())
    }
}

impl Execute for ClassDecl {
    fn execute(&self, _: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        env.define(self.name.clone(), RuntimeValue::Nil);
        let class_ref = RuntimeValue::Class(self.clone().into_ref(env));
        env.assign(self.name.clone(), class_ref);
        Ok(())
    }
}

impl Execute for Expr {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        self.evaluate(interpreter, env)?;
        Ok(())
    }
}

impl Execute for FunctionDecl {
    fn execute(&self, _: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        match env.define(
            self.name.clone(),
            RuntimeValue::Function(FunctionRef::new(
                self.name.clone(),
                self.params
                    .clone()
                    .into_iter()
                    .map(|(param, _)| param)
                    .collect(),
                self.body.clone(),
                false,
                env.clone(),
            )),
        ) {
            None => Err(RuntimeException::redefinition_error(&self.name, self.line)),
            Some(_) => Ok(()),
        }
    }
}

impl Execute for If {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        if self.condition.evaluate(interpreter, env)?.is_truthy() {
            return self.then_branch.execute(interpreter, env);
        } else if let Some(else_branch) = &self.else_branch {
            return else_branch.execute(interpreter, env);
        }
        Ok(())
    }
}

impl Execute for Print {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        let value = self.expr.evaluate(interpreter, env)?;
        println!("{}", value);
        Ok(())
    }
}

impl Execute for Return {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        match &self.expr {
            None => Err(RuntimeException::return_value(RuntimeValue::Nil, self.line)),
            Some(expr) => Err(RuntimeException::return_value(
                expr.evaluate(interpreter, env)?,
                self.line,
            )),
        }
    }
}

impl Execute for While {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        while self.condition.evaluate(interpreter, env)?.is_truthy() {
            self.body.execute(interpreter, env)?;
        }
        Ok(())
    }
}

impl Execute for Var {
    fn execute(&self, interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<()> {
        let mut value = RuntimeValue::Nil;

        if let Some(expr) = &self.initializer {
            value = expr.evaluate(interpreter, env)?;
        }

        match env.define(self.name.clone(), value) {
            None => Err(RuntimeException::redefinition_error(&self.name, self.line)),
            Some(_) => Ok(()),
        }
    }
}
