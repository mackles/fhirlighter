# FHIRLighter 


## Overview
A FHIRPath expression parser and evaluator implemented in Rust to minimally implement the [FHIRPath specification](https://www.hl7.org/fhirpath/) . Focused on small binary sizes and support for SQL on FHIR operations, forgoing semantic validation and other functions which return collections.

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
./target/debug/fhirlighter "Patient.name[0].given.first()" "examples/patient.json"
# Result: "Peter"

./target/debug/fhirlighter "Patient.gender" "examples/patient.json"
# Result: "male"

./target/debug/fhirlighter "Patient.identifier[0].value" "examples/patient.json"
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

FHIRLighter follows FHIRPath specification for error handling:

- **Parse Errors**: Return empty arrays (`[]`) for non-matching expressions
- **Unrecoverable Errors**: Return error messages for invalid syntax
- **Graceful Degradation**: Continue evaluation when possible

## License

This project is open source and available under the MIT License.

## Resources

- [FHIRPath Specification](https://www.hl7.org/fhirpath/)
- [Crafting Interpreters](https://craftinginterpreters.com/)
