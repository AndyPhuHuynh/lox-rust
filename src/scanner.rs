use std::ascii::AsciiExt;

use crate::{error::error, token::{Token, TokenType}};

pub fn scan_tokens(_source: &str) {
    todo!("scan_tokens");
}

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn is_at_end(&mut self) -> bool {
        self.current > self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, r#type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let lexeme = std::str::from_utf8(lexeme).expect("Unexpected byte sequence while scanning");
        self.tokens.push(Token::new(r#type, lexeme.to_string(), self.line));
    } 

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            _ => {
                error(self.line, format!("Unexpected character {}", c.to_ascii_lowercase()));
            }
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source.as_bytes(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        self.tokens.clear();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line));
        &self.tokens
    }
}
