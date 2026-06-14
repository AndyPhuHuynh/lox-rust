use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::{ClassRef, FunctionRef, InstanceRef, InstanceRefExt, RuntimeValue};

pub trait Callable {
    fn call(
        self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue>;
}

impl Callable for RuntimeValue {
    fn call(
        self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
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
    fn call(
        self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        _: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        if args.len() != self.func.borrow().params.len() {
            return Err(RuntimeException::arity_error(
                &self.func.borrow().name,
                self.func.borrow().params.len(),
                args.len(),
            ));
        }
        let mut new_env = EnvRef::with_enclosing(Some(self.closure.clone()));

        for i in 0..args.len() {
            new_env.define(self.func.borrow().params[i].clone(), args[i].clone());
        }

        for stmt in &self.func.borrow().body {
            match stmt.execute(interpreter, &mut new_env) {
                Ok(_) => {}
                Err(RuntimeException::Return { value, line: _line }) => {
                    return if self.func.borrow().is_initializer {
                        Ok(self.closure.get_at("this", 0).unwrap())
                    } else {
                        Ok(value)
                    };
                }
                Err(err) => return Err(err),
            }
        }

        if self.func.borrow().is_initializer {
            return Ok(self.closure.get_at("this", 0).unwrap());
        }
        Ok(RuntimeValue::Nil)
    }
}

impl Callable for ClassRef {
    fn call(
        self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        if let Some(init) = self.borrow().methods.get("init") {
            if args.len() != init.func.borrow().params.len() {
                return Err(RuntimeException::arity_error(
                    &format!("{}.init", self.borrow().name),
                    init.func.borrow().params.len(),
                    args.len(),
                ));
            }
            let instance = InstanceRef::new_instance(self.clone());
            init.clone()
                .bind(instance.clone())
                .call(args, interpreter, env)?;
            Ok(RuntimeValue::Instance(instance))
        } else {
            if args.len() != 0 {
                return Err(RuntimeException::arity_error(
                    &format!("{}.init", self.borrow().name),
                    0,
                    args.len(),
                ));
            }

            Ok(RuntimeValue::Instance(InstanceRef::new_instance(
                self.clone(),
            )))
        }
    }
}
