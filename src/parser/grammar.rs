use crate::evaluator::error::Error;
use std::fmt;
/*

Grammar Defintion:
expression = term
           | expression "." invocation
           | expression "[" expression "]"
           | ("+" | "-") expression
           | expression ("*" | "/" | "div" | "mod") expression
           | expression ("+" | "-" | "&") expression
           | expression ("is" | "as") type_specifier
           | expression "|" expression
           | expression ("<=" | "<" | ">" | ">=") expression
           | expression ("=" | "~" | "!=" | "!~") expression
           | expression ("in" | "contains") expression
           | expression "and" expression
           | expression ("or" | "xor") expression
           | expression "implies" expression
           ;

term = invocation
     | literal
     | external_constant
     | "(" expression ")"
     ;

     literal = "{" "}"
        | ("true" | "false")
        | string_literal
        | number_literal
        | date_literal
        | datetime_literal
        | time_literal
        | quantity
        ;

external_constant = "%" (identifier | string_literal) ;

quantity = number_literal [unit] ;

invocation = identifier
           | function_call
           | "$this"
           | "$index"
           | "$total"
           ;

function_call = identifier "(" [param_list] ")" ;

param_list = expression {"," expression} ;
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ExprPool(Vec<Expression>);

impl Default for ExprPool {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprPool {
    #[must_use]
    pub const fn new() -> Self {
        // TODO: Reduce re-allocations by estimating capacity
        Self(Vec::new())
    }

    /// # Errors
    ///
    /// Returns `Error::Parse` if the number of expressions exceeds the maximum size
    /// that can be represented by a u16 (65,535 expressions).
    pub fn add(&mut self, expr: Expression) -> Result<ExprRef, Error> {
        self.0.push(expr);
        let index = (self.0.len() - 1)
            .try_into()
            .map_err(|_| Error::Parse("Number of expressions exceeded pool size".to_string()))?;
        Ok(ExprRef(index))
    }

    #[must_use]
    pub fn get(&self, expr_ref: ExprRef) -> &Expression {
        &self.0[expr_ref.0 as usize]
    }

    // TODO: Avoid this
    pub fn set_function_object(&mut self, expr_ref: ExprRef, object: ExprRef) -> ExprRef {
        let expression = &self.0[expr_ref.0 as usize];
        if let Expression::FunctionCall {
            object: _,
            function,
            arguments,
        } = expression
        {
            self.0[expr_ref.0 as usize] = Expression::FunctionCall {
                object: Some(object),
                function: function.to_owned(),
                arguments: arguments.to_owned(),
            };
        }
        expr_ref
    }
}

// TODO: Remove Copy due to function update in Arena
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ExprRef(u16);

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // Simple identifier like "Patient" or "name"
    Identifier(String),
    // Function calls like "name.exists()" or "value.substring(0, 3)"
    FunctionCall {
        object: Option<ExprRef>, // Optional for standalone functions
        function: ExprRef,
        arguments: Vec<ExprRef>,
    },

    // Member access like "Patient.name" or "name.given"
    MemberAccess {
        object: ExprRef,
        member: String,
    },

    // Array indexing like "name[0]"
    Index {
        object: ExprRef,
        index: ExprRef,
    },

    // Literals
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(name) => write!(f, "{name}"),
            Self::MemberAccess { object, member } => {
                write!(f, "{object}.{member}")
            }
            Self::FunctionCall {
                object,
                function,
                arguments,
            } => {
                write!(f, "Function Call:")?;
                // Handle optional object
                if let Some(obj) = object {
                    write!(f, "{obj}.")?;
                }

                // Write function name and arguments
                write!(f, "{function}(")?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
            }
            Self::Index { object, index } => {
                write!(f, "{object}[{index}]")
            }
            Self::String(s) => write!(f, "'{s}'"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::Boolean(b) => write!(f, "{b}"),
        }
    }
}

impl fmt::Display for ExprRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExprRef({})", self.0)
    }
}
