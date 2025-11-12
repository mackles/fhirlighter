use std::borrow::Cow;

use super::error::Error;
use crate::{evaluator::utils::get_from_array, parser::grammar::Expression};
use serde_json::Value;

pub fn eval_function<'a>(
    resource: Cow<'a, Value>,
    function: &str,
) -> Result<Cow<'a, Value>, Error> {
    match function {
        "first" => get_from_array(resource, 0),
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
