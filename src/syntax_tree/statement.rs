use crate::syntax_tree::expression::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Print),
    Var(Var),
    Block(Block),
}

impl Stmt {
    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }

    pub fn print(expr: Expr) -> Self {
        Self::Print(Print::new(expr))
    }

    pub fn var(str: String, initializer: Option<Expr>) -> Self {
        Self::Var(Var::new(str, initializer))
    }

    pub fn block(stmts: Vec<Stmt>) -> Self {
        Self::Block(Block::new(stmts))
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

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub initializer: Option<Expr>,
}

impl Var {
    pub fn new(name: String, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}