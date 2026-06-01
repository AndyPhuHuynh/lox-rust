use crate::token::{Token, TokenType};

pub fn error(line: usize, message: impl AsRef<str>) {
    eprintln!("[Error at line {line}]: {}", message.as_ref());
}

pub fn error_token(token: Token, message: impl AsRef<str>) {
    if token.r#type == TokenType::Eof {
        eprintln!(
            "[Error at line {line} (end of file)]: {msg}",
            msg = message.as_ref(),
            line = token.line
        );
    } else {
        eprintln!(
            "[Error at line {line}] with token [{lexeme}]: {msg}",
            line = token.line,
            lexeme = token.lexeme,
            msg = message.as_ref(),
        );
    }
}
