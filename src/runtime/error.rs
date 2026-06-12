use crate::runtime::value::RuntimeValue;

#[derive(Debug)]
pub enum RuntimeException {
    RuntimeError {
        message: String,
        line: Option<usize>,
    },
    Return {
        value: RuntimeValue,
        line: Option<usize>,
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
        Self::Return {
            value,
            line: Some(line),
        }
    }

    pub fn at_line(mut self, line_num: usize) -> Self {
        match &mut self {
            Self::RuntimeError {
                message: _message,
                line,
            } => match line {
                None => *line = Some(line_num),
                Some(_) => {}
            },
            Self::Return {
                value: _value,
                line,
            } => match line {
                None => *line = Some(line_num),
                Some(_) => {}
            },
        }
        self
    }
}

pub fn redefinition_error(symbol: &str, line: usize) -> RuntimeException {
    RuntimeException::with_message(
        format!(
            "Attempting to redefine symbol '{}' which has already been previously defined",
            symbol
        )
        .as_str(),
    )
    .at_line(line)
}
