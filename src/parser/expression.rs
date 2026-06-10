use crate::error::error_token;
use crate::parser::{ParseError, ParseResult, Parser};
use crate::syntax_tree::expression::{
    AssignmentTarget, AssignmentTargetType, BinaryOp, BinaryOpToken, Expr, LogicalOp, UnaryOp, UnaryOpToken,
};
use crate::token::{TokenKind, TokenType};

impl Parser {
    pub(in crate::parser) fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.logical_or()?;

        if self.match_token_kind(&[TokenKind::Equal]) {
            let equal = self.previous();
            let value = self.assignment()?;

            return match expr {
                Expr::Variable(var) => {
                    Ok(Expr::assignment(
                        AssignmentTarget::new(AssignmentTargetType::Variable(var.name), equal.line),
                        value,
                    ))
                }
                _ => {
                    error_token(equal, "Invalid assignment target");
                    Err(ParseError)
                }
            }
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.logical_and()?;

        while self.match_token_kind(&[TokenKind::Or]) {
            let token_op = self.previous();
            let right = self.logical_or()?;

            let logical_op = match token_op.r#type {
                TokenType::Or => LogicalOp::Or,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected logical or operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            expr = Expr::logical(expr, logical_op, right);
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;

        while self.match_token_kind(&[TokenKind::And]) {
            let token_op = self.previous();
            let right = self.logical_and()?;

            let logical_op = match token_op.r#type {
                TokenType::And => LogicalOp::And,
                _ => {
                    error_token(
                        token_op.clone(),
                        format!("Unexpected logical and operator: {:?}", token_op),
                    );
                    return Err(ParseError);
                }
            };
            expr = Expr::logical(expr, logical_op, right);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;

        while self.match_token_kind(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
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

        while self.match_token_kind(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
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

        while self.match_token_kind(&[TokenKind::Plus, TokenKind::Minus]) {
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

        while self.match_token_kind(&[TokenKind::Star, TokenKind::Slash]) {
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
        if self.match_token_kind(&[TokenKind::Bang, TokenKind::Minus]) {
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
                self.consume(TokenKind::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::grouping(expr))
            }
            TokenType::Identifier(name) => Ok(Expr::variable(name, token.line)),
            _ => {
                error_token(token, "Expect expression.");
                Err(ParseError)
            }
        }
    }
}
