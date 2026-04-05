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
            let mut result: Vec<&str> = Vec::with_capacity(array.len());
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

// Helper: get from array by index, borrow if possible, move if owned
pub fn get_from_array(cow_arr: Cow<Value>, index: usize) -> Result<Cow<Value>, Error> {
    match cow_arr {
        Cow::Borrowed(Value::Array(obj)) => obj
            .get(index)
            .map(Cow::Borrowed)
            .ok_or_else(|| Error::Parse(format!("Couldn't retrieve index: {index}"))),
        Cow::Owned(Value::Array(mut arr)) => {
            if index < arr.len() {
                Ok(Cow::Owned(arr.swap_remove(index)))
            } else {
                Err(Error::Parse(format!("Couldn't retrieve index: {index}")))
            }
        }
        _ => Err(Error::Parse("Expected an array".to_string())),
    }
}
