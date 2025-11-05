use super::error::Error;
use super::evaluation_utils::{eval_function, eval_index};
use crate::parser::grammar::Expression;
use serde_json::Value;

pub struct Evaluator;

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// # Errors
    ///
    /// Returns an error if expression evaluation fails due to invalid syntax or runtime issues.
    pub fn evaluate(&self, expression: &Expression, resource: &Value) -> Result<Value, Error> {
        match self.eval(expression, resource) {
            Ok(value) => Ok(value.clone()),
            Err(error) => match error {
                Error::Parse(_) => Ok(Value::Array(vec![])),
                _ => Err(error),
            },
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn eval<'a>(&self, expression: &Expression, resource: &'a Value) -> Result<&'a Value, Error> {
        match expression {
            Expression::Identifier(name) => {
                let resource_type = resource.get("resourceType").unwrap_or_default();
                if resource_type.as_str().unwrap_or("") == name {
                    return Ok(resource);
                }
                Err(Error::Parse(format!("Resource is not of type: {name}")))
            }
            Expression::MemberAccess { object, member } => {
                let member_object = self.eval(object, resource)?;
                member_object
                    .get(member)
                    .ok_or_else(|| Error::Parse(format!("Couldn't retrieve member: {member}")))
            }
            Expression::Index { object, index } => {
                let index_object = self.eval(object, resource)?;
                let index = eval_index(index, resource)?;
                index_object
                    .get(index)
                    .ok_or_else(|| Error::Parse(format!("Couldn't retrieve index: {index}")))
            }
            Expression::FunctionCall {
                object,
                function,
                arguments,
            } => {
                if let Some(context) = object {
                    let function_object = self.eval(context, resource)?;
                    Ok(eval_function(function_object, function, arguments)?)
                } else {
                    Err(Error::Parse(
                        "Standalone functions are not implemented".to_string(),
                    ))
                }
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_test_patient() -> Value {
        json!({
            "resourceType": "Patient",
            "id": "example",
            "identifier": [{
                "use": "usual",
                "type": {
                    "coding": [{
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR"
                    }]
                },
                "system": "urn:oid:1.2.36.146.595.217.0.1",
                "value": "12345"
            }],
            "active": true,
            "name": [{
                "use": "official",
                "family": "Chalmers",
                "given": ["Peter", "James"]
            }, {
                "use": "usual",
                "given": ["Jim"]
            }],
            "gender": "male"
        })
    }

    #[test]
    fn test_identifier_base_case_success() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::Identifier("Patient".to_string());

        let result = evaluator.evaluate(&expression, &patient.clone()).unwrap();
        assert_eq!(result, patient);
    }

    #[test]
    fn test_identifier_base_case_failure() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::Identifier("Observation".to_string());

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_member_access_simple() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("Patient".to_string())),
            member: "gender".to_string(),
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(result, json!("male"));
    }

    #[test]
    fn test_member_access_nested() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::Index {
            object: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("Patient".to_string())),
                member: "identifier".to_string(),
            }),
            index: Box::new(Expression::Integer(0)),
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(
            result,
            json!({
                "use": "usual",
                "type": {
                    "coding": [{
                        "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                        "code": "MR"
                    }]
                },
                "system": "urn:oid:1.2.36.146.595.217.0.1",
                "value": "12345"
            })
        );
    }

    #[test]
    fn test_member_access_nonexistent() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::MemberAccess {
            object: Box::new(Expression::Identifier("Patient".to_string())),
            member: "nonexistent".to_string(),
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_index_access_array() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::Index {
            object: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("Patient".to_string())),
                member: "name".to_string(),
            }),
            index: Box::new(Expression::Integer(0)),
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(
            result,
            json!({
                "use": "official",
                "family": "Chalmers",
                "given": ["Peter", "James"]
            })
        );
    }

    #[test]
    fn test_index_access_out_of_bounds() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::Index {
            object: Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("Patient".to_string())),
                member: "name".to_string(),
            }),
            index: Box::new(Expression::Integer(5)),
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_first_function_call() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::FunctionCall {
            object: Some(Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("Patient".to_string())),
                member: "name".to_string(),
            })),
            function: "first".to_string(),
            arguments: vec![],
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(
            result,
            json!({
                "use": "official",
                "family": "Chalmers",
                "given": ["Peter", "James"]
            })
        );
    }

    #[test]
    fn test_first_function_call_empty_array() {
        let evaluator = Evaluator::new();
        let patient = json!({
            "resourceType": "Patient",
            "id": "empty",
            "name": []
        });
        let expression = Expression::FunctionCall {
            object: Some(Box::new(Expression::MemberAccess {
                object: Box::new(Expression::Identifier("Patient".to_string())),
                member: "name".to_string(),
            })),
            function: "first".to_string(),
            arguments: vec![],
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_first_function_call_no_object() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let expression = Expression::FunctionCall {
            object: None,
            function: "first".to_string(),
            arguments: vec![],
        };

        let result = evaluator.evaluate(&expression, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }
}
