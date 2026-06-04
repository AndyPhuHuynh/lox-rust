use crate::parser::{ParseResult, Parser};
use crate::syntax_tree::expression::Expr;
use crate::syntax_tree::statement::Stmt;
use crate::token::{TokenKind, TokenType};

impl Parser {
    pub(in crate::parser) fn declaration(&mut self) -> ParseResult<Stmt> {
        if self.match_token_kind(&[TokenKind::Var]) {
            return self.var_declaration();
        }
        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(error) => {
                self.synchronize();
                Err(error)
            }
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenKind::Identifier, "Expect variable name")?;
        let mut initializer: Option<Expr> = None;

        if self.match_token_kind(&[TokenKind::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration",
        )?;
        Ok(Stmt::var(name.lexeme, initializer))
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.match_token_kind(&[TokenKind::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::expr(expr))
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after print expression")?;
        Ok(Stmt::print(expr))
    }
}
