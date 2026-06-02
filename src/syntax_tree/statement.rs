use crate::syntax_tree::expression::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Print),
}

impl Stmt {
    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }
    pub fn print(expr: Expr) -> Self {
        Self::Print(Print::new(expr))
    }
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expr: Expr,
}

impl Print {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}
