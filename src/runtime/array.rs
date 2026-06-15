use crate::environment::EnvRef;
use crate::interpreter::Interpreter;
use crate::runtime::RuntimeResult;
use crate::runtime::call::Callable;
use crate::runtime::value::RuntimeValue;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub type ArrayRef = Rc<RefCell<Array>>;

pub trait ArrayRefExt {
    fn new_array(elements: Vec<RuntimeValue>) -> Self;
}

impl ArrayRefExt for ArrayRef {
    fn new_array(elements: Vec<RuntimeValue>) -> Self {
        Rc::new(RefCell::new(Array::new(elements)))
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Vec<RuntimeValue>,
}

impl Array {
    pub fn new(elements: Vec<RuntimeValue>) -> Self {
        Self { elements }
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        if !self.elements.is_empty() {
            write!(f, "{}", self.elements[0])?;
        }
        for i in 1..self.elements.len() {
            write!(f, ", {}", self.elements[i])?;
        }
        write!(f, "]")
    }
}

pub struct ArrayLen {
    pub array: ArrayRef,
}

impl Callable for ArrayLen {
    fn name(&self) -> String {
        "array.len".to_string()
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        args: &[RuntimeValue],
        _: &mut Interpreter,
        _: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        self.check_arity(args.len())?;
        Ok(RuntimeValue::Number(
            self.array.borrow().elements.len() as f64
        ))
    }
}

pub struct ArrayPush {
    pub array: ArrayRef,
}

impl Callable for ArrayPush {
    fn name(&self) -> String {
        "array.push".to_string()
    }

    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        args: &[RuntimeValue],
        _: &mut Interpreter,
        _: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        self.check_arity(args.len())?;
        self.array.borrow_mut().elements.push(args[0].clone());
        Ok(args[0].clone())
    }
}

pub struct ArrayPop {
    pub array: ArrayRef,
}

impl Callable for ArrayPop {
    fn name(&self) -> String {
        "array.pop".to_string()
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        args: &[RuntimeValue],
        _: &mut Interpreter,
        _: &mut EnvRef,
    ) -> RuntimeResult<RuntimeValue> {
        self.check_arity(args.len())?;
        match self.array.borrow_mut().elements.pop() {
            None => Ok(RuntimeValue::Nil),
            Some(element) => Ok(element),
        }
    }
}
