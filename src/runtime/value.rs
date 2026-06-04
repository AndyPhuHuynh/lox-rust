use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Object,
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
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
            RuntimeValue::Object => todo!("Implement objects"),
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Number(num) => write!(f, "{}", num),
            RuntimeValue::String(str) => write!(f, "{}", str),
            RuntimeValue::Bool(bool) => write!(f, "{}", bool),
        }
    }
}
