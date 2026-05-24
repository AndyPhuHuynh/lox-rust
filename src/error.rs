pub fn error(line: usize, message: impl AsRef<str>) {
    eprintln!("[Error at line {line}]: {}", message.as_ref());
}