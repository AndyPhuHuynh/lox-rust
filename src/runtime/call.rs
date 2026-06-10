use std::rc::Rc;
use crate::environment::EnvRef;
use crate::interpreter::statement::Execute;
use crate::runtime::error::RuntimeError;
use crate::runtime::RuntimeResult;
use crate::runtime::value::RuntimeValue;
use crate::syntax_tree::statement::Function;

pub trait Callable {
    fn call(self, args: &[RuntimeValue], env: &mut EnvRef) -> RuntimeResult<RuntimeValue>;
}

impl Callable for RuntimeValue {
    fn call(self, args: &[RuntimeValue], env: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        match self {
            RuntimeValue::Function(func) => func.call(args, env),
            _ => Err(RuntimeError::with_message(
                "You can only call functions or classes",
            )),
        }
    }
}

impl Callable for Rc<Function> {
    fn call(self, args: &[RuntimeValue], env: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        if args.len() != self.params.len() {
            return Err(RuntimeError::with_message(
                &format!("Expected {} arguments but got {} for call to {}", self.params.len(), args.len(), self.name),
            ))
        }
        let mut new_env = EnvRef::with_enclosing(Some(env.clone()));

        for i in 0..args.len() {
            new_env.define(self.params[i].clone(), args[i].clone());
        }

        for stmt in &self.body {
            stmt.execute(&mut new_env)?;
        }

        Ok(RuntimeValue::Nil)
    }
}