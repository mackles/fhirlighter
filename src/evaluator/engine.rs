use super::error::Error;
use super::evaluation_utils::{eval_function, eval_index};
use crate::parser::ast::Ast;
#[cfg(test)]
use crate::parser::grammar::ExprPool;
use crate::parser::grammar::{ExprRef, Expression};
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
    pub fn evaluate(&self, ast: &Ast, resource: &Value) -> Result<Value, Error> {
        let start = ast.start;
        match self.eval(ast, start, resource) {
            Ok(value) => Ok(value.clone()),
            Err(error) => match error {
                Error::Parse(_) => Ok(Value::Array(vec![])),
                _ => Err(error),
            },
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn eval<'a>(
        &self,
        ast: &Ast,
        expr_ref: ExprRef,
        resource: &'a Value,
    ) -> Result<&'a Value, Error> {
        let expression = ast.expressions.get(expr_ref);
        match expression {
            Expression::Identifier(name) => {
                let resource_type = resource.get("resourceType").unwrap_or_default();
                if resource_type.as_str().unwrap_or("") == name {
                    return Ok(resource);
                }
                Err(Error::Parse(format!("Resource is not of type: {name}")))
            }
            Expression::MemberAccess { object, member } => {
                let member_object = self.eval(ast, *object, resource)?;
                member_object
                    .get(member)
                    .ok_or_else(|| Error::Parse(format!("Couldn't retrieve member: {member}")))
            }
            Expression::Index { object, index } => {
                let index_object = self.eval(ast, *object, resource)?;
                let index = eval_index(ast.expressions.get(index.to_owned()), resource)?;
                index_object
                    .get(index)
                    .ok_or_else(|| Error::Parse(format!("Couldn't retrieve index: {index}")))
            }
            Expression::FunctionCall {
                object,
                function,
                arguments: _,
            } => {
                if let Some(context) = object {
                    let function_object = self.eval(ast, *context, resource)?;
                    let function_expression = ast.expressions.get(*function);
                    if let Expression::Identifier(function_name) = function_expression {
                        Ok(eval_function(function_object, function_name)?)
                    } else {
                        Err(Error::Parse(
                            "Function name must be an identifier".to_string(),
                        ))
                    }
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

    fn create_test_ast_with_single_expr(expr: Expression) -> Ast {
        let mut pool = ExprPool::new();
        let start = pool.add(expr).unwrap();
        Ast {
            expressions: pool,
            start,
        }
    }

    fn create_member_access_ast(object_name: &str, member: &str) -> Ast {
        let mut pool = ExprPool::new();
        let object_ref = pool
            .add(Expression::Identifier(object_name.to_string()))
            .unwrap();
        let start = pool
            .add(Expression::MemberAccess {
                object: object_ref,
                member: member.to_string(),
            })
            .unwrap();
        Ast {
            expressions: pool,
            start,
        }
    }

    fn create_function_call_on_member_ast(
        object_name: &str,
        member: &str,
        function_name: &str,
    ) -> Ast {
        let mut pool = ExprPool::new();
        let object_ref = pool
            .add(Expression::Identifier(object_name.to_string()))
            .unwrap();
        let member_access_ref = pool
            .add(Expression::MemberAccess {
                object: object_ref,
                member: member.to_string(),
            })
            .unwrap();
        let function_ref = pool
            .add(Expression::Identifier(function_name.to_string()))
            .unwrap();
        let start = pool
            .add(Expression::FunctionCall {
                object: Some(member_access_ref),
                function: function_ref,
                arguments: vec![],
            })
            .unwrap();
        Ast {
            expressions: pool,
            start,
        }
    }

    #[test]
    fn test_identifier_base_case_success() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let ast = create_test_ast_with_single_expr(Expression::Identifier("Patient".to_string()));

        let result = evaluator.evaluate(&ast, &patient.clone()).unwrap();
        assert_eq!(result, patient);
    }

    #[test]
    fn test_identifier_base_case_failure() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let ast =
            create_test_ast_with_single_expr(Expression::Identifier("Observation".to_string()));

        let result = evaluator.evaluate(&ast, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_member_access_simple() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let ast = create_member_access_ast("Patient", "gender");

        let result = evaluator.evaluate(&ast, &patient).unwrap();
        assert_eq!(result, json!("male"));
    }

    #[test]
    fn test_member_access_nested() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();

        // Create Patient.identifier[0]
        let mut pool = ExprPool::new();
        let patient_ref = pool
            .add(Expression::Identifier("Patient".to_string()))
            .unwrap();
        let member_access_ref = pool
            .add(Expression::MemberAccess {
                object: patient_ref,
                member: "identifier".to_string(),
            })
            .unwrap();
        let index_ref = pool.add(Expression::Integer(0)).unwrap();
        let start = pool
            .add(Expression::Index {
                object: member_access_ref,
                index: index_ref,
            })
            .unwrap();

        let ast = Ast {
            expressions: pool,
            start,
        };

        let result = evaluator.evaluate(&ast, &patient).unwrap();
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
        let ast = create_member_access_ast("Patient", "nonexistent");

        let result = evaluator.evaluate(&ast, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_index_access_array() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();

        // Create Patient.name[0]
        let mut pool = ExprPool::new();
        let patient_ref = pool
            .add(Expression::Identifier("Patient".to_string()))
            .unwrap();
        let member_access_ref = pool
            .add(Expression::MemberAccess {
                object: patient_ref,
                member: "name".to_string(),
            })
            .unwrap();
        let index_ref = pool.add(Expression::Integer(0)).unwrap();
        let start = pool
            .add(Expression::Index {
                object: member_access_ref,
                index: index_ref,
            })
            .unwrap();

        let ast = Ast {
            expressions: pool,
            start,
        };

        let result = evaluator.evaluate(&ast, &patient).unwrap();
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
        // Create Patient.name[5]
        let mut pool = ExprPool::new();
        let patient_ref = pool
            .add(Expression::Identifier("Patient".to_string()))
            .unwrap();
        let member_access_ref = pool
            .add(Expression::MemberAccess {
                object: patient_ref,
                member: "name".to_string(),
            })
            .unwrap();
        let index_ref = pool.add(Expression::Integer(5)).unwrap();
        let start = pool
            .add(Expression::Index {
                object: member_access_ref,
                index: index_ref,
            })
            .unwrap();

        let ast = Ast {
            expressions: pool,
            start,
        };

        let result = evaluator.evaluate(&ast, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_first_function_call() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();
        let ast = create_function_call_on_member_ast("Patient", "name", "first");

        let result = evaluator.evaluate(&ast, &patient).unwrap();
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
        let ast = create_function_call_on_member_ast("Patient", "name", "first");

        let result = evaluator.evaluate(&ast, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_first_function_call_no_object() {
        let evaluator = Evaluator::new();
        let patient = get_test_patient();

        let mut pool = ExprPool::new();
        let function_ref = pool
            .add(Expression::Identifier("first".to_string()))
            .unwrap();
        let start = pool
            .add(Expression::FunctionCall {
                object: None,
                function: function_ref,
                arguments: vec![],
            })
            .unwrap();

        let ast = Ast {
            expressions: pool,
            start,
        };

        let result = evaluator.evaluate(&ast, &patient).unwrap();
        assert_eq!(result, Value::Array(vec![]));
    }
}
