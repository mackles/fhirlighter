use super::error::Error;
use crate::{evaluator::comparable_types::FHIRPathValue, parser::grammar::Expression};
use serde_json::Value;
use std::borrow::Cow;

pub fn get_from_object<'a>(
    cow_obj: FHIRPathValue<'a>,
    key: &str,
) -> Result<FHIRPathValue<'a>, Error> {
    match cow_obj {
        FHIRPathValue::Json(Cow::Borrowed(Value::Object(obj))) => obj
            .get(key)
            .map(Cow::Borrowed)
            .map(FHIRPathValue::Json)
            .ok_or_else(|| Error::Parse(format!("Couldn't retrieve member: {key}"))),
        FHIRPathValue::Json(Cow::Owned(Value::Object(mut map))) => map
            .remove(key)
            .map(Cow::Owned)
            .map(FHIRPathValue::Json)
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
