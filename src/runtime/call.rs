use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::{ClassRef, FunctionRef, InstanceRef, InstanceRefExt, RuntimeValue};

pub trait Callable {
    fn call(self, args: &[RuntimeValue], interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<RuntimeValue>;
}

impl Callable for RuntimeValue {
    fn call(self, args: &[RuntimeValue], interpreter: &mut Interpreter, env: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        match self {
            RuntimeValue::Function(func) => func.call(args, interpreter, env),
            RuntimeValue::Class(class) => class.call(args, interpreter, env),
            _ => Err(RuntimeException::with_message(
                "You can only call functions or classes",
            )),
        }
    }
}

impl Callable for FunctionRef {
    fn call(self, args: &[RuntimeValue], interpreter: &mut Interpreter, _: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        if args.len() != self.borrow().params.len() {
            return Err(RuntimeException::with_message(&format!(
                "Expected {} arguments but got {} for call to {}",
                self.borrow().params.len(),
                args.len(),
                self.borrow().name
            )));
        }
        let mut new_env = EnvRef::with_enclosing(Some(self.borrow().closure.clone()));

        for i in 0..args.len() {
            new_env.define(self.borrow().params[i].clone(), args[i].clone());
        }

        for stmt in &self.borrow().body {
            match stmt.execute(interpreter, &mut new_env) {
                Ok(_) => {}
                Err(RuntimeException::Return { value, line: _line }) => return Ok(value),
                Err(err) => return Err(err),
            }
        }

        Ok(RuntimeValue::Nil)
    }
}

impl Callable for ClassRef {
    fn call(self, _: &[RuntimeValue], _: &mut Interpreter, _: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        Ok(RuntimeValue::Instance(InstanceRef::new_instance(
            self.clone(),
        )))
    }
}
