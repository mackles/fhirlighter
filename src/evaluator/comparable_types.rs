use super::error::Error;
use serde_json::Value;
use time::{Date, PrimitiveDateTime, format_description::well_known::Iso8601};

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
    pub fn from_value(value: &Value) -> Result<Self, Error> {
        match value {
            Value::String(string) => {
                if let Ok(date) = Date::parse(&string, &Iso8601::DATE) {
                    return Ok(Self::ISODate(date));
                }
                if let Ok(datetime) = PrimitiveDateTime::parse(&string, &Iso8601::DEFAULT) {
                    return Ok(Self::ISODateTime(datetime));
                }

                // If parsing fails, treat as regular string
                Ok(Self::String(string.to_string()))
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
            Value::Bool(b) => Ok(Self::Boolean(*b)),
            _ => Err(Error::Parse(
                "Not implemented comparison for type.".to_string(),
            )),
        }
    }
}
