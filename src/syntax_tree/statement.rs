use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::{ClassRef, ClassRefExt, FunctionRef, RuntimeValue};
use crate::syntax_tree::expression::{Expr, Variable};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Class(ClassDecl),
    Expr(Expr),
    Function(FunctionDecl),
    If(If),
    Print(Print),
    Return(Return),
    While(While),
    Var(Var),
}

impl Stmt {
    pub fn block(stmts: Vec<Stmt>) -> Self {
        Self::Block(Block::new(stmts))
    }

    pub fn class(
        name: String,
        superclass: Option<Variable>,
        methods: Vec<FunctionDecl>,
        line: usize,
    ) -> Self {
        Self::Class(ClassDecl::new(name, superclass, methods, line))
    }

    pub fn expr(expr: Expr) -> Self {
        Self::Expr(expr)
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

#[derive(Debug, Clone)]
pub struct ClassDecl {
    pub name: String,
    pub superclass: Option<Variable>,
    pub methods: Vec<FunctionDecl>,
    pub line: usize,
}

impl ClassDecl {
    pub fn new(
        name: String,
        superclass: Option<Variable>,
        methods: Vec<FunctionDecl>,
        line: usize,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
            line,
        }
    }

    pub fn into_ref(
        self,
        interpreter: &mut Interpreter,
        closure: &mut EnvRef,
    ) -> RuntimeResult<ClassRef> {
        let superclass: Option<ClassRef> = self
            .superclass
            .as_ref()
            .map(|superclass| {
                let value = EnvRef::get_var(superclass, &interpreter.globals, closure)?;
                match value {
                    RuntimeValue::Class(class) => Ok(class),
                    _ => Err(RuntimeException::with_message(&format!(
                        "{} is not a class. Superclass must be a class",
                        superclass.name
                    ))
                    .at_line(superclass.line)),
                }
            })
            .transpose()?;

        let mut methods: HashMap<String, FunctionRef> = HashMap::new();
        for method in self.methods {
            let is_initializer = method.name == "init";
            methods.insert(
                method.name.clone(),
                method.into_ref(is_initializer, closure),
            );
        }
        Ok(ClassRef::new_class(
            self.name,
            superclass,
            methods,
            closure.clone(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<(String, usize)>,
    pub body: Vec<Stmt>,
    pub line: usize,
}

impl FunctionDecl {
    pub fn new(name: String, params: Vec<(String, usize)>, body: Vec<Stmt>, line: usize) -> Self {
        Self {
            name,
            params,
            body,
            line,
        }
    }

    pub fn into_ref(self, is_initializer: bool, closure: &mut EnvRef) -> FunctionRef {
        FunctionRef::new(
            self.name.clone(),
            self.params.into_iter().map(|(param, _)| param).collect(),
            self.body,
            is_initializer,
            closure.clone(),
        )
    }
}

impl Display for FunctionDecl {
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
