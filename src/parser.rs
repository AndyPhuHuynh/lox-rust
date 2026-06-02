mod expression;
mod helper;
mod statement;

use crate::syntax_tree::statement::Stmt;
use crate::token::Token;

#[derive(Debug)]
pub struct ParseError;

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut result: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            result.push(self.statement()?);
        }
        Ok(result)
    }
}
