use crate::runtime::value::RuntimeValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Environment {
    enclosing: Option<EnvRef>,
    values: HashMap<String, RuntimeValue>,
}

impl Environment {
    fn with_enclosing(enclosing: Option<EnvRef>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
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

    fn assign(&mut self, name: String, value: RuntimeValue) -> Option<()> {
        match self.values.insert(name, value) {
            None => Some(()),
            Some(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvRef {
    env: Rc<RefCell<Environment>>,
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

    pub fn assign(&mut self, name: String, value: RuntimeValue) -> Option<()> {
        let mut current = Some(self.clone());
        while let Some(env_ref) = current {
            if env_ref.env.borrow_mut().is_defined(name.as_str()) {
                env_ref.env.borrow_mut().assign(name, value);
                return Some(());
            }
            current = env_ref.env.borrow().enclosing.clone()
        }
        None
    }
}
