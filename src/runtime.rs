pub mod error;
pub mod ops;
pub mod value;

use crate::runtime::error::RuntimeError;
use crate::runtime::value::RuntimeValue;

pub type RuntimeResult = Result<RuntimeValue, RuntimeError>;

pub trait RuntimeResultExt {
    fn at_line(self, line: usize) -> Self;
}

impl RuntimeResultExt for RuntimeResult {
    fn at_line(self, line: usize) -> Self {
        self.map_err(|err| err.at_line(line))
    }
}
