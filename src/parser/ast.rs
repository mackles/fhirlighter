use crate::lexer::token::FhirPathToken;
use super::grammar::Expression;
use crate::evaluator::error::Error;

pub struct FhirParser<'a> {
    tokens: &'a Vec<FhirPathToken>,
    position: usize,
}

impl<'a> FhirParser<'a> {
    #[must_use]
    pub const fn new(tokens: &'a Vec<FhirPathToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse tokens into an Abstract Syntax Tree (`AST`)
    /// 
    /// # Errors
    /// 
    /// Returns `Error::Parse` if:
    /// - The token sequence cannot be parsed into a valid expression
    /// - Invalid syntax is encountered
    /// - Unexpected tokens are found
    pub fn parse(&mut self) -> Result<Expression, Error> {
        self.parse_expression()
    }

    /// Parse a complete `FHIRPath` expression with member access, indexing, and function calls
    /// 
    /// # Errors
    /// 
    /// Returns `Error::Parse` if:
    /// - An invocation after a dot operator cannot be parsed
    /// - Invalid syntax is encountered in member access or function calls
    pub fn parse_expression(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_term()?;
        loop {
            if self.peek() == FhirPathToken::Dot {
                self.advance();
                let invocation = self.parse_invocation()?;
                
                match invocation {
                    Expression::FunctionCall{object: _,function,arguments} => {
                        expression = Expression::FunctionCall {
                            object: Some(Box::new(expression)),
                            function, 
                            arguments
                        };
                    },
                    Expression::Identifier(member) => {
                        expression = Expression::MemberAccess {
                            object: Box::new(expression),
                            member: member.to_string(),
                        };
                    },

                    _ => return Err(Error::Parse(format!("Couldn't parse invocation. Received: {invocation}"))),
                } 
            // LeftBracket denotes index of e.g. we have name[0]
            } else if self.peek() == FhirPathToken::LeftBracket {
                self.advance();
                while !self.match_tokens(vec![FhirPathToken::RightBracket]) {
                    let index = self.parse_term()?;
                    expression = Expression::Index {
                        object: Box::new(expression),
                        index: Box::new(index),
                    };
                }
            } else {
                break;
            }
        }

        Ok(expression)
    }


    
    /// Parse a term (literal values or identifiers)
    /// 
    /// # Errors
    /// 
    /// Returns `Error::Parse` if:
    /// - An unexpected token type is encountered
    /// - The current token cannot be parsed as a valid term
    pub fn parse_term(&mut self) -> Result<Expression, Error> {
        match self.peek() {
            FhirPathToken::String(value) => {
                self.advance();
                Ok(Expression::String(value))
            },
            FhirPathToken::Integer(value) => {
                self.advance();
                Ok(Expression::Integer(value))
            },
            FhirPathToken::Number(value) => {
                self.advance();
                Ok(Expression::Number(value))
            },
            FhirPathToken::Boolean(value) => {
                self.advance();
                Ok(Expression::Boolean(value))
            },
            FhirPathToken::Identifier(_) => {
                self.parse_invocation()
            },
            value => Err(Error::Parse(format!("Couldn't parse term. Received: {}. Token: {}", value, self.peek()))),
        }
    }

    /// Parse an invocation (identifier or function call)
    /// 
    /// # Errors
    /// 
    /// Returns `Error::Parse` if:
    /// - The identifier cannot be parsed
    /// - Function call syntax is malformed
    pub fn parse_invocation(&mut self) -> Result<Expression, Error> {
        let mut identifier = self.parse_identifier()?;
        // If we have a function
        if self.peek() == FhirPathToken::LeftParen {
            // Consume the left paren.
            self.advance();
            let mut arguments = Vec::new();
            // If the function parameters are non-empty.
            while self.peek() != FhirPathToken::RightParen {
                let expression = self.parse_expression()?;
                arguments.push(expression);
                // If we hit a comma, skip and loop for the next argument.
                if self.peek() == FhirPathToken::Comma {
                    self.advance();
                }
            }
            
            // Consume the right paren.
            self.advance();
            identifier = Expression::FunctionCall {
                object: None,
                function: identifier.to_string(),
                arguments
            }
        }

        Ok(identifier)
    }

    /// Parse an identifier token
    /// 
    /// # Errors
    /// 
    /// Returns `Error::Parse` if the current token is not an identifier.
    pub fn parse_identifier(&mut self) -> Result<Expression, Error> {
        match self.peek() {
            FhirPathToken::Identifier(value) => {
                self.advance();
                  Ok(Expression::Identifier(value))
            },
            value => Err(Error::Parse(format!("Couldn't parse identifier. Received: {}. token: {}", value, self.peek()))),
        }
    }

    pub fn match_tokens(&mut self, tokens: Vec<FhirPathToken>) -> bool {
        for token in tokens {
            if self.check(&token) {
                self.advance();
                return true;
            }
        }
        false
    }

    #[must_use]
    pub fn check(&self, token: &FhirPathToken) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == *token
    }

