//! Tests ported from the FHIRPath specification testMiscellaneousAccessorTests group
//!
//! These tests are based on the official FHIRPath test suite:
//! tests/specification/tests-fhir-r4.xml (testMiscellaneousAccessorTests group)

use fhirlighter::{Error, evaluate};
use serde_json::Value;
use std::fs;

/// Load the patient example JSON file
fn load_patient_example() -> Value {
    let contents = fs::read_to_string("tests/examples/patient-example.json")
        .expect("Failed to read patient-example.json");
    serde_json::from_str(&contents).expect("Failed to parse patient-example.json")
}

/// Helper function to evaluate an expression against patient example
fn eval_patient(expression: &str) -> Result<Value, Error> {
    let patient = load_patient_example();
    evaluate(expression, &patient)
}

/// Helper to assert evaluation returns specific string values
fn assert_strings(expression: &str, expected: &[&str]) {
    let result = eval_patient(expression).unwrap();
    let array = result.as_array().expect("Result should be an array");

    assert_eq!(
        array.len(),
        expected.len(),
        "Expected {} results, got {}",
        expected.len(),
        array.len()
    );

    for (i, expected_val) in expected.iter().enumerate() {
        assert_eq!(
            array[i].as_str().unwrap(),
            *expected_val,
            "Result[{}] mismatch",
            i
        );
    }
}

// Test: Extract birthDate
// XML: <test name="testExtractBirthDate" description="Extract birthDate"
//           inputfile="patient-example.xml" predicate="false">
//        <expression>birthDate</expression>
//        <output type="date">@1974-12-25</output>
//      </test>
#[test]
fn test_extract_birth_date() {
    let result = eval_patient("birthDate").unwrap();

    // birthDate is a direct field access, should return the value
    assert_eq!(result.as_str().unwrap(), "1974-12-25");
}

// Test: patient has a birthDate
// XML: <test name="testPatientHasBirthDate" description="patient has a birthDate"
//           inputfile="patient-example.xml" predicate="true">
//        <expression>birthDate</expression>
//        <output type="boolean">true</output>
//      </test>
//
// Note: When predicate="true", the test evaluates whether the expression
// returns a truthy value (non-empty result). This is testing the existence
// of birthDate rather than its value.
#[test]
fn test_patient_has_birth_date() {
    let result = eval_patient("birthDate").unwrap();

    // Check that birthDate exists and is not empty
    // In FHIRPath, a non-empty result is truthy
    if let Some(array) = result.as_array() {
        assert!(!array.is_empty(), "birthDate should exist");
    } else {
        // If not an array, it's a single value which means it exists
        assert!(
            result.as_str().is_some(),
            "birthDate should exist and be a string"
        );
    }
}

// Test: patient telecom types
// XML: <test name="testPatientTelecomTypes" description="patient telecom types"
//           inputfile="patient-example.xml">
//        <expression>telecom.use</expression>
//        <output type="code">home</output>
//        <output type="code">work</output>
//        <output type="code">mobile</output>
//        <output type="code">old</output>
//      </test>
#[test]
fn test_patient_telecom_types() {
    assert_strings("telecom.use", &["home", "work", "mobile", "old"]);
}
