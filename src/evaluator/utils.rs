use super::error::Error;
use crate::parser::grammar::Expression;
use serde_json::Value;
use std::borrow::Cow;

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

pub fn eval_index(index: &Expression, _: &Value) -> Result<usize, Error> {
    match index {
        Expression::Integer(i) => usize::try_from(*i).map_err(|e| {
            Error::IntegerConversion(format!("Couldn't convert integer: {i} with error: {e}"))
        }),
        _other => Err(Error::Unrecoverable("Couldn't evaluate index".to_string())),
    }
}
