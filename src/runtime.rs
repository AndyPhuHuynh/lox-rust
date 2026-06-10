pub mod call;
pub mod error;
pub mod ops;
pub mod value;

use crate::runtime::error::RuntimeException;

pub type RuntimeResult<OkValue> = Result<OkValue, RuntimeException>;

pub trait RuntimeResultExt {
    fn at_line(self, line: usize) -> Self;
}

impl<T> RuntimeResultExt for RuntimeResult<T> {
    fn at_line(self, line: usize) -> Self {
        self.map_err(|err| err.at_line(line))
    }
}
