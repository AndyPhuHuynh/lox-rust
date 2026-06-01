use crate::runtime::value::RuntimeValue;
use crate::runtime::{RuntimeResult, RuntimeResultExt};
use crate::syntax_tree::expressions::{
    BinaryExpr, BinaryOp, Expr, GroupingExpr, Literal, UnaryExpr, UnaryOp,
};

pub trait Interpret {
    fn interpret(&self) -> RuntimeResult;
}

impl Interpret for Expr {
    fn interpret(&self) -> RuntimeResult {
        match self {
            Expr::Literal(expr) => expr.interpret(),
            Expr::Unary(expr) => expr.interpret(),
            Expr::Binary(expr) => expr.interpret(),
            Expr::Grouping(expr) => expr.interpret(),
        }
    }
}

impl Interpret for Literal {
    fn interpret(&self) -> RuntimeResult {
        match &self {
            Literal::Number(num) => Ok(RuntimeValue::Number(*num)),
            Literal::String(str) => Ok(RuntimeValue::String(str.clone())),
            Literal::Bool(bool) => Ok(RuntimeValue::Bool(*bool)),
            Literal::Nil => Ok(RuntimeValue::Nil),
        }
    }
}

impl Interpret for UnaryExpr {
    fn interpret(&self) -> RuntimeResult {
        let right = self.expr.interpret()?;

        match self.op_token.operator {
            UnaryOp::LogicalNot => Ok(right.logical_not()),
            UnaryOp::Negation => right.negation().at_line(self.op_token.line),
        }
    }
}

impl Interpret for BinaryExpr {
    fn interpret(&self) -> RuntimeResult {
        let left = self.left.interpret()?;
        let right = self.right.interpret()?;

        match self.op_token.operator {
            BinaryOp::Add => left.add(right),
            BinaryOp::Sub => left.sub(right),
            BinaryOp::Mul => left.mul(right),
            BinaryOp::Div => left.div(right),
            BinaryOp::Equal => left.equal(right),
            BinaryOp::NotEqual => left.not_equal(right),
            BinaryOp::LessThan => left.less_than(right),
            BinaryOp::LessThanEqual => left.less_than_or_equal(right),
            BinaryOp::GreaterThan => left.greater_than(right),
            BinaryOp::GreaterThanEqual => left.greater_than_or_equal(right),
        }
        .at_line(self.op_token.line)
    }
}

impl Interpret for GroupingExpr {
    fn interpret(&self) -> RuntimeResult {
        self.expression.interpret()
    }
}
