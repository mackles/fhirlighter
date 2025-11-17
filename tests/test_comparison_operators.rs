//! Tests ported from the FHIRPath specification for comparison operators
//!
//! These tests are based on the official FHIRPath test suite:
//! - testEquality (=)
//! - testNEquality (!=)
//! - testLessThan (<)
//! - testLessOrEqual (<=)
//! - testGreaterThan (>)
//! - testGreatorOrEqual (>=)
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

// ============================================================================
// Group: testEquality
// ============================================================================

#[test]
fn test_equality_integers() {
    assert_bool("1 = 1", true);
    assert_bool("1 = 2", false);
    assert_bool("0 = 0", true);
}

#[test]
#[ignore = "decimal literals not yet supported"]
fn test_equality_decimals() {
    assert_bool("1.1 = 1.1", true);
    assert_bool("1.1 = 1.2", false);
    assert_bool("1.10 = 1.1", true);
    assert_bool("0.0 = 0", true);
}

#[test]
fn test_equality_strings() {
    assert_bool("'a' = 'a'", true);
    assert_bool("'a' = 'A'", false);  // Case sensitive
    assert_bool("'a' = 'b'", false);
}

#[test]
fn test_equality_dates() {
    assert_bool("@2012-04-15 = @2012-04-15", true);
    assert_bool("@2012-04-15 = @2012-04-16", false);
}

#[test]
fn test_equality_datetimes() {
    assert_bool("@2012-04-15T15:00:00 = @2012-04-15T10:00:00", false);
    assert_bool("@2012-04-15T15:30:31 = @2012-04-15T15:30:31.0", true);
    assert_bool("@2012-04-15T15:30:31 = @2012-04-15T15:30:31.1", false);
}

#[test]
#[ignore = "requires timezone support"]
fn test_equality_datetimes_with_timezone() {
    assert_bool("@2012-04-15T15:00:00+02:00 = @2012-04-15T16:00:00+03:00", true);
}

// ============================================================================
// Group: testNEquality (!=)
// ============================================================================

#[test]
fn test_not_equal_integers() {
    assert_bool("1 != 1", false);
    assert_bool("1 != 2", true);
    assert_bool("0 != 0", false);
}

#[test]
#[ignore = "decimal literals not yet supported"]
fn test_not_equal_decimals() {
    assert_bool("1.1 != 1.1", false);
    assert_bool("1.1 != 1.2", true);
    assert_bool("1.10 != 1.1", false);
    assert_bool("0.0 != 0", false);
}

#[test]
fn test_not_equal_strings() {
    assert_bool("'a' != 'a'", false);
    assert_bool("'a' != 'b'", true);
}

#[test]
fn test_not_equal_dates() {
    assert_bool("@2012-04-15 != @2012-04-15", false);
    assert_bool("@2012-04-15 != @2012-04-16", true);
}

#[test]
fn test_not_equal_datetimes() {
    assert_bool("@2012-04-15T15:00:00 != @2012-04-15T10:00:00", true);
    assert_bool("@2012-04-15T15:30:31 != @2012-04-15T15:30:31.0", false);
}

// ============================================================================
// Group: testLessThan (<)
// ============================================================================

#[test]
fn test_less_than_integers() {
    assert_bool("1 < 2", true);
    assert_bool("1 < 1", false);
    assert_bool("2 < 1", false);
}

#[test]
#[ignore = "decimal literals not yet supported"]
fn test_less_than_decimals() {
    assert_bool("1.0 < 1.2", true);
    assert_bool("1.0 < 1.0", false);
    assert_bool("1.1 < 1.0", false);
}

#[test]
fn test_less_than_strings() {
    assert_bool("'a' < 'b'", true);
    assert_bool("'A' < 'a'", true);  // Uppercase < lowercase in ASCII
    assert_bool("'a' < 'a'", false);
    assert_bool("'b' < 'a'", false);
}

#[test]
fn test_less_than_dates() {
    assert_bool("@2014-12-12 < @2014-12-13", true);
    assert_bool("@2014-12-12 < @2014-12-12", false);
    assert_bool("@2014-12-13 < @2014-12-12", false);
}

#[test]
fn test_less_than_datetimes() {
    assert_bool("@2014-12-13T12:00:00 < @2014-12-13T12:00:01", true);
    assert_bool("@2014-12-13T12:00:00 < @2014-12-13T12:00:00", false);
    assert_bool("@2014-12-13T12:00:01 < @2014-12-13T12:00:00", false);
}

