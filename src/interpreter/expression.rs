use crate::environment::Environment;
use crate::runtime::error::RuntimeError;
use crate::runtime::value::RuntimeValue;
use crate::runtime::{RuntimeResult, RuntimeResultExt};
use crate::syntax_tree::expression::{
    Assignment, AssignmentTargetType, BinaryExpr, BinaryOp, Expr, GroupingExpr, Literal, UnaryExpr,
    UnaryOp, Variable,
};

pub trait Evaluate {
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue>;
}

impl Evaluate for Expr {
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue> {
        match self {
            Expr::Literal(expr) => expr.evaluate(env),
            Expr::Unary(expr) => expr.evaluate(env),
            Expr::Binary(expr) => expr.evaluate(env),
            Expr::Grouping(expr) => expr.evaluate(env),
            Expr::Variable(expr) => expr.evaluate(env),
            Expr::Assignment(expr) => expr.evaluate(env),
        }
    }
}

impl Evaluate for Literal {
    fn evaluate(&self, _: &mut Environment) -> RuntimeResult<RuntimeValue> {
        match &self {
            Literal::Number(num) => Ok(RuntimeValue::Number(*num)),
            Literal::String(str) => Ok(RuntimeValue::String(str.clone())),
            Literal::Bool(bool) => Ok(RuntimeValue::Bool(*bool)),
            Literal::Nil => Ok(RuntimeValue::Nil),
        }
    }
}

impl Evaluate for UnaryExpr {
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue> {
        let right = self.expr.evaluate(env)?;

        match self.op_token.operator {
            UnaryOp::LogicalNot => Ok(right.logical_not()),
            UnaryOp::Negation => right.negation().at_line(self.op_token.line),
        }
    }
}

impl Evaluate for BinaryExpr {
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue> {
        let left = self.left.evaluate(env)?;
        let right = self.right.evaluate(env)?;

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
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue> {
        self.expression.evaluate(env)
    }
}

impl Evaluate for Variable {
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue> {
        env.get(self.name.as_str())
            .cloned()
            .ok_or(RuntimeError::with_message(
                format!("Undefined variable: {}", self.name).as_str(),
            ))
    }
}

impl Evaluate for Assignment {
    fn evaluate(&self, env: &mut Environment) -> RuntimeResult<RuntimeValue> {
        match &self.target.r#type {
            AssignmentTargetType::Variable(name) => {
                if !env.is_defined(name.as_str()) {
                    return Err(RuntimeError::with_message(
                        format!("Undefined variable: {}", name).as_str(),
                    ));
                }
                let rhs_value = self.value.evaluate(env)?;
                env.define(name.clone(), rhs_value.clone());
                Ok(rhs_value)
            }
        }
    }
}
