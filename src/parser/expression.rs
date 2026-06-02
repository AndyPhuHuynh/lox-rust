use crate::error::error_token;
use crate::parser::{ParseError, ParseResult, Parser};
use crate::syntax_tree::expression::{BinaryOp, BinaryOpToken, Expr, UnaryOp, UnaryOpToken};
use crate::token::TokenType;

impl Parser {
    pub(in crate::parser) fn expression(&mut self) -> ParseResult<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let token_op = self.previous();
            let right = self.comparison()?;

            let binary_op = match token_op.r#type {
                TokenType::BangEqual => BinaryOp::NotEqual,
                TokenType::EqualEqual => BinaryOp::Equal,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected equality operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, BinaryOpToken::new(binary_op, token_op.line), right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        while self.match_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let token_op = self.previous();
            let right = self.term()?;

            let binary_op = match token_op.r#type {
                TokenType::Greater => BinaryOp::GreaterThan,
                TokenType::GreaterEqual => BinaryOp::GreaterThanEqual,
                TokenType::Less => BinaryOp::LessThan,
                TokenType::LessEqual => BinaryOp::LessThanEqual,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected comparison operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, BinaryOpToken::new(binary_op, token_op.line), right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let token_op = self.previous();
            let right = self.factor()?;

            let binary_op = match token_op.r#type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected term operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, BinaryOpToken::new(binary_op, token_op.line), right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while self.match_token_type(&[TokenType::Star, TokenType::Slash]) {
            let token_op = self.previous();
            let right = self.unary()?;

            let binary_op = match token_op.r#type {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected factor operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, BinaryOpToken::new(binary_op, token_op.line), right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let token_op = self.previous();
            let right = self.unary()?;

            let unary_op = match token_op.r#type {
                TokenType::Bang => UnaryOp::LogicalNot,
                TokenType::Minus => UnaryOp::Negation,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected unary operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            return Ok(Expr::unary(
                UnaryOpToken::new(unary_op, token_op.line),
                right,
            ));
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        let token = self.advance();
        match token.r#type {
            TokenType::False => Ok(Expr::literal_bool(false)),
            TokenType::True => Ok(Expr::literal_bool(true)),
            TokenType::Nil => Ok(Expr::literal_nil()),
            TokenType::Number(num) => Ok(Expr::literal_num(num)),
            TokenType::String(str) => Ok(Expr::literal_str(str.as_str())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::grouping(expr))
            }
            _ => {
                error_token(token, "Expect expression.");
                Err(ParseError)
            }
        }
    }
}
