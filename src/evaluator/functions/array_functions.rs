use super::super::error::Error;
use serde_json::{Number, Value};
use std::borrow::Cow;

pub fn empty(cow_arr: Cow<Value>) -> Result<Cow<Value>, Error> {
    match cow_arr {
        Cow::Borrowed(Value::Array(array)) => Ok(Cow::Owned(Value::Bool(array.is_empty()))),
        Cow::Owned(Value::Array(array)) => Ok(Cow::Owned(Value::Bool(array.is_empty()))),
        _ => Err(Error::Parse("Expected an array".to_string())),
    }
}

pub fn last(cow_arr: Cow<Value>) -> Result<Cow<Value>, Error> {
    match cow_arr {
        Cow::Borrowed(Value::Array(array)) => array
            .last()
            .map(Cow::Borrowed)
            .ok_or_else(|| Error::Parse("Couldn't last item from array".to_string())),
        Cow::Owned(Value::Array(mut arr)) => arr
            .pop()
            .map(Cow::Owned)
            .ok_or_else(|| Error::Parse("Couldn't last item from array".to_string())),
        _ => Err(Error::Parse("Expected an array".to_string())),
    }
}

pub fn count(cow_arr: Cow<Value>) -> Result<Cow<Value>, Error> {
    match cow_arr {
        Cow::Borrowed(Value::Array(array)) => {
            Ok(Cow::Owned(Value::Number(Number::from(array.len()))))
        }
        Cow::Owned(Value::Array(array)) => Ok(Cow::Owned(Value::Number(Number::from(array.len())))),
        _ => Err(Error::Parse("Expected an array".to_string())),
    }
}

pub fn exists(cow_arr: Cow<Value>) -> Result<Cow<Value>, Error> {
    match cow_arr {
        Cow::Borrowed(Value::Array(array)) => Ok(Cow::Owned(Value::Bool(!array.is_empty()))),
        Cow::Owned(Value::Array(array)) => Ok(Cow::Owned(Value::Bool(!array.is_empty()))),
        _ => Err(Error::Parse("Expected an array".to_string())),
    }
}
