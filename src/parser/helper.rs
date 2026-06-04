use crate::error::error_token;
use crate::parser::{ParseError, Parser};
use crate::token::{Token, TokenKind, TokenType};

impl Parser {
    pub(in crate::parser) fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    pub(in crate::parser) fn check(&self, token_kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type.kind() == *token_kind
    }

    pub(in crate::parser) fn match_token_kind(&mut self, types: &[TokenKind]) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub(in crate::parser) fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub(in crate::parser) fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub(in crate::parser) fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    pub(in crate::parser) fn consume(
        &mut self,
        token_kind: TokenKind,
        message: &str,
    ) -> Result<Token, ParseError> {
        if self.check(&token_kind) {
            return Ok(self.advance());
        }
        error_token(self.peek(), message);
        Err(ParseError)
    }

    pub(in crate::parser) fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }

            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}
