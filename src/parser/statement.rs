use crate::parser::{ParseResult, Parser};
use crate::syntax_tree::expression::Expr;
use crate::syntax_tree::statement::Stmt;
use crate::token::TokenKind;

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
        if self.match_token_kind(&[TokenKind::For]) {
            return self.for_statement();
        }
        if self.match_token_kind(&[TokenKind::If]) {
            return self.if_statement();
        }
        if self.match_token_kind(&[TokenKind::Print]) {
            return self.print_statement();
        }
        if self.match_token_kind(&[TokenKind::While]) {
            return self.while_statement();
        }
        if self.match_token_kind(&[TokenKind::LeftBrace]) {
            return Ok(Stmt::block(self.block()?));
        }
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::expr(expr))
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'for'")?;

        let init: Option<Stmt> = if self.match_token_kind(&[TokenKind::Semicolon]) {
            None
        } else if self.match_token_kind(&[TokenKind::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition: Option<Expr> = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenKind::Semicolon, "Expect ';' after loop condition")?;

        let increment: Option<Expr> = if self.check(&TokenKind::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenKind::RightParen, "Expect ')' after for loop condition")?;

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::block(vec!(body, Stmt::Expr(increment)))
        }

        let condition = condition.unwrap_or_else(|| Expr::literal_bool(true));
        body = Stmt::r#while(condition, body);

        if let Some(init) = init {
            body = Stmt::block(vec![init, body])
        }

        println!("{:#?}", body);

        Ok(body)
    }

    fn if_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, "Expect ')' after 'if'")?;

        let then_branch = self.statement()?;
        let mut else_branch: Option<Stmt> = None;
        if self.match_token_kind(&[TokenKind::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Stmt::r#if(condition, then_branch, else_branch))
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenKind::Semicolon, "Expect ';' after print expression")?;
        Ok(Stmt::print(expr))
    }

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenKind::LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenKind::RightParen, "Expect ')' after 'while'")?;
        let body = self.statement()?;
        Ok(Stmt::r#while(condition, body))
    }

    fn block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        self.consume(TokenKind::RightBrace, "Expect '}' after block")?;
        Ok(stmts)
    }
}
