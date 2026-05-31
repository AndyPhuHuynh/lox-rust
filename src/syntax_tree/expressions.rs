pub enum Expr {
    Literal(Literal),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
}

impl Expr {
    pub fn literal_num(num: f64) -> Expr {
        Expr::Literal(Literal::Number(num))
    }

    pub fn literal_str(str: &str) -> Expr {
        Expr::Literal(Literal::String(str.to_owned()))
    }

    pub fn literal_true() -> Expr {
        Expr::Literal(Literal::True)
    }

    pub fn literal_false() -> Expr {
        Expr::Literal(Literal::False)
    }

    pub fn literal_nil() -> Expr {
        Expr::Literal(Literal::Nil)
    }

    pub fn unary(operator: UnaryOp, expr: Expr) -> Expr {
        Expr::Unary(UnaryExpr::new(operator, expr))
    }

    pub fn binary(left: Expr, operator: BinaryOp, right: Expr) -> Expr {
        Expr::Binary(BinaryExpr::new(left, operator, right))
    }

    pub fn grouping(expression: Expr) -> Expr {
        Expr::Grouping(GroupingExpr::new(expression))
    }
}

impl Expr {
    pub fn print(&self) -> String {
        match &self {
            Expr::Literal(expr) => expr.print(),
            Expr::Unary(expr) => expr.print(),
            Expr::Binary(expr) => expr.print(),
            Expr::Grouping(expr) => expr.print(),
        }
    }
}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl Literal {
    pub fn print(&self) -> String {
        match &self {
            Literal::Number(num) => num.to_string(),
            Literal::String(str) => str.clone(),
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }
}

pub enum UnaryOp {
    LogicalNot,
    Negation,
}

impl UnaryOp {
    pub fn print(&self) -> String {
        match &self {
            UnaryOp::LogicalNot => "!".to_string(),
            UnaryOp::Negation => "-".to_string(),
        }
    }
}

pub struct UnaryExpr {
    operator: UnaryOp,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOp, expr: Expr) -> Self {
        Self {
            operator,
            expr: Box::new(expr),
        }
    }

    pub fn print(&self) -> String {
        format!("({} {})", self.operator.print(), self.expr.print())
    }
}

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

impl BinaryOp {
    pub fn print(&self) -> String {
        match &self {
            BinaryOp::Add => "+".to_string(),
            BinaryOp::Sub => "-".to_string(),
            BinaryOp::Mul => "*".to_string(),
            BinaryOp::Div => "/".to_string(),
            BinaryOp::Equal => "==".to_string(),
            BinaryOp::NotEqual => "!=".to_string(),
            BinaryOp::LessThan => "<".to_string(),
            BinaryOp::LessThanEqual => "<=".to_string(),
            BinaryOp::GreaterThan => ">".to_string(),
            BinaryOp::GreaterThanEqual => ">=".to_string(),
        }
    }
}

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: BinaryOp,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: BinaryOp, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn print(&self) -> String {
        format!(
            "({} {} {})",
            self.operator.print(),
            self.left.print(),
            self.right.print()
        )
    }
}

pub struct GroupingExpr {
    expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }

    pub fn print(&self) -> String {
        format!("(group {})", self.expression.print())
    }
}
