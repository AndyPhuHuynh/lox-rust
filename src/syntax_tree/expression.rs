use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Variable(Variable),
    Assignment(Assignment),
}

impl Expr {
    pub fn literal_num(num: f64) -> Expr {
        Expr::Literal(Literal::Number(num))
    }

    pub fn literal_str(str: &str) -> Expr {
        Expr::Literal(Literal::String(str.to_owned()))
    }

    pub fn literal_bool(bool: bool) -> Expr {
        Expr::Literal(Literal::Bool(bool))
    }

    pub fn literal_nil() -> Expr {
        Expr::Literal(Literal::Nil)
    }

    pub fn unary(operator: UnaryOpToken, expr: Expr) -> Expr {
        Expr::Unary(UnaryExpr::new(operator, expr))
    }

    pub fn binary(left: Expr, operator: BinaryOpToken, right: Expr) -> Expr {
        Expr::Binary(BinaryExpr::new(left, operator, right))
    }

    pub fn grouping(expression: Expr) -> Expr {
        Expr::Grouping(GroupingExpr::new(expression))
    }

    pub fn variable(name: String, line: usize) -> Expr {
        Expr::Variable(Variable::new(name, line))
    }

    pub fn assignment(target: AssignmentTarget, expr: Expr) -> Expr {
        Expr::Assignment(Assignment::new(target, expr))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Expr::Literal(expr) => write!(f, "{expr}"),
            Expr::Unary(expr) => write!(f, "{expr}"),
            Expr::Binary(expr) => write!(f, "{expr}"),
            Expr::Grouping(expr) => write!(f, "{expr}"),
            Expr::Variable(expr) => write!(f, "{expr}"),
            Expr::Assignment(expr) => write!(f, "{expr}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Number(num) => write!(f, "{num}"),
            Literal::String(str) => write!(f, "{str}"),
            Literal::Bool(bool) => write!(f, "{bool}"),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    LogicalNot,
    Negation,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            UnaryOp::LogicalNot => write!(f, "!"),
            UnaryOp::Negation => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryOpToken {
    pub operator: UnaryOp,
    pub line: usize,
}

impl UnaryOpToken {
    pub fn new(operator: UnaryOp, line: usize) -> UnaryOpToken {
        UnaryOpToken { operator, line }
    }
}

impl Display for UnaryOpToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.operator.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op_token: UnaryOpToken,
    pub expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOpToken, expr: Expr) -> Self {
        Self {
            op_token: operator,
            expr: Box::new(expr),
        }
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.op_token, self.expr)
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Equal => write!(f, "=="),
            BinaryOp::NotEqual => write!(f, "!="),
            BinaryOp::LessThan => write!(f, "<"),
            BinaryOp::LessThanEqual => write!(f, "<="),
            BinaryOp::GreaterThan => write!(f, ">"),
            BinaryOp::GreaterThanEqual => write!(f, ">="),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOpToken {
    pub operator: BinaryOp,
    pub line: usize,
}

impl BinaryOpToken {
    pub fn new(operator: BinaryOp, line: usize) -> BinaryOpToken {
        BinaryOpToken { operator, line }
    }
}

impl Display for BinaryOpToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.operator.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op_token: BinaryOpToken,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: BinaryOpToken, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op_token: operator,
            right: Box::new(right),
        }
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op_token, self.left, self.right,)
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

impl Display for GroupingExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub line: usize,
}

impl Variable {
    pub fn new(name: String, line: usize) -> Self {
        Self { name, line }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(variable {})", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentTargetType {
    Variable(String),
}

impl Display for AssignmentTargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentTargetType::Variable(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentTarget {
    pub r#type: AssignmentTargetType,
    pub line: usize,
}

impl AssignmentTarget {
    pub fn new(r#type: AssignmentTargetType, line: usize) -> Self {
        Self { r#type, line }
    }
}

impl Display for AssignmentTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub target: AssignmentTarget,
    pub value: Box<Expr>,
}

impl Assignment {
    pub fn new(target: AssignmentTarget, value: Expr) -> Self {
        Self {
            target,
            value: Box::new(value),
        }
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(assign {}, {})", self.target, self.value)
    }
}