    pub fn advance(&mut self) -> FhirPathToken {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    #[must_use]
    pub fn is_at_end(&self) -> bool {
        self.peek() == FhirPathToken::Eof
    }

    #[must_use]
    pub fn previous(&self) -> FhirPathToken {
        self.tokens[self.position - 1].clone()
    }

    #[must_use]
    pub fn peek(&self) -> FhirPathToken {
        self.tokens[self.position].clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::FhirPathToken;

    // Helper function to create a parser with given tokens
      fn create_parser(tokens: &Vec<FhirPathToken>) -> FhirParser {
        FhirParser::new(tokens)
    }


    #[test]
    fn test_parse_identifier_string() {
        let tokens = vec![FhirPathToken::String("test".to_string()), FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::String("test".to_string()));
    }

    #[test]
    fn test_parse_identifier_integer() {
        let tokens = vec![FhirPathToken::Integer(42), FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::Integer(42));
    }

    #[test]
    fn test_parse_identifier_number() {
        let tokens = vec![FhirPathToken::Number(3.14), FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::Number(3.14));
    }

    #[test]
    fn test_parse_identifier_boolean() {
        let tokens = vec![FhirPathToken::Boolean(true), FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::Boolean(true));
    }

    #[test]
    fn test_parse_identifier_name() {
        let tokens = vec![FhirPathToken::Identifier("Patient".to_string()), FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_identifier().unwrap();
        assert_eq!(result, Expression::Identifier("Patient".to_string()));
    }

    #[test]
    fn test_parse_identifier_invalid_token() {
        let tokens = vec![FhirPathToken::Dot, FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_identifier();
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Couldn't parse identifier"));
    }

    #[test]
    fn test_parse_simple_function_call() {
        let tokens = vec![
            FhirPathToken::Identifier("count".to_string()),
            FhirPathToken::LeftParen,
            FhirPathToken::RightParen,
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_invocation().unwrap();
        match result {
            Expression::FunctionCall { object, function, arguments } => {
                assert!(object.is_none());
                assert_eq!(function, "count");
                assert_eq!(arguments.len(), 0);
            },
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_function_call_with_arguments() {
        let tokens = vec![
            FhirPathToken::Identifier("substring".to_string()),
            FhirPathToken::LeftParen,
            FhirPathToken::Integer(0),
            FhirPathToken::Comma,
            FhirPathToken::Integer(5),
            FhirPathToken::RightParen,
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_invocation().unwrap();
        match result {
            Expression::FunctionCall { object, function, arguments } => {
                assert!(object.is_none());
                assert_eq!(function, "substring");
                assert_eq!(arguments.len(), 2);
                assert_eq!(arguments[0], Expression::Integer(0));
                assert_eq!(arguments[1], Expression::Integer(5));
            },
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_member_access() {
        let tokens = vec![
            FhirPathToken::Identifier("Patient".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Identifier("name".to_string()),
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::MemberAccess { object, member } => {
                assert_eq!(*object, Expression::Identifier("Patient".to_string()));
                assert_eq!(member, "name");
            },
            _ => panic!("Expected MemberAccess, got: {:?}", result),
        }
    }

    #[test]
    fn test_parse_chained_member_access() {
        let tokens = vec![
            FhirPathToken::Identifier("Patient".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Identifier("name".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Identifier("family".to_string()),
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::MemberAccess { object, member } => {
                assert_eq!(member, "family");
                match *object {
                    Expression::MemberAccess { object: inner_object, member: inner_member } => {
                        assert_eq!(*inner_object, Expression::Identifier("Patient".to_string()));
                        assert_eq!(inner_member, "name");
                    },
                    _ => panic!("Expected nested MemberAccess"),
                }
            },
            _ => panic!("Expected MemberAccess"),
        }
    }

    #[test]
    fn test_parse_function_call_on_object() {
        let tokens = vec![
            FhirPathToken::Identifier("Patient".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Identifier("count".to_string()),
            FhirPathToken::LeftParen,
            FhirPathToken::RightParen,
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::FunctionCall { object, function, arguments } => {
                assert!(object.is_some());
                assert_eq!(*object.unwrap(), Expression::Identifier("Patient".to_string()));
                assert_eq!(function, "count");
                assert_eq!(arguments.len(), 0);
            },
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_index_access() {
        let tokens = vec![
            FhirPathToken::Identifier("Patient".to_string()),
            FhirPathToken::LeftBracket,
            FhirPathToken::Integer(0),
            FhirPathToken::RightBracket,
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_expression().unwrap();
        match result {
            Expression::Index { object, index } => {
                assert_eq!(*object, Expression::Identifier("Patient".to_string()));
                assert_eq!(*index, Expression::Integer(0));
            },
            _ => panic!("Expected Index"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        // Patient.name[0].family.count()
        let tokens = vec![
            FhirPathToken::Identifier("Patient".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Identifier("name".to_string()),
            FhirPathToken::LeftBracket,
            FhirPathToken::Integer(0),
            FhirPathToken::RightBracket,
            FhirPathToken::Dot,
            FhirPathToken::Identifier("family".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Identifier("count".to_string()),
            FhirPathToken::LeftParen,
            FhirPathToken::RightParen,
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_expression().unwrap();
        // The result should be a function call with a complex object
        match result {
            Expression::FunctionCall { object, function, arguments } => {
                assert!(object.is_some());
                assert_eq!(function, "count");
                assert_eq!(arguments.len(), 0);
                
                // Verify the nested structure
                match *object.unwrap() {
                    Expression::MemberAccess { object: family_object, member } => {
                        assert_eq!(member, "family");
                        match *family_object {
                            Expression::Index { object: index_object, index } => {
                                assert_eq!(*index, Expression::Integer(0));
                                match *index_object {
                                    Expression::MemberAccess { object: patient_object, member: name_member } => {
                                        assert_eq!(*patient_object, Expression::Identifier("Patient".to_string()));
                                        assert_eq!(name_member, "name");
                                    },
                                    _ => panic!("Expected MemberAccess for Patient.name"),
                                }
                            },
                            _ => panic!("Expected Index"),
                        }
                    },
                    _ => panic!("Expected MemberAccess for .family"),
                }
            },
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parser_helper_methods() {
        let tokens = vec![
            FhirPathToken::Identifier("test".to_string()),
            FhirPathToken::Dot,
            FhirPathToken::Eof,
        ];
        let mut parser = create_parser(&tokens);
        
        // Test peek
        assert_eq!(parser.peek(), FhirPathToken::Identifier("test".to_string()));
        
        // Test advance
        let advanced = parser.advance();
        assert_eq!(advanced, FhirPathToken::Identifier("test".to_string()));
        assert_eq!(parser.peek(), FhirPathToken::Dot);
        
        // Test previous
        assert_eq!(parser.previous(), FhirPathToken::Identifier("test".to_string()));
        
        // Test check
        assert!(parser.check(&FhirPathToken::Dot));
        assert!(!parser.check(&FhirPathToken::Comma));
        
        // Test match_tokens
        assert!(parser.match_tokens(vec![FhirPathToken::Dot, FhirPathToken::Comma]));
        
        // Should now be at EOF
        assert!(parser.is_at_end());
    }

    #[test]
    fn test_parse_expression() {
        let tokens = vec![FhirPathToken::Identifier("Patient".to_string()), FhirPathToken::Eof];
        let mut parser = create_parser(&tokens);
        
        let result = parser.parse_expression().unwrap();
        assert_eq!(result, Expression::Identifier("Patient".to_string()));
    }

    #[test]
    fn test_empty_token_list() {
        let tokens = vec![FhirPathToken::Eof];
        let parser = create_parser(&tokens);
        
        // Should immediately be at end
        assert!(parser.is_at_end());
    }
}