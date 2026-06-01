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
