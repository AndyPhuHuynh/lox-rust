use crate::runtime::value::RuntimeValue;
use std::fmt::Display;

#[derive(Debug)]
pub enum RuntimeException {
    Error {
        error_kind: RuntimeErrorKind,
        line: Option<usize>,
    },
    Return {
        value: RuntimeValue,
        line: Option<usize>,
    },
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    ArityMismatch {
        func_name: String,
        func_arity: usize,
        params_arity: usize,
    },
    InvalidArrayAccess {
        message: String,
    },
    InvalidArrayIndex {
        message: String,
    },
    InvalidCallee,
    InvalidOperator {
        message: String,
    },
    InvalidProperty {
        property_name: String,
    },
    InvalidSuperClass {
        super_name: String,
    },
    Redefinition {
        symbol: String,
    },
    UndefinedProperty {
        property_name: String,
    },
    UndefinedVariable {
        var_name: String,
    },
}

impl RuntimeException {
    pub fn arity_error(
        func_name: String,
        func_arity: usize,
        params_arity: usize,
    ) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::ArityMismatch {
                func_name,
                func_arity,
                params_arity,
            },
            line: None,
        }
    }

    pub fn invalid_array_access(message: String) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::InvalidArrayAccess { message },
            line: None,
        }
    }

    pub fn invalid_array_index(message: String) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::InvalidArrayIndex { message },
            line: None,
        }
    }

    pub fn invalid_callee() -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::InvalidCallee,
            line: None,
        }
    }

    pub fn invalid_operator(message: String) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::InvalidOperator { message },
            line: None,
        }
    }

    pub fn invalid_property(property_name: String, line: usize) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::InvalidProperty { property_name },
            line: Some(line),
        }
    }

    pub fn invalid_super_class(super_name: String, line: usize) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::InvalidSuperClass { super_name },
            line: Some(line),
        }
    }

    pub fn redefinition_error(symbol: String, line: usize) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::Redefinition { symbol },
            line: Some(line),
        }
    }

    pub fn undefined_property(property_name: String, line: usize) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::UndefinedProperty { property_name },
            line: Some(line),
        }
    }

    pub fn undefined_variable(var_name: String, line: usize) -> RuntimeException {
        RuntimeException::Error {
            error_kind: RuntimeErrorKind::UndefinedVariable { var_name },
            line: Some(line),
        }
    }

    pub fn return_value(value: RuntimeValue, line: usize) -> Self {
        Self::Return {
            value,
            line: Some(line),
        }
    }

    pub fn at_line(mut self, line_num: usize) -> Self {
        match &mut self {
            Self::Error { line, .. } => match line {
                None => *line = Some(line_num),
                Some(_) => {}
            },
            Self::Return { line, .. } => match line {
                None => *line = Some(line_num),
                Some(_) => {}
            },
        }
        self
    }
}

impl Display for RuntimeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeException::Error { error_kind, line } => match line {
                None => write!(f, "Runtime error: {error_kind}"),
                Some(line) => write!(f, "Runtime error at line {line}: {error_kind}"),
            },
            RuntimeException::Return { line, .. } => match line {
                None => write!(
                    f,
                    "Runtime error: return statement encountered outside of function or method"
                ),
                Some(line) => write!(
                    f,
                    "Runtime error at line {line}: return statement encountered outside of function or method",
                ),
            },
        }
    }
}

impl Display for RuntimeErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeErrorKind::ArityMismatch {
                func_name,
                func_arity,
                params_arity,
            } => {
                write!(
                    f,
                    "Expected {func_arity} arguments but got {params_arity} for call to {func_name}",
                )
            }
            RuntimeErrorKind::InvalidArrayAccess { message } => {
                write!(f, "{}", message)
            }
            RuntimeErrorKind::InvalidArrayIndex { message } => {
                write!(f, "{}", message)
            }
            RuntimeErrorKind::InvalidCallee => {
                write!(f, "Only functions or classes can be called")
            }
            RuntimeErrorKind::InvalidOperator { message } => {
                write!(f, "{}", message)
            }
            RuntimeErrorKind::InvalidProperty { property_name } => {
                write!(
                    f,
                    "Unable to access property '{property_name}'. Only instances have properties"
                )
            }
            RuntimeErrorKind::InvalidSuperClass { super_name } => {
                write!(f, "{super_name} is not a class. Superclass must be a class")
            }
            RuntimeErrorKind::Redefinition { symbol } => {
                write!(
                    f,
                    "Attempting to redefine symbol '{symbol}' which has already been previously defined",
                )
            }
            RuntimeErrorKind::UndefinedProperty { property_name } => {
                write!(f, "Undefined property '{property_name}'")
            }
            RuntimeErrorKind::UndefinedVariable { var_name } => {
                write!(f, "Undefined variable '{var_name}'")
            }
        }
    }
}
