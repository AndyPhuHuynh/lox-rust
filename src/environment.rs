use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::RuntimeValue;
use crate::syntax_tree::expression::Variable;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Environment {
    values: HashMap<String, RuntimeValue>,
    enclosing: Option<EnvRef>,
}

impl Environment {
    fn with_enclosing(enclosing: Option<EnvRef>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    fn define(&mut self, name: String, value: RuntimeValue) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<RuntimeValue> {
        self.values.get(name).cloned()
    }

    fn assign(&mut self, name: String, value: RuntimeValue) -> Option<RuntimeValue> {
        self.values.insert(name, value)
    }
}

#[derive(Debug, Clone)]
pub struct EnvRef {
    env: Rc<RefCell<Environment>>,
}

impl PartialEq for EnvRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.env, &other.env)
    }
}

impl EnvRef {
    pub fn with_enclosing(enclosing: Option<EnvRef>) -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::with_enclosing(enclosing))),
        }
    }

    pub fn define(&self, name: String, value: RuntimeValue) -> Option<()> {
        if self.env.borrow().is_defined(name.as_str()) {
            return None;
        }
        self.env.borrow_mut().define(name, value);
        Some(())
    }

    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        let mut current = Some(self.clone());
        while let Some(env_ref) = current {
            if let Some(val) = env_ref.env.borrow().get(name) {
                return Some(val);
            }
            current = env_ref.env.borrow().enclosing.clone()
        }
        None
    }

    pub fn get_at(&self, name: &str, mut distance: usize) -> Option<RuntimeValue> {
        let mut env_ref = self.clone();
        while distance > 0 {
            let next = env_ref.env.borrow().enclosing.clone()?;
            env_ref = next;
            distance -= 1;
        }
        env_ref.env.borrow().get(name)
    }

    pub fn get_var(
        var: &Variable,
        global_env: &EnvRef,
        current_env: &EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        match var.local_distance {
            None => global_env.get(var.name.as_str()),
            Some(distance) => current_env.get_at(var.name.as_str(), distance),
        }
        .ok_or(RuntimeException::undefined_variable(
            var.name.clone(),
            var.line,
        ))
    }

    pub fn assign(&mut self, name: String, value: RuntimeValue) -> Option<RuntimeValue> {
        let mut current = Some(self.clone());
        while let Some(env_ref) = current {
            if env_ref.env.borrow_mut().is_defined(name.as_str()) {
                return Some(env_ref.env.borrow_mut().assign(name, value)?);
            }
            current = env_ref.env.borrow().enclosing.clone()
        }
        None
    }

    pub fn assign_at(
        &mut self,
        name: String,
        value: RuntimeValue,
        mut distance: usize,
    ) -> Option<RuntimeValue> {
        let mut env_ref = self.clone();
        while distance > 0 {
            let next = env_ref.env.borrow().enclosing.clone()?;
            env_ref = next;
            distance -= 1;
        }

        env_ref.env.borrow_mut().assign(name, value)
    }

    pub fn assign_var(
        var: &Variable,
        rhs_value: RuntimeValue,
        global_env: &mut EnvRef,
        current_env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        match var.local_distance {
            None => global_env.assign(var.name.clone(), rhs_value.clone()),
            Some(distance) => current_env.assign_at(var.name.clone(), rhs_value.clone(), distance),
        }
        .ok_or(RuntimeException::undefined_variable(
            var.name.clone(),
            var.line,
        ))
    }
}
