use crate::runtime::RuntimeResult;
use crate::runtime::error::RuntimeException;
use crate::runtime::value::RuntimeValue;

// Unary ops
impl RuntimeValue {
    pub fn logical_not(self) -> RuntimeValue {
        RuntimeValue::Bool(!self.is_truthy())
    }

    pub fn negation(self) -> RuntimeResult<RuntimeValue> {
        match self {
            RuntimeValue::Number(num) => Ok(RuntimeValue::Number(-num)),
            _ => Err(RuntimeException::invalid_operator(
                "Negation operand must be a number".to_string(),
            )),
        }
    }
}

fn check_both_numbers(lhs: &RuntimeValue, rhs: &RuntimeValue) -> Option<(f64, f64)> {
    if let RuntimeValue::Number(lhs_num) = lhs {
        if let RuntimeValue::Number(rhs_num) = rhs {
            return Some((*lhs_num, *rhs_num));
        }
    }
    None
}

fn check_both_strings<'a>(
    lhs: &'a RuntimeValue,
    rhs: &'a RuntimeValue,
) -> Option<(&'a str, &'a str)> {
    if let RuntimeValue::String(lhs_str) = lhs {
        if let RuntimeValue::String(rhs_str) = rhs {
            return Some((lhs_str, rhs_str));
        }
    }
    None
}

// Binary ops
impl RuntimeValue {
    pub fn add(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            Ok(RuntimeValue::Number(left_num + right_num))
        } else if let Some((left_str, right_str)) = check_both_strings(&self, &rhs) {
            Ok(RuntimeValue::String(left_str.to_string() + right_str))
        } else {
            Err(RuntimeException::invalid_operator(
                "Addition operands must either be both strings or both numbers".to_string(),
            ))
        }
    }

    pub fn sub(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Number(left_num - right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Subtraction operands must both be numbers".to_string(),
        ))
    }

    pub fn mul(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Number(left_num * right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Multiplication operands must both be numbers".to_string(),
        ))
    }

    pub fn div(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Number(left_num / right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Division operands must both be numbers".to_string(),
        ))
    }

    pub fn greater_than(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Bool(left_num > right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Greater than operands must both be numbers".to_string(),
        ))
    }

    pub fn greater_than_or_equal(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Bool(left_num >= right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Greater than or equal operands must both be numbers".to_string(),
        ))
    }
    pub fn less_than(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Bool(left_num < right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Less than operands must both be numbers".to_string(),
        ))
    }

    pub fn less_than_or_equal(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        if let Some((left_num, right_num)) = check_both_numbers(&self, &rhs) {
            return Ok(RuntimeValue::Bool(left_num <= right_num));
        }
        Err(RuntimeException::invalid_operator(
            "Less than or equal operands must both be numbers".to_string(),
        ))
    }

    pub fn not_equal(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        Ok(RuntimeValue::Bool(self != rhs))
    }

    pub fn equal(self, rhs: RuntimeValue) -> RuntimeResult<RuntimeValue> {
        Ok(RuntimeValue::Bool(self == rhs))
    }
}
