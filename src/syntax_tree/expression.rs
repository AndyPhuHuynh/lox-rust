use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Array(ArrayExpr),
    ArrayAccess(ArrayAccessExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Logical(LogicalExpr),
    Call(Call),
    Get(Get),
    Set(Set),
    Super(Super),
    This(Variable),
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

    pub fn array(elements: Vec<Expr>) -> Expr {
        Expr::Array(ArrayExpr::new(elements))
    }

    pub fn array_access(array: Expr, index: Expr, line: usize) -> Expr {
        Expr::ArrayAccess(ArrayAccessExpr::new(array, index, line))
    }

    pub fn unary(operator: UnaryOpToken, expr: Expr) -> Expr {
        Expr::Unary(UnaryExpr::new(operator, expr))
    }

    pub fn binary(left: Expr, operator: BinaryOpToken, right: Expr) -> Expr {
        Expr::Binary(BinaryExpr::new(left, operator, right))
    }

    pub fn logical(left: Expr, operator: LogicalOp, right: Expr) -> Expr {
        Expr::Logical(LogicalExpr::new(left, operator, right))
    }

    pub fn get(expr: Expr, name: String, line: usize) -> Expr {
        Expr::Get(Get::new(expr, name, line))
    }

    pub fn set(expr: Expr, name: String, value: Expr, line: usize) -> Expr {
        Expr::Set(Set::new(expr, name, value, line))
    }

    pub fn super_(super_: Variable, method: String) -> Expr {
        Expr::Super(Super::new(super_, method))
    }

    pub fn this(name: String, line: usize) -> Expr {
        Expr::This(Variable::new(name, line))
    }

    pub fn grouping(expression: Expr) -> Expr {
        Expr::Grouping(GroupingExpr::new(expression))
    }

    pub fn call(callee: Expr, arguments: Vec<Expr>, line: usize) -> Expr {
        Expr::Call(Call::new(callee, arguments, line))
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
            Expr::Array(expr) => write!(f, "{}", expr),
            Expr::ArrayAccess(expr) => write!(f, "{}", expr),
            Expr::Unary(expr) => write!(f, "{expr}"),
            Expr::Binary(expr) => write!(f, "{expr}"),
            Expr::Logical(expr) => write!(f, "{expr}"),
            Expr::Call(expr) => write!(f, "{expr}"),
            Expr::Get(expr) => write!(f, "{expr}"),
            Expr::Set(expr) => write!(f, "{expr}"),
            Expr::Super(expr) => write!(f, "{expr}"),
            Expr::This(_) => write!(f, "(this)"),
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
pub struct ArrayExpr {
    pub elements: Vec<Expr>,
}

impl ArrayExpr {
    pub fn new(elements: Vec<Expr>) -> ArrayExpr {
        ArrayExpr { elements }
    }
}

impl Display for ArrayExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(array ")?;
        if !self.elements.is_empty() {
            write!(f, "{}", self.elements[0])?;
        }
        for i in 1..self.elements.len() {
            write!(f, ", {}", self.elements[i])?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub struct ArrayAccessExpr {
    pub array: Box<Expr>,
    pub index: Box<Expr>,
    pub line: usize,
}

impl ArrayAccessExpr {
    pub fn new(array: Expr, index: Expr, line: usize) -> ArrayAccessExpr {
        ArrayAccessExpr {
            array: Box::new(array),
            index: Box::new(index),
            line
        }
    }
}

impl Display for ArrayAccessExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(index {} {})", self.array, self.index)
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
    pub fn new(operator: BinaryOp, line: usize) -> Self {
        Self { operator, line }
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
pub enum LogicalOp {
    Or,
    And,
}

impl Display for LogicalOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            LogicalOp::Or => write!(f, "or"),
            LogicalOp::And => write!(f, "and"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub op: LogicalOp,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Expr, operator: LogicalOp, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op: operator,
            right: Box::new(right),
        }
    }
}

impl Display for LogicalExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right,)
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub line: usize,
}

impl Call {
    pub fn new(callee: Expr, args: Vec<Expr>, line: usize) -> Self {
        Self {
            callee: Box::new(callee),
            args,
            line,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(call {}", self.callee)?;
        for arg in &self.args {
            write!(f, " {}", arg)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone)]
pub struct Get {
    pub expr: Box<Expr>,
    pub name: String,
    pub line: usize,
}

impl Get {
    pub fn new(expr: Expr, name: String, line: usize) -> Self {
        Self {
            expr: Box::new(expr),
            name,
            line,
        }
    }
}

impl Display for Get {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(get {}, {})", self.expr, self.line)
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: String,
    pub value: Box<Expr>,
    pub line: usize,
}

impl Set {
    pub fn new(expr: Expr, name: String, value: Expr, line: usize) -> Self {
        Self {
            object: Box::new(expr),
            name,
            value: Box::new(value),
            line,
        }
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(set {}.{}, {})", self.object, self.name, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Super {
    pub super_: Variable,
    pub method: String,
}

impl Super {
    pub fn new(super_: Variable, method: String) -> Self {
        Self { super_, method }
    }
}

impl Display for Super {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(super {})", self.method)
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
    pub local_distance: Option<usize>,
}

impl Variable {
    pub fn new(name: String, line: usize) -> Self {
        Self {
            name,
            line,
            local_distance: None,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(variable {})", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum AssignmentTarget {
    Variable(Variable),
}

impl Display for AssignmentTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentTarget::Variable(name) => write!(f, "{}", name),
        }
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
