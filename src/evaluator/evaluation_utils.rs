use super::error::Error;
use crate::parser::grammar::Expression;
use serde_json::Value;

pub fn eval_function<'a>(resource: &'a Value, function: &str) -> Result<&'a Value, Error> {
    match function {
        "first" => resource
            .get(0)
            .ok_or_else(|| Error::Parse("Array index out of bounds: first()".to_string())),
        function => Err(Error::Unrecoverable(format!(
            "Couldn't evaluate function: {function}"
        ))),
    }
}

pub fn eval_index(index: &Expression, _: &Value) -> Result<usize, Error> {
    match index {
        Expression::Integer(i) => usize::try_from(*i).map_err(|e| {
            Error::IntegerConversion(format!("Couldn't convert integer: {i} with error: {e}"))
        }),
        _other => Err(Error::Unrecoverable("Couldn't evaluate index".to_string())),
    }
}
