use crate::{
    evaluator::comparable_types::FHIRPathValue,
    parser::{
        ast::Ast,
        grammar::{ExprRef, Expression},
    },
};

use super::super::error::Error;
use serde_json::Value;
use std::borrow::Cow;

pub fn empty(value: &FHIRPathValue) -> Result<FHIRPathValue<'static>, Error> {
    match value.as_json_ref()? {
        Value::Array(array) => Ok(FHIRPathValue::Boolean(array.is_empty())),
        _ => Err(Error::Parse(
            "empty() function expects an array".to_string(),
        )),
    }
}

pub fn last(value: FHIRPathValue) -> Result<FHIRPathValue, Error> {
    match value {
        FHIRPathValue::Json(Cow::Borrowed(Value::Array(array))) => array
            .last()
            .map(Cow::Borrowed)
            .map(FHIRPathValue::Json)
            .ok_or_else(|| Error::Parse("last() function expects an array".to_string())),
        FHIRPathValue::Json(Cow::Owned(Value::Array(mut arr))) => arr
            .pop()
            .map(Cow::Owned)
            .map(FHIRPathValue::Json)
            .ok_or_else(|| Error::Parse("Couldn't last item from array".to_string())),
        _ => Err(Error::Parse("last() function expects an array".to_string())),
    }
}

pub fn count(value: &FHIRPathValue) -> Result<FHIRPathValue<'static>, Error> {
    match value.as_json_ref()? {
        // Unlikely to have a medical record with max 64 bit value items.
        #[allow(clippy::cast_possible_wrap)]
        Value::Array(array) => Ok(FHIRPathValue::Integer(array.len() as i64)),
        _ => Err(Error::Parse(
            "count() function expects an array".to_string(),
        )),
    }
}

pub fn exists(value: &FHIRPathValue) -> Result<FHIRPathValue<'static>, Error> {
    match value.as_json_ref()? {
        Value::Array(array) => Ok(FHIRPathValue::Boolean(!array.is_empty())),
        _ => Err(Error::Parse(
            "exists() function expects an array".to_string(),
        )),
    }
}

pub fn single(value: &FHIRPathValue) -> Result<FHIRPathValue<'static>, Error> {
    match value.as_json_ref()? {
        Value::Array(array) => Ok(FHIRPathValue::Boolean(array.len() == 1)),
        _ => Err(Error::Parse(
            "single() function expects an array".to_string(),
        )),
    }
}

pub fn join(
    ast: &Ast,
    value: &FHIRPathValue,
    args: &[ExprRef],
) -> Result<FHIRPathValue<'static>, Error> {
    let default_seperator = &Expression::String(String::new());
    let seperator_arg = args
        .first()
        .map_or(default_seperator, |arg_ref| ast.expressions.get(*arg_ref));
    if let Expression::String(seperator) = seperator_arg {
        if let Value::Array(array) = value.as_json_ref()? {
            let mut result: Vec<&str> = Vec::with_capacity(array.len());
            for val in array {
                if let Value::String(string) = val {
                    result.push(string.as_str());
                }
            }
            return Ok(FHIRPathValue::String(result.join(seperator)));
        }
        return Err(Error::Parse("join() function expects an array".to_string()));
    }
    Err(Error::Parse(
        "join() function expects an string to join".to_string(),
    ))
}

// Helper: get from array by index, borrow if possible, move if owned
pub fn get_from_array(value: FHIRPathValue, index: usize) -> Result<FHIRPathValue, Error> {
    match value {
        FHIRPathValue::Json(Cow::Borrowed(Value::Array(obj))) => obj
            .get(index)
            .map(Cow::Borrowed)
            .map(FHIRPathValue::Json)
            .ok_or_else(|| Error::Parse(format!("Couldn't retrieve index: {index}"))),
        FHIRPathValue::Json(Cow::Owned(Value::Array(mut arr))) => {
            if index < arr.len() {
                Ok(FHIRPathValue::Json(Cow::Owned(arr.swap_remove(index))))
            } else {
                Err(Error::Parse(format!("Couldn't retrieve index: {index}")))
            }
        }
        _ => Err(Error::Parse("Expected an array".to_string())),
    }
}
