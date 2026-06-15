use crate::environment::EnvRef;
use crate::syntax_tree::statement::Stmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::runtime::error::RuntimeException;
use crate::runtime::RuntimeResult;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Array(ArrayRef),
    Function(FunctionRef),
    Class(ClassRef),
    Instance(InstanceRef),
}

impl RuntimeValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            RuntimeValue::Bool(false) => false,
            RuntimeValue::Nil => false,
            _ => true,
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Number(num) => write!(f, "{}", num),
            RuntimeValue::String(str) => write!(f, "{}", str),
            RuntimeValue::Bool(bool) => write!(f, "{}", bool),
            RuntimeValue::Array(array) => write!(f, "{}", array.borrow()),
            RuntimeValue::Function(func) => write!(f, "{}", func),
            RuntimeValue::Class(class) => write!(f, "{}", class.borrow()),
            RuntimeValue::Instance(instance) => write!(f, "{}", instance.borrow()),
        }
    }
}

impl PartialEq for RuntimeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RuntimeValue::Nil, RuntimeValue::Nil) => true,
            (RuntimeValue::Number(num1), RuntimeValue::Number(num2)) => num1 == num2,
            (RuntimeValue::String(str1), RuntimeValue::String(str2)) => str1 == str2,
            (RuntimeValue::Bool(bool1), RuntimeValue::Bool(bool2)) => bool1 == bool2,
            (RuntimeValue::Function(a), RuntimeValue::Function(b)) => a == b,
            (RuntimeValue::Class(a), RuntimeValue::Class(b)) => Rc::ptr_eq(a, b),
            (RuntimeValue::Instance(a), RuntimeValue::Instance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

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

impl RuntimeValue {
    pub fn as_index(&self) -> RuntimeResult<usize> {
        match self {
            RuntimeValue::Number(num) => {
                if *num < 0.0 {
                    Err(RuntimeException::with_message("Array index cannot be negative"))
                } else if num.fract() != 0.0 {
                    Err(RuntimeException::with_message("Array index must be an integer value"))
                } else {
                    Ok(*num as usize)
                }
            },
            _ => {
                Err(RuntimeException::with_message("Array index must be a number"))
            }
        }
    }
}

pub type ClassRef = Rc<RefCell<Class>>;

pub trait ClassRefExt {
    fn new_class(
        name: String,
        superclass: Option<ClassRef>,
        methods: HashMap<String, FunctionRef>,
        closure: EnvRef,
    ) -> ClassRef;
}

impl ClassRefExt for ClassRef {
    fn new_class(
        name: String,
        superclass: Option<ClassRef>,
        methods: HashMap<String, FunctionRef>,
        closure: EnvRef,
    ) -> ClassRef {
        Rc::new(RefCell::new(Class::new(name, superclass, methods, closure)))
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub superclass: Option<ClassRef>,
    pub methods: HashMap<String, FunctionRef>,
    pub closure: EnvRef,
}

impl Class {
    pub fn new(
        name: String,
        superclass: Option<ClassRef>,
        methods: HashMap<String, FunctionRef>,
        closure: EnvRef,
    ) -> Self {
        Self {
            name,
            superclass,
            methods,
            closure,
        }
    }

    pub fn get_method(&self, name: &str) -> Option<FunctionRef> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }
        if let Some(superclass) = self.superclass.as_ref() {
            return superclass.borrow().get_method(name);
        }
        None
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionRef {
    pub func: Rc<RefCell<Function>>,
    pub closure: EnvRef,
}

impl FunctionRef {
    pub fn new(
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        is_initializer: bool,
        closure: EnvRef,
    ) -> Self {
        Self {
            func: Rc::new(RefCell::new(Function::new(
                name,
                params,
                body,
                is_initializer,
            ))),
            closure,
        }
    }

    pub fn bind(self, instance: InstanceRef) -> Self {
        let new_env = EnvRef::with_enclosing(Some(self.closure.clone()));
        new_env.define("this".to_string(), RuntimeValue::Instance(instance));

        Self {
            func: self.func,
            closure: new_env,
        }
    }
}

impl PartialEq for FunctionRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func) && self.closure == other.closure
    }
}

impl Display for FunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.func.borrow())
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub is_initializer: bool,
}

impl Function {
    pub fn new(name: String, params: Vec<String>, body: Vec<Stmt>, is_initializer: bool) -> Self {
        Self {
            name,
            params,
            body,
            is_initializer,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

pub type InstanceRef = Rc<RefCell<Instance>>;

pub trait InstanceRefExt {
    fn new_instance(class: ClassRef) -> Self;
    fn get(&self, name: &str) -> Option<RuntimeValue>;
}

impl InstanceRefExt for InstanceRef {
    fn new_instance(class: ClassRef) -> Self {
        Rc::new(RefCell::new(Instance::new(class)))
    }

    fn get(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(value) = self.borrow().fields.get(name) {
            return Some(value.clone());
        }
        if let Some(value) = self.borrow().class.borrow().get_method(name) {
            return Some(RuntimeValue::Function(value.clone().bind(self.clone())));
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    class: ClassRef,
    fields: HashMap<String, RuntimeValue>,
}

impl Instance {
    pub fn new(class: ClassRef) -> Self {
        Instance {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: RuntimeValue) {
        self.fields.insert(name.to_string(), value);
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.borrow().name)
    }
}
