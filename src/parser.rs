use crate::error::error_token;
use crate::syntax_tree::expressions::{BinaryOp, Expr, UnaryOp};
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }
}

impl Parser {
    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == *token_type
    }

    fn match_token_type(&mut self, types: &[TokenType]) -> bool {
        for token in types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        error_token(self.peek(), message);
        Err(ParseError)
    }

    fn synchronize(&mut self) {
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

impl Parser {
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let token_op = self.previous();
            let right = self.comparison()?;

            let binary_op = match token_op.r#type {
                TokenType::BangEqual => BinaryOp::NotEqual,
                TokenType::EqualEqual => BinaryOp::Equal,
                _ => {
                    error_token(token_op.clone(), format!("Unexpected equality operator: {:?}", token_op));
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, binary_op, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
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
                    error_token(token_op.clone(), format!("Unexpected comparison operator: {:?}", token_op));
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, binary_op, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token_type(&[TokenType::Plus, TokenType::Minus]) {
            let token_op = self.previous();
            let right = self.factor()?;

            let binary_op = match token_op.r#type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => {
                    error_token(token_op.clone(), format!("Unexpected term operator: {:?}", token_op));
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, binary_op, right);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token_type(&[TokenType::Star, TokenType::Slash]) {
            let token_op = self.previous();
            let right = self.unary()?;

            let binary_op = match token_op.r#type {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                _ => {
                    error_token(token_op.clone(), format!("Unexpected factor operator: {:?}", token_op));
                    return Err(ParseError);
                }
            };
            expr = Expr::binary(expr, binary_op, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            let token_op = self.previous();
            let right = self.unary()?;

            let unary_op = match token_op.r#type {
                TokenType::Bang => UnaryOp::LogicalNot,
                TokenType::Minus => UnaryOp::Negation,
                _ => {
                    error_token(token_op.clone(), format!("Unexpected unary operator: {:?}", token_op));
                    return Err(ParseError);
                }
            };
            return Ok(Expr::unary(unary_op, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();
        match token.r#type {
            TokenType::False => Ok(Expr::literal_false()),
            TokenType::True => Ok(Expr::literal_true()),
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
