use super::error::Error;
use crate::parser::grammar::Expression;
use serde_json::Value;
use std::borrow::Cow;
use time::{Date, PrimitiveDateTime, format_description::well_known::Iso8601};

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

pub fn eval_index(index: &Expression, _: &Value) -> Result<usize, Error> {
    match index {
        Expression::Integer(i) => usize::try_from(*i).map_err(|e| {
            Error::IntegerConversion(format!("Couldn't convert integer: {i} with error: {e}"))
        }),
        _other => Err(Error::Unrecoverable("Couldn't evaluate index".to_string())),
    }
}

#[derive(Debug)]
pub enum ComparableTypes {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    ISODateTime(PrimitiveDateTime),
    ISODate(Date),
}

impl PartialEq for ComparableTypes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            // Allow Integer/Float cross-comparisons
            #[allow(clippy::cast_precision_loss)]
            (Self::Integer(a), Self::Float(b)) => (*a as f64) == *b,
            #[allow(clippy::cast_precision_loss)]
            (Self::Float(a), Self::Integer(b)) => *a == (*b as f64),
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::ISODate(a), Self::ISODate(b)) => a == b,
            (Self::ISODateTime(a), Self::ISODateTime(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for ComparableTypes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            // Allow Integer/Float cross-comparisons
            #[allow(clippy::cast_precision_loss)]
            (Self::Integer(a), Self::Float(b)) => (*a as f64).partial_cmp(b),
            #[allow(clippy::cast_precision_loss)]
            (Self::Float(a), Self::Integer(b)) => a.partial_cmp(&(*b as f64)),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Boolean(a), Self::Boolean(b)) => a.partial_cmp(b),
            (Self::ISODate(a), Self::ISODate(b)) => a.partial_cmp(b),
            (Self::ISODateTime(a), Self::ISODateTime(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl ComparableTypes {
    pub fn from_value(value: Value) -> Result<Self, Error> {
        match value {
            Value::String(string) => {
                if let Ok(date) = Date::parse(&string, &Iso8601::DATE) {
                    return Ok(Self::ISODate(date));
                }
                if let Ok(datetime) = PrimitiveDateTime::parse(&string, &Iso8601::DEFAULT) {
                    return Ok(Self::ISODateTime(datetime));
                }

                // If parsing fails, treat as regular string
                Ok(Self::String(string))
            }
            Value::Number(number) => {
                if let Some(int) = number.as_i64() {
                    return Ok(Self::Integer(int));
                } else if let Some(float) = number.as_f64() {
                    return Ok(Self::Float(float));
                }
                Err(Error::Parse(format!(
                    "Couldn't convert: {number} into int or float."
                )))
            }
            Value::Bool(b) => Ok(Self::Boolean(b)),
            _ => Err(Error::Parse(
                "Not implemented comparison for type.".to_string(),
            )),
        }
    }
}
