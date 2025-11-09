//! EmberPath-RS: A `FHIRPath` expression parser and evaluator
//!
//! This library provides functionality to parse and evaluate `FHIRPath` expressions
//! against JSON resources, following the `FHIRPath` specification.
//!
//! # Examples
//!
//! ```rust
//! use emberpath_rs::{evaluate, Error};
//! use serde_json::json;
//!
//! let patient = json!({
//!     "resourceType": "Patient",
//!     "name": [{
//!         "given": ["John", "Doe"]
//!     }],
//!     "gender": "male"
//! });
//!
//! // Simple member access
//! let result = evaluate("Patient.gender", &patient).unwrap();
//! assert_eq!(result, json!("male"));
//!
//! // Array access with function
//! let result = evaluate("Patient.name[0].given.first()", &patient).unwrap();
//! assert_eq!(result, json!("John"));
//! ```

pub mod evaluator;
pub mod lexer;
pub mod parser;

use evaluator::engine::Evaluator;
use lexer::tokenizer::Lexer;
use parser::ast::FhirParser;

// Re-export key types for public API
pub use evaluator::error::Error;
pub use parser::grammar::Expression;
pub use serde_json::Value;

use crate::parser::ast::Ast;

/// Evaluate a `FHIRPath` expression against a JSON resource
///
/// This is the main entry point for the library. It takes a `FHIRPath` expression
/// string and a JSON resource, then returns the evaluated result.
///
/// # Arguments
///
/// * `expression` - A `FHIRPath` expression string (e.g., "Patient.name[0]`.given.first()`")
/// * `resource` - A JSON value representing the FHIR resource
///
/// # Returns
///
/// Returns `Ok(Value)` with the evaluation result, or `Err(Error)` if evaluation fails.
/// Following `FHIRPath` specification, non-matching expressions return empty arrays rather than errors.
///
/// # Examples
///
/// ```rust
/// use emberpath_rs::{evaluate, Error};
/// use serde_json::json;
///
/// let patient = json!({
///     "resourceType": "Patient",
///     "gender": "male"
/// });
///
/// let result = evaluate("Patient.gender", &patient)?;
/// assert_eq!(result, json!("male"));
/// # Ok::<(), Error>(())
/// ```
///
/// # Errors
///
/// Returns an error only for truly unrecoverable conditions like invalid syntax.
/// Non-matching expressions return empty arrays as per `FHIRPath` specification.
pub fn evaluate(expression: &str, resource: &Value) -> Result<Value, Error> {
    // Tokenize the expression
    let lexer = Lexer::new(expression);
    let tokens = lexer
        .tokenize()
        .map_err(|e| Error::Parse(format!("Lexer error: {e}")))?;

    // Parse tokens into AST
    let parser = FhirParser::new(&tokens, expression);
    let ast = parser.parse()?;

    // Evaluate AST against resource
    let evaluator = Evaluator::new();
    evaluator.evaluate(&ast, resource)
}

/// Parse a `FHIRPath` expression into an Abstract Syntax Tree (AST)
///
/// This function is useful if you want to parse an expression once and evaluate
/// it multiple times against different resources.
///
/// # Arguments
///
/// * `expression` - A `FHIRPath` expression string
///
/// # Returns
///
/// Returns the parsed `Expression` AST, or an error if parsing fails.
///
/// # Examples
///
/// ```rust
/// use emberpath_rs::{parse, evaluate_ast};
/// use serde_json::json;
///
/// let ast = parse("Patient.gender")?;
///
/// let patient1 = json!({"resourceType": "Patient", "gender": "male"});
/// let patient2 = json!({"resourceType": "Patient", "gender": "female"});
///
/// let result1 = evaluate_ast(&ast, &patient1)?;
/// let result2 = evaluate_ast(&ast, &patient2)?;
///
/// assert_eq!(result1, json!("male"));
/// assert_eq!(result2, json!("female"));
/// # Ok::<(), emberpath_rs::Error>(())
/// ```
///
/// # Errors
///
/// Returns an error if the expression contains invalid syntax or cannot be parsed.
pub fn parse(expression: &str) -> Result<Ast, Error> {
    let lexer = Lexer::new(expression);
    let tokens = lexer
        .tokenize()
        .map_err(|e| Error::Parse(format!("Lexer error: {e}")))?;

    let parser = FhirParser::new(&tokens, expression);
    parser.parse()
}

/// Evaluate a pre-parsed AST against a JSON resource
///
/// This function takes a pre-parsed Expression AST and evaluates it against
/// a resource. Useful for evaluating the same expression against multiple resources.
///
/// # Arguments
///
/// * `ast` - A parsed `Expression` AST
/// * `resource` - A JSON value representing the FHIR resource
///
/// # Returns
///
/// Returns `Ok(Value)` with the evaluation result, or `Err(Error)` if evaluation fails.
///
/// # Errors
///
/// Returns an error if evaluation fails due to runtime issues.
pub fn evaluate_ast(ast: &Ast, resource: &Value) -> Result<Value, Error> {
    let evaluator = Evaluator::new();
    evaluator.evaluate(ast, resource)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_library_evaluate() {
        let patient = json!({
            "resourceType": "Patient",
            "gender": "male",
            "name": [{
                "given": ["John", "Doe"]
            }]
        });

        // Test simple access
        let result = evaluate("Patient.gender", &patient).unwrap();
        assert_eq!(result, json!("male"));

        // Test array access with function
        let result = evaluate("Patient.name[0].given.first()", &patient).unwrap();
        assert_eq!(result, json!("John"));
    }

    #[test]
    fn test_library_parse_and_evaluate() {
        let ast = parse("Patient.gender").unwrap();

        let patient1 = json!({"resourceType": "Patient", "gender": "male"});
        let patient2 = json!({"resourceType": "Patient", "gender": "female"});

        let result1 = evaluate_ast(&ast, &patient1).unwrap();
        let result2 = evaluate_ast(&ast, &patient2).unwrap();

        assert_eq!(result1, json!("male"));
        assert_eq!(result2, json!("female"));
    }

    #[test]
    fn test_library_empty_result() {
        let patient = json!({
            "resourceType": "Patient",
            "gender": "male"
        });

        // Non-matching expression should return empty array
        let result = evaluate("Patient.nonexistent", &patient).unwrap();
        assert_eq!(result, json!([]));
    }
}
