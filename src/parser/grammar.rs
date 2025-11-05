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
pub enum Expression {
    // Simple identifier like "Patient" or "name"
    Identifier(String),
    // Function calls like "name.exists()" or "value.substring(0, 3)"
    FunctionCall {
        object: Option<Box<Expression>>, // Optional for standalone functions
        function: String,
        arguments: Vec<Expression>,
    },

    // Member access like "Patient.name" or "name.given"
    MemberAccess {
        object: Box<Expression>,
        member: String,
    },

    // Array indexing like "name[0]"
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
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
