use std::borrow::Cow;

use serde_json::Value;

use super::error::Error;

pub fn get_from_object<'a>(cow_obj: Cow<'a, Value>, key: &str) -> Result<Cow<'a, Value>, Error> {
    match cow_obj {
        Cow::Borrowed(Value::Object(obj)) => obj
            .get(key)
            .map(Cow::Borrowed)
            .ok_or_else(|| Error::Parse(format!("Couldn't retrieve member: {key}"))),
        Cow::Owned(Value::Object(mut map)) => map
            .remove(key)
            .map(Cow::Owned)
            .ok_or_else(|| Error::Parse(format!("Couldn't retrieve member: {key}"))),
        _ => Err(Error::Parse("Expected an object".to_string())),
    }
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

#[derive(PartialEq, PartialOrd)]
pub enum ComparableTypes {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl ComparableTypes {
    #[must_use]
    pub fn from_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::String(string) => Ok(Self::String(string)),
            Value::Number(number) => {
                if let Some(int) = number.as_i64() {
                    Ok(Self::Integer(int))
                } else {
                    Err(Error::Parse(
                        "Number cannot be represented as i64".to_string(),
                    ))
                }
            }
            Value::Bool(b) => Ok(Self::Boolean(b)),
            _ => Err(Error::Parse(
                "Not implemented comparison for type.".to_string(),
            )),
        }
    }
}
