use crate::environment::EnvRef;
use crate::syntax_tree::statement::Function;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Function { func: Rc<Function>, closure: EnvRef },
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Bool(false) => false,
            RuntimeValue::Nil => false,
            _ => true,
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Number(num) => write!(f, "{}", num),
            RuntimeValue::String(str) => write!(f, "{}", str),
            RuntimeValue::Bool(bool) => write!(f, "{}", bool),
            RuntimeValue::Function { func, .. } => write!(f, "{}", func),
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuntimeValue::Nil, RuntimeValue::Nil) => true,
            (RuntimeValue::Number(num1), RuntimeValue::Number(num2)) => num1 == num2,
            (RuntimeValue::String(str1), RuntimeValue::String(str2)) => str1 == str2,
            (RuntimeValue::Bool(bool1), RuntimeValue::Bool(bool2)) => bool1 == bool2,
            (RuntimeValue::Function { func: a, .. }, RuntimeValue::Function { func: b, .. }) => {
                Rc::ptr_eq(a, b)
            }
            _ => false,
        }
    }
}
