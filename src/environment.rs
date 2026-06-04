use crate::runtime::value::RuntimeValue;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new() -> Environment {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: RuntimeValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&RuntimeValue> {
        self.values.get(name)
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
