use serde_json::Value;
use std::borrow::Cow;
use time::{Date, PrimitiveDateTime, format_description::well_known::Iso8601};

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

#[derive(Eq, PartialEq, PartialOrd, Debug)]
pub enum ComparableTypes {
    String(String),
    Integer(i64),
    Boolean(bool),
    ISODateTime(PrimitiveDateTime),
    ISODate(Date),
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
                number.as_i64().map_or_else(|| Err(Error::Parse(
                        "Number cannot be represented as i64".to_string(),
                 )), |int| Ok(Self::Integer(int)))
            }
            Value::Bool(b) => Ok(Self::Boolean(b)),
            _ => Err(Error::Parse(
                "Not implemented comparison for type.".to_string(),
            )),
        }
    }
}
