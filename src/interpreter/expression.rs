use crate::runtime::value::RuntimeValue;
use crate::runtime::{RuntimeResult, RuntimeResultExt};
use crate::syntax_tree::expression::{
    BinaryExpr, BinaryOp, Expr, GroupingExpr, Literal, UnaryExpr, UnaryOp,
};

pub trait Evaluate {
    fn evaluate(&self) -> RuntimeResult<RuntimeValue>;
}

impl Evaluate for Expr {
    fn evaluate(&self) -> RuntimeResult<RuntimeValue> {
        match self {
            Expr::Literal(expr) => expr.evaluate(),
            Expr::Unary(expr) => expr.evaluate(),
            Expr::Binary(expr) => expr.evaluate(),
            Expr::Grouping(expr) => expr.evaluate(),
        }
    }
}

impl Evaluate for Literal {
    fn evaluate(&self) -> RuntimeResult<RuntimeValue> {
        match &self {
            Literal::Number(num) => Ok(RuntimeValue::Number(*num)),
            Literal::String(str) => Ok(RuntimeValue::String(str.clone())),
            Literal::Bool(bool) => Ok(RuntimeValue::Bool(*bool)),
            Literal::Nil => Ok(RuntimeValue::Nil),
        }
    }
}

impl Evaluate for UnaryExpr {
    fn evaluate(&self) -> RuntimeResult<RuntimeValue> {
        let right = self.expr.evaluate()?;

        match self.op_token.operator {
            UnaryOp::LogicalNot => Ok(right.logical_not()),
            UnaryOp::Negation => right.negation().at_line(self.op_token.line),
        }
    }
}

impl Evaluate for BinaryExpr {
    fn evaluate(&self) -> RuntimeResult<RuntimeValue> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

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

impl Evaluate for GroupingExpr {
    fn evaluate(&self) -> RuntimeResult<RuntimeValue> {
        self.expression.evaluate()
    }
}
