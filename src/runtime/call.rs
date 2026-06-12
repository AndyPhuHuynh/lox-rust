use crate::environment::EnvRef;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::RuntimeValue;
use crate::syntax_tree::statement::Function;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Callable {
    fn call(self, args: &[RuntimeValue], env: &mut EnvRef) -> RuntimeResult<RuntimeValue>;
}

impl Callable for RuntimeValue {
    fn call(self, args: &[RuntimeValue], _: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        match self {
            RuntimeValue::Function { func, closure } => func.call(args, &mut closure.clone()),
            _ => Err(RuntimeException::with_message(
                "You can only call functions or classes",
            )),
        }
    }
}

impl Callable for Rc<RefCell<Function>> {
    fn call(self, args: &[RuntimeValue], env: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        if args.len() != self.borrow().params.len() {
            return Err(RuntimeException::with_message(&format!(
                "Expected {} arguments but got {} for call to {}",
                self.borrow().params.len(),
                args.len(),
                self.borrow().name
            )));
        }
        let mut new_env = EnvRef::with_enclosing(Some(env.clone()));

        for i in 0..args.len() {
            new_env.define(self.borrow().params[i].clone(), args[i].clone());
        }

        for stmt in &self.borrow().body {
            match stmt.execute(&mut new_env) {
                Ok(_) => {}
                Err(RuntimeException::Return { value, line: _line }) => return Ok(value),
                Err(err) => return Err(err),
            }
        }

        Ok(RuntimeValue::Nil)
    }
}
