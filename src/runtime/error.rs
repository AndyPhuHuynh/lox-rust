use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: Option<usize>,
}

impl RuntimeError {
    pub fn with_message(message: &str) -> RuntimeError {
        RuntimeError {
            message: message.to_string(),
            line: None,
        }
    }

    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            write!(f, "Runtime error at line {}: {}", line, self.message)
        } else {
            write!(f, "Runtime error: {}", self.message)
        }
    }
}
