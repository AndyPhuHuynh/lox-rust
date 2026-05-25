use crate::{
    error::error,
    token::{Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.source[self.current] != expected {
            return false;
        };

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        return self.source[self.current];
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        return self.source[self.current + 1];
    }

    fn add_token(&mut self, r#type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let lexeme = std::str::from_utf8(lexeme).expect("Unexpected byte sequence while scanning");
        self.tokens
            .push(Token::new(r#type, lexeme.to_string(), self.line));
    }

    fn add_token_conditionally(
        &mut self,
        expected: u8,
        match_token: TokenType,
        not_match_token: TokenType,
    ) {
        let token = if self.match_char(expected) {
            match_token
        } else {
            not_match_token
        };

        self.add_token(token);
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
            return;
        }

        // The close quotation mark
        self.advance();

        let value = std::str::from_utf8(&self.source[self.start + 1..self.current - 1])
            .expect("Unexpected byte sequence while scanning")
            .to_string();

        self.add_token(TokenType::String(value));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // Consume the dot
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let str = std::str::from_utf8(&self.source[self.start..self.current])
            .expect("Unexpected byte sequence while scanning");
        let num: f64 = str.parse().unwrap();

        self.add_token(TokenType::Number(num));
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
            b'!' => self.add_token_conditionally(b'=', TokenType::BangEqual, TokenType::Bang),
            b'=' => self.add_token_conditionally(b'=', TokenType::EqualEqual, TokenType::Equal),
            b'<' => self.add_token_conditionally(b'=', TokenType::LessEqual, TokenType::Less),
            b'>' => self.add_token_conditionally(b'=', TokenType::GreaterEqual, TokenType::Greater),
            b'/' => {
                // Handle comments
                if self.match_char(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(),
            b'0'..=b'9' => self.number(),
            _ => {
                error(self.line, format!("Unexpected byte 0x{:x}", c));
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
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        self.tokens.clear();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
        &self.tokens
    }
}
