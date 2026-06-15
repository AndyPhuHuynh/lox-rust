use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::interpreter::statement::Execute;
use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::{ClassRef, FunctionRef, InstanceRef, InstanceRefExt, RuntimeValue};

pub trait Callable {
    fn name(&self) -> String;
    fn arity(&self) -> usize;
    fn call(
        &self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue>;

    fn check_arity(&self, args_len: usize) -> RuntimeResult<()> {
        if args_len != self.arity() {
            return Err(RuntimeException::arity_error(
                &self.name(),
                self.arity(),
                args_len,
            ));
        }
        Ok(())
    }
}

impl Callable for FunctionRef {
    fn name(&self) -> String {
        self.func.borrow().name.clone()
    }

    fn arity(&self) -> usize {
        self.func.borrow().params.len()
    }

    fn call(
        &self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        _: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
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
    fn name(&self) -> String {
        format!("{}.init", self.borrow().name)
    }

    fn arity(&self) -> usize {
        if let Some(init) = self.borrow().get_method("init") {
            init.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        args: &[RuntimeValue],
        interpreter: &mut Interpreter,
        _: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        self.check_arity(args.len())?;
        if let Some(init) = self.borrow().get_method("init") {
            let instance = InstanceRef::new_instance(self.clone());
            init.clone().bind(instance.clone()).call(
                args,
                interpreter,
                &mut self.borrow().closure.clone(),
            )?;
            Ok(RuntimeValue::Instance(instance))
        } else {
            Ok(RuntimeValue::Instance(InstanceRef::new_instance(
                self.clone(),
            )))
        }
    }
}
