use crate::syntax_tree::expression::Expr;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Function(Rc<RefCell<Function>>),
    If(If),
    Print(Print),
    Return(Return),
    While(While),
    Var(Var),
    Block(Block),
}

impl Stmt {
    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
    }

    pub fn function(name: String, params: Vec<String>, body: Vec<Stmt>, line: usize) -> Self {
        Stmt::Function(Rc::new(RefCell::new(Function::new(
            name, params, body, line,
        ))))
    }

    pub fn if_(cond: Expr, then: Stmt, else_: Option<Stmt>) -> Self {
        Self::If(If::new(cond, then, else_))
    }

    pub fn print(expr: Expr) -> Self {
        Self::Print(Print::new(expr))
    }

    pub fn return_(expr: Option<Expr>, line: usize) -> Self {
        Self::Return(Return::new(expr, line))
    }

    pub fn while_(cond: Expr, body: Stmt) -> Self {
        Self::While(While::new(cond, body))
    }

    pub fn var(str: String, initializer: Option<Expr>, line: usize) -> Self {
        Self::Var(Var::new(str, initializer, line))
    }

    pub fn block(stmts: Vec<Stmt>) -> Self {
        Self::Block(Block::new(stmts))
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub line: usize,
}

impl Function {
    pub fn new(name: String, params: Vec<String>, body: Vec<Stmt>, line: usize) -> Self {
        Self {
            name,
            params,
            body,
            line,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Self {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
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
pub struct Return {
    pub expr: Option<Expr>,
    pub line: usize,
}

impl Return {
    pub fn new(expr: Option<Expr>, line: usize) -> Self {
        Self { expr, line }
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub initializer: Option<Expr>,
    pub line: usize,
}

impl Var {
    pub fn new(name: String, initializer: Option<Expr>, line: usize) -> Self {
        Self {
            name,
            initializer,
            line,
        }
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
