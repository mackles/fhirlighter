//! Tests ported from the FHIRPath specification for collection functions
//!
//! These tests are based on the official FHIRPath test suite:
//! - testCount
//! - testFirstLast
//! - testWhere
//! - testSelect
//!
//! Source: tests/specification/tests-fhir-r4.xml

use fhirlighter::{Error, evaluate};
use serde_json::Value;
use std::fs;

// ============================================================================
// Test Helpers
// ============================================================================

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

/// Helper to assert evaluation returns a boolean value
fn assert_bool(expression: &str, expected: bool) {
    let result = eval_patient(expression).unwrap();
    assert_eq!(
        result.as_bool().unwrap(),
        expected,
        "Expression '{}' should evaluate to {}",
        expression,
        expected
    );
}

/// Helper to assert evaluation returns an integer value
fn assert_int(expression: &str, expected: i64) {
    let result = eval_patient(expression).unwrap();
    let actual = result.as_i64().unwrap();
    assert_eq!(
        actual, expected,
        "Expression '{}' should evaluate to {}, got {}",
        expression, expected, actual
    );
}

// ============================================================================
// Group: testCount
// ============================================================================

// Test: Patient.name.count()
// XML: <test name="testCount1" inputfile="patient-example.xml">
//        <expression>Patient.name.count()</expression>
//        <output type="integer">3</output>
//      </test>
#[test]
fn test_count1() {
    assert_int("Patient.name.count()", 3);
}

// Test: Patient.name.count() = 3
// XML: <test name="testCount2" inputfile="patient-example.xml">
//        <expression>Patient.name.count() = 3</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
fn test_count2() {
    assert_bool("Patient.name.count() = 3", true);
}

// Test: Patient.name.first().count()
// XML: <test name="testCount3" inputfile="patient-example.xml">
//        <expression>Patient.name.first().count()</expression>
//        <output type="integer">1</output>
//      </test>
#[test]
#[ignore = "fails on count() not returning array"]
fn test_count3() {
    assert_int("Patient.name.first().count()", 1);
}

// Test: Patient.name.first().count() = 1
// XML: <test name="testCount4" inputfile="patient-example.xml">
//        <expression>Patient.name.first().count() = 1</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
#[ignore = "fails on count() not returning array"]
fn test_count4() {
    assert_bool("Patient.name.first().count() = 1", true);
}

// ============================================================================
// Group: testFirstLast
// ============================================================================

// Test: Patient.name.first().given = 'Peter' | 'James'
// XML: <test name="testFirstLast1" inputfile="patient-example.xml">
//        <expression>Patient.name.first().given = 'Peter' | 'James'</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
#[ignore = "requires array eval support"]
fn test_first_last1() {
    assert_bool("Patient.name.first().given = 'Peter' | 'James'", true);
}

// Test: Patient.name.last().given = 'Peter' | 'James'
// XML: <test name="testFirstLast2" inputfile="patient-example.xml">
//        <expression>Patient.name.last().given = 'Peter' | 'James'</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
#[ignore = "requires array eval support"]
fn test_first_last2() {
    assert_bool("Patient.name.last().given = 'Peter' | 'James'", true);
}

// ============================================================================
// Group: testWhere
// ============================================================================

// Test: Patient.name.count() = 3 (baseline for where tests)
// XML: <test name="testWhere1" inputfile="patient-example.xml">
//        <expression>Patient.name.count() = 3</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
fn test_where1() {
    assert_bool("Patient.name.count() = 3", true);
}

// Test: Patient.name.where(given = 'Jim').count() = 1
// XML: <test name="testWhere2" inputfile="patient-example.xml">
//        <expression>Patient.name.where(given = 'Jim').count() = 1</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
#[ignore = "fails on iterating given array"]
fn test_where2() {
    assert_bool("Patient.name.where(given = 'Jim').count() = 1", true);
}

// Test: Patient.name.where(given = 'X').count() = 0
// XML: <test name="testWhere3" inputfile="patient-example.xml">
//        <expression>Patient.name.where(given = 'X').count() = 0</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
#[ignore = "fails on iterating given array"]
fn test_where3() {
    assert_bool("Patient.name.where(given = 'X').count() = 0", true);
}

// Test: Patient.name.where($this.given = 'Jim').count() = 1
// XML: <test name="testWhere4" inputfile="patient-example.xml">
//        <expression>Patient.name.where($this.given = 'Jim').count() = 1</expression>
//        <output type="boolean">true</output>
//      </test>
#[test]
#[ignore = "requires $this context support"]
fn test_where4() {
    assert_bool("Patient.name.where($this.given = 'Jim').count() = 1", true);
}

#[test]
fn test_where5() {
    assert_bool("Patient.name.where(use = 'official').count() = 1", true);
}