#[test]
#[ignore = "requires time-only literal support"]
fn test_less_than_times() {
    assert_bool("@T12:00:00 < @T14:00:00", true);
    assert_bool("@T12:00:00 < @T12:00:00", false);
    assert_bool("@T12:00:01 < @T12:00:00", false);
}

// ============================================================================
// Group: testLessOrEqual (<=)
// ============================================================================

#[test]
fn test_less_or_equal_integers() {
    assert_bool("1 <= 2", true);
    assert_bool("1 <= 1", true);
    assert_bool("2 <= 1", false);
}

#[test]
#[ignore = "decimal literals not yet supported"]
fn test_less_or_equal_decimals() {
    assert_bool("1.0 <= 1.2", true);
    assert_bool("1.0 <= 1.0", true);
    assert_bool("1.1 <= 1.0", false);
}

#[test]
fn test_less_or_equal_strings() {
    assert_bool("'a' <= 'b'", true);
    assert_bool("'A' <= 'a'", true);
    assert_bool("'a' <= 'a'", true);
    assert_bool("'b' <= 'a'", false);
}

#[test]
fn test_less_or_equal_dates() {
    assert_bool("@2014-12-12 <= @2014-12-13", true);
    assert_bool("@2014-12-12 <= @2014-12-12", true);
    assert_bool("@2014-12-13 <= @2014-12-12", false);
}

#[test]
fn test_less_or_equal_datetimes() {
    assert_bool("@2014-12-13T12:00:00 <= @2014-12-13T12:00:01", true);
    assert_bool("@2014-12-13T12:00:00 <= @2014-12-13T12:00:00", true);
    assert_bool("@2014-12-13T12:00:01 <= @2014-12-13T12:00:00", false);
}

// ============================================================================
// Group: testGreaterThan (>)
// ============================================================================

#[test]
fn test_greater_than_integers() {
    assert_bool("1 > 2", false);
    assert_bool("1 > 1", false);
    assert_bool("2 > 1", true);
}

#[test]
#[ignore = "decimal literals not yet supported"]
fn test_greater_than_decimals() {
    assert_bool("1.0 > 1.2", false);
    assert_bool("1.0 > 1.0", false);
    assert_bool("1.1 > 1.0", true);
}

#[test]
fn test_greater_than_strings() {
    assert_bool("'a' > 'b'", false);
    assert_bool("'A' > 'a'", false);
    assert_bool("'a' > 'a'", false);
    assert_bool("'b' > 'a'", true);
}

#[test]
fn test_greater_than_dates() {
    assert_bool("@2014-12-12 > @2014-12-13", false);
    assert_bool("@2014-12-12 > @2014-12-12", false);
    assert_bool("@2014-12-13 > @2014-12-12", true);
}

#[test]
fn test_greater_than_datetimes() {
    assert_bool("@2014-12-13T12:00:00 > @2014-12-13T12:00:01", false);
    assert_bool("@2014-12-13T12:00:00 > @2014-12-13T12:00:00", false);
    assert_bool("@2014-12-13T12:00:01 > @2014-12-13T12:00:00", true);
}

// ============================================================================
// Group: testGreatorOrEqual (>=)
// ============================================================================

#[test]
fn test_greater_or_equal_integers() {
    assert_bool("1 >= 2", false);
    assert_bool("1 >= 1", true);
    assert_bool("2 >= 1", true);
}

#[test]
#[ignore = "decimal literals not yet supported"]
fn test_greater_or_equal_decimals() {
    assert_bool("1.0 >= 1.2", false);
    assert_bool("1.0 >= 1.0", true);
    assert_bool("1.1 >= 1.0", true);
}

#[test]
fn test_greater_or_equal_strings() {
    assert_bool("'a' >= 'b'", false);
    assert_bool("'A' >= 'a'", false);
    assert_bool("'a' >= 'a'", true);
    assert_bool("'b' >= 'a'", true);
}

#[test]
fn test_greater_or_equal_dates() {
    assert_bool("@2014-12-12 >= @2014-12-13", false);
    assert_bool("@2014-12-12 >= @2014-12-12", true);
    assert_bool("@2014-12-13 >= @2014-12-12", true);
}

#[test]
fn test_greater_or_equal_datetimes() {
    assert_bool("@2014-12-13T12:00:00 >= @2014-12-13T12:00:01", false);
    assert_bool("@2014-12-13T12:00:00 >= @2014-12-13T12:00:00", true);
    assert_bool("@2014-12-13T12:00:01 >= @2014-12-13T12:00:00", true);
}
