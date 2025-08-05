# Emberpath 

A FHIRPath expression parser and evaluator implemented in Rust as a learning project for both Rust programming and parser implementation.

## Overview

Emberpath is a basic implementation of the [FHIRPath specification](https://www.hl7.org/fhirpath/) - a path-based navigation and extraction language for FHIR resources. 

## Architecture

The project is organized into three main components:

- **Lexer** (`src/lexer/`): Tokenizes FHIRPath expressions into meaningful tokens
- **Parser** (`src/parser/`): Constructs an Abstract Syntax Tree (AST) from tokens
- **Evaluator** (`src/evaluator/`): Evaluates expressions against JSON resources

## Usage

### Command Line Interface

Run FHIRPath expressions against JSON files:

```bash
# Build the project
cargo build

# Evaluate expressions
./target/debug/emberpath-rs "Patient.name[0].given.first()" "examples/patient.json"
# Result: "Peter"

./target/debug/emberpath-rs "Patient.gender" "examples/patient.json"
# Result: "male"

./target/debug/emberpath-rs "Patient.identifier[0].value" "examples/patient.json"
# Result: "12345"
```

### Development Commands

```bash
# Run all tests
cargo test

# Build in release mode
cargo build --release

# Run with debug output
cargo run -- "Patient.name" "examples/patient.json"
```

## Example Usage

Given a FHIR Patient resource in `examples/patient.json`:

```json
{
  "resourceType": "Patient",
  "id": "example",
  "name": [{
    "use": "official",
    "family": "Chalmers",
    "given": ["Peter", "James"]
  }],
  "gender": "male"
}
```

You can evaluate various FHIRPath expressions:

| Expression | Result | Description |
|------------|--------|-------------|
| `Patient` | `{...}` | Returns the entire Patient resource |
| `Patient.gender` | `"male"` | Simple member access |
| `Patient.name[0]` | `{...}` | Array indexing |
| `Patient.name[0].given.first()` | `"Peter"` | Chained operations |


## Testing

```bash
# Run all tests
cargo test

```

## Error Handling

Emberpath follows FHIRPath specification for error handling:

- **Parse Errors**: Return empty arrays (`[]`) for non-matching expressions
- **Unrecoverable Errors**: Return error messages for invalid syntax
- **Graceful Degradation**: Continue evaluation when possible

## License

This project is open source and available under the MIT License.

## Resources

- [FHIRPath Specification](https://www.hl7.org/fhirpath/)
- [FHIR Resources](https://www.hl7.org/fhir/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Crafting Interpreters](https://craftinginterpreters.com/)
