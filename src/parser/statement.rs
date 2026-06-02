use crate::parser::{ParseResult, Parser};
use crate::syntax_tree::statement::Stmt;
use crate::token::TokenType;

impl Parser {
    pub(in crate::parser) fn statement(&mut self) -> ParseResult<Stmt> {
        if self.match_token_type(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::expr(expr))
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after print expression")?;
        Ok(Stmt::print(expr))
    }
}
