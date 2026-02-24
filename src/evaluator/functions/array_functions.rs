use crate::parser::grammar::Expression;

use super::super::error::Error;
use serde_json::{Number, Value};
use std::borrow::Cow;

pub fn empty(value: &Value) -> Result<Value, Error> {
    match value {
        Value::Array(array) => Ok(Value::Bool(array.is_empty())),
        _ => Err(Error::Parse(
            "empty() function expects an array".to_string(),
        )),
    }
}

pub fn last(value: Cow<Value>) -> Result<Cow<Value>, Error> {
    match value {
        Cow::Borrowed(Value::Array(array)) => array
            .last()
            .map(Cow::Borrowed)
            .ok_or_else(|| Error::Parse("last() function expects an array".to_string())),
        Cow::Owned(Value::Array(mut arr)) => arr
            .pop()
            .map(Cow::Owned)
            .ok_or_else(|| Error::Parse("Couldn't last item from array".to_string())),
        _ => Err(Error::Parse("last() function expects an array".to_string())),
    }
}

pub fn count(value: &Value) -> Result<Value, Error> {
    match value {
        Value::Array(array) => Ok(Value::Number(Number::from(array.len()))),
        _ => Err(Error::Parse(
            "count() function expects an array".to_string(),
        )),
    }
}

pub fn exists(value: &Value) -> Result<Value, Error> {
    match value {
        Value::Array(array) => Ok(Value::Bool(!array.is_empty())),
        _ => Err(Error::Parse(
            "exists() function expects an array".to_string(),
        )),
    }
}

pub fn single(value: &Value) -> Result<Value, Error> {
    match value {
        Value::Array(array) => Ok(Value::Bool(array.len() == 1)),
        _ => Err(Error::Parse(
            "single() function expects an array".to_string(),
        )),
    }
}

pub fn join(value: &Value, join_char: &Expression) -> Result<Value, Error> {
    if let Expression::String(seperator) = join_char {
        if let Value::Array(array) = value {
            let mut result: Vec<&str> = Vec::new();
            for val in array {
                if let Value::String(string) = val {
                    result.push(string.as_str());
                }
            }
            return Ok(Value::String(result.join(seperator)));
        }
        return Err(Error::Parse("join() function expects an array".to_string()));
    }
    Err(Error::Parse(
        "join() function expects an string to join".to_string(),
    ))
}
