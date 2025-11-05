mod evaluator;
mod lexer;
mod parser;

use evaluator::engine::Evaluator;
use lexer::token::FhirPathToken;
use lexer::tokenizer::FhirPathLexer;
use parser::ast::FhirParser;
use serde_json::Value;
use std::env;
use std::fs;
use std::process;

/// # Errors
///
/// Returns an error if the expression contains invalid tokens or malformed syntax.
pub fn parse_fhirpath_expression(expression: &str) -> Result<Vec<FhirPathToken>, String> {
    let mut lexer = FhirPathLexer::new(expression);
    lexer.tokenize()
}

// Example main function demonstrating usage
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Require two args: ./emberpath-rs <path> <file>");
        process::exit(1);
    }
    let test = &args[1];
    let expression = parse_fhirpath_expression(test).unwrap();
    let mut parser = FhirParser::new(&expression);
    let compiled_expression = parser.parse().unwrap();
    let contents = fs::read_to_string(&args[2]).unwrap();
    let data: Value = serde_json::from_str(&contents).unwrap();
    let evaluator = Evaluator::new();
    let result = evaluator.evaluate(&compiled_expression, &data).unwrap();
    println!("Result: {result}");
}
