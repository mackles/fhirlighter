use super::super::error::Error;
use serde_json::{Number, Value};
use std::borrow::Cow;

pub fn empty<'a>(value: &Value) -> Result<Cow<'a, Value>, Error> {
    match value {
        Value::Array(array) => Ok(Cow::Owned(Value::Bool(array.is_empty()))),
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

pub fn count<'a>(value: &Value) -> Result<Cow<'a, Value>, Error> {
    match value {
        Value::Array(array) => Ok(Cow::Owned(Value::Number(Number::from(array.len())))),
        _ => Err(Error::Parse(
            "count() function expects an array".to_string(),
        )),
    }
}

pub fn exists<'a>(value: &Value) -> Result<Cow<'a, Value>, Error> {
    match value {
        Value::Array(array) => Ok(Cow::Owned(Value::Bool(!array.is_empty()))),
        _ => Err(Error::Parse(
            "exists() function expects an array".to_string(),
        )),
    }
}
