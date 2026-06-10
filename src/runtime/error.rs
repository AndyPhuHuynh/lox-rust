use crate::runtime::value::RuntimeValue;

#[derive(Debug)]
pub enum RuntimeException {
    RuntimeError {
        message: String,
        line: Option<usize>,
    },
    Return {
        value: RuntimeValue,
        line: usize,
    },
}

impl RuntimeException {
    pub fn with_message(message: &str) -> Self {
        Self::RuntimeError {
            message: message.to_string(),
            line: None,
        }
    }

    pub fn return_value(value: RuntimeValue, line: usize) -> Self {
        Self::Return { value, line }
    }

    pub fn at_line(mut self, line_num: usize) -> Self {
        match &mut self {
            Self::RuntimeError {
                message: _message,
                line,
            } => {
                *line = Some(line_num);
            }
            Self::Return {
                value: _value,
                line,
            } => {
                *line = line_num;
            }
        }
        self
    }
}
