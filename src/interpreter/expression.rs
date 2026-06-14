use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::runtime::call::Callable;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::{InstanceRefExt, RuntimeValue};
use crate::runtime::{RuntimeResult, RuntimeResultExt};
use crate::syntax_tree::expression::{
    Assignment, AssignmentTarget, BinaryExpr, BinaryOp, Call, Expr, Get, GroupingExpr, Literal,
    LogicalExpr, LogicalOp, Set, UnaryExpr, UnaryOp, Variable,
};

pub trait Evaluate {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue>;
}

impl Evaluate for Expr {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        match self {
            Expr::Literal(expr) => expr.evaluate(interpreter, env),
            Expr::Unary(expr) => expr.evaluate(interpreter, env),
            Expr::Binary(expr) => expr.evaluate(interpreter, env),
            Expr::Logical(expr) => expr.evaluate(interpreter, env),
            Expr::Call(expr) => expr.evaluate(interpreter, env),
            Expr::Get(expr) => expr.evaluate(interpreter, env),
            Expr::Set(expr) => expr.evaluate(interpreter, env),
            Expr::This(expr) => expr.evaluate(interpreter, env),
            Expr::Grouping(expr) => expr.evaluate(interpreter, env),
            Expr::Variable(expr) => expr.evaluate(interpreter, env),
            Expr::Assignment(expr) => expr.evaluate(interpreter, env),
        }
    }
}

impl Evaluate for Literal {
    fn evaluate(&self, _: &mut Interpreter, _: &mut EnvRef) -> RuntimeResult<RuntimeValue> {
        match &self {
            Literal::Number(num) => Ok(RuntimeValue::Number(*num)),
            Literal::String(str) => Ok(RuntimeValue::String(str.clone())),
            Literal::Bool(bool) => Ok(RuntimeValue::Bool(*bool)),
            Literal::Nil => Ok(RuntimeValue::Nil),
        }
    }
}

impl Evaluate for UnaryExpr {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        let right = self.expr.evaluate(interpreter, env)?;

        match self.op_token.operator {
            UnaryOp::LogicalNot => Ok(right.logical_not()),
            UnaryOp::Negation => right.negation().at_line(self.op_token.line),
        }
    }
}

impl Evaluate for BinaryExpr {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        let left = self.left.evaluate(interpreter, env)?;
        let right = self.right.evaluate(interpreter, env)?;

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

impl Evaluate for LogicalExpr {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        let left = self.left.evaluate(interpreter, env)?;

        match self.op {
            LogicalOp::Or if left.is_truthy() => Ok(left),
            LogicalOp::And if !left.is_truthy() => Ok(left),
            _ => self.right.evaluate(interpreter, env),
        }
    }
}

impl Evaluate for Call {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        let callee = self.callee.evaluate(interpreter, env)?;
        let arguments = self
            .args
            .iter()
            .map(|arg| arg.evaluate(interpreter, env))
            .collect::<RuntimeResult<Vec<_>>>()?;
        callee.call(&arguments, interpreter, env).at_line(self.line)
    }
}

impl Evaluate for Get {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        let object = self.expr.evaluate(interpreter, env)?;
        match object {
            RuntimeValue::Instance(instance) => instance.get(&self.name).ok_or(
                RuntimeException::with_message(&format!("Undefined property '{}'", self.name))
                    .at_line(self.line),
            ),
            _ => Err(RuntimeException::with_message(&format!(
                "Unable to access property '{}'. Only instances have properties",
                self.name
            ))
            .at_line(self.line)),
        }
    }
}

impl Evaluate for Set {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        let object = self.object.evaluate(interpreter, env)?;
        match object {
            RuntimeValue::Instance(instance) => {
                let value = self.value.evaluate(interpreter, env)?;
                instance.borrow_mut().set(&self.name, value.clone());
                Ok(value)
            }
            _ => Err(RuntimeException::with_message(&format!(
                "Unable to set property '{}'. Only instances have properties",
                self.name
            ))
            .at_line(self.line)),
        }
    }
}

impl Evaluate for GroupingExpr {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        self.expression.evaluate(interpreter, env)
    }
}

impl Evaluate for Variable {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        EnvRef::get_var(self, &interpreter.globals, &env)
    }
}

impl Evaluate for Assignment {
    fn evaluate(
        &self,
        interpreter: &mut Interpreter,
        env: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        match &self.target {
            AssignmentTarget::Variable(var) => {
                let rhs_value = self.value.evaluate(interpreter, env)?;
                EnvRef::assign_var(var, rhs_value, &mut interpreter.globals, env)
            }
        }
    }
}
