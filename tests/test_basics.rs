//! Tests ported from the FHIRPath specification testBasics group
//!
//! These tests are based on the official FHIRPath test suite:
//! tests/specification/tests-fhir-r4.xml (testBasics group)

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

/// Helper to assert evaluation returns empty result
fn assert_empty(expression: &str) {
    let result = eval_patient(expression).unwrap();
    let array = result.as_array().expect("Result should be an array");
    assert_eq!(array.len(), 0, "Expected empty result");
}

// Test: name.given
// XML: <test name="testSimple" inputfile="patient-example.xml">
//        <expression>name.given</expression>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//        <output type="string">Jim</output>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//      </test>
#[test]
fn test_simple() {
    assert_strings("name.given", &["Peter", "James", "Jim", "Peter", "James"]);
}

// Test: name.suffix (should return empty)
// XML: <test name="testSimpleNone" inputfile="patient-example.xml">
//        <expression>name.suffix</expression>
//      </test>
#[test]
fn test_simple_none() {
    assert_empty("name.suffix");
}

// Test: name.`given` (with backticks)
// XML: <test name="testEscapedIdentifier" inputfile="patient-example.xml">
//        <expression>name.`given`</expression>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//        <output type="string">Jim</output>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//      </test>
#[test]
fn test_escaped_identifier() {
    assert_strings("name.`given`", &["Peter", "James", "Jim", "Peter", "James"]);
}

// Test: `Patient`.name.`given`
// XML: <test name="testSimpleBackTick1" inputfile="patient-example.xml">
//        <expression>`Patient`.name.`given`</expression>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//        <output type="string">Jim</output>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//      </test>
#[test]
fn test_simple_backtick1() {
    assert_strings(
        "`Patient`.name.`given`",
        &["Peter", "James", "Jim", "Peter", "James"],
    );
}

// Test: name.given1 (invalid field - semantic error)
// XML: <test name="testSimpleFail" inputfile="patient-example.xml" mode="strict">
//        <expression invalid="semantic">name.given1</expression>
//      </test>
#[test]
fn test_simple_fail() {
    // In strict mode, accessing non-existent field should fail
    // For now, it returns empty array in non-strict mode
    let result = eval_patient("name.given1");

    // TODO: When strict mode is implemented, this should return an error
    // For now, we expect empty result
    match result {
        Ok(value) => {
            let array = value.as_array().expect("Result should be an array");
            assert_eq!(array.len(), 0, "Non-existent field should return empty");
        }
        Err(_) => {
            // This is the expected behavior in strict mode
        }
    }
}

// Test: Patient.name.given (with context)
// XML: <test name="testSimpleWithContext" inputfile="patient-example.xml">
//        <expression>Patient.name.given</expression>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//        <output type="string">Jim</output>
//        <output type="string">Peter</output>
//        <output type="string">James</output>
//      </test>
#[test]
fn test_simple_with_context() {
    assert_strings(
        "Patient.name.given",
        &["Peter", "James", "Jim", "Peter", "James"],
    );
}

// Test: Encounter.name.given (wrong context - semantic error)
// XML: <test name="testSimpleWithWrongContext" inputfile="patient-example.xml" mode="strict">
//        <expression invalid="semantic">Encounter.name.given</expression>
//      </test>
#[test]
fn test_simple_with_wrong_context() {
    // In strict mode, using wrong resource type should fail
    // For now, it returns empty array in non-strict mode
    let result = eval_patient("Encounter.name.given");

    // TODO: When strict mode is implemented, this should return an error
    // For now, we expect empty result
    match result {
        Ok(value) => {
            let array = value.as_array().expect("Result should be an array");
            assert_eq!(
                array.len(),
                0,
                "Wrong context should return empty in non-strict mode"
            );
        }
        Err(_) => {
            // This is the expected behavior in strict mode
        }
    }
}
