use super::grammar::Expression;
use crate::evaluator::error::Error;
use crate::lexer::token::{Token, TokenKind};

pub struct FhirParser<'a> {
    tokens: &'a Vec<Token>,
    input: &'a str,
    position: usize,
    str_position: usize,
}

impl<'a> FhirParser<'a> {
    #[must_use]
    pub const fn new(tokens: &'a Vec<Token>, input: &'a str) -> Self {
        Self {
            tokens,
            input,
            position: 0,
            str_position: 0, // end of current token
        }
    }

    /// Get the text for a token from the original input
    fn token_text(&self, token: &Token) -> &str {
        token.text(self.input, self.str_position - token.length)
    }

    /// # Errors
    /// Parsing error.
    pub fn parse(&mut self) -> Result<Expression, Error> {
        self.parse_expression()
    }


    fn parse_expression(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_term()?;
        loop {
            if self.peek().kind == TokenKind::Dot {
                self.advance();
                let invocation = self.parse_invocation()?;

                match invocation {
                    Expression::FunctionCall {
                        object: _,
                        function,
                        arguments,
                    } => {
                        expression = Expression::FunctionCall {
                            object: Some(Box::new(expression)),
                            function,
                            arguments,
                        };
                    }
                    Expression::Identifier(member) => {
                        expression = Expression::MemberAccess {
                            object: Box::new(expression),
                            member: member.to_string(),
                        };
                    }

                    _ => {
                        return Err(Error::Parse(format!(
                            "Couldn't parse invocation. Received: {invocation}"
                        )));
                    }
                }
            // LeftBracket denotes index of e.g. we have name[0]
            } else if self.peek().kind == TokenKind::LeftBracket {
                self.advance();
                while !self.match_tokens(vec![TokenKind::RightBracket]) {
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

    fn parse_term(&mut self) -> Result<Expression, Error> {
        match self.peek().kind {
            TokenKind::String => {
                let token = self.advance();
                let text = self.token_text(&token);
                Ok(Expression::String(text.to_string()))
            }
            TokenKind::Integer(value) => {
                self.advance();
                Ok(Expression::Integer(value))
            }
            TokenKind::Number(value) => {
                self.advance();
                Ok(Expression::Number(value))
            }
            TokenKind::Boolean(value) => {
                self.advance();
                Ok(Expression::Boolean(value))
            }
            TokenKind::Identifier => self.parse_invocation(),
            _ => {
                let token = self.peek();
                Err(Error::Parse(format!(
                    "Couldn't parse term. Received: {token}"
                )))
            }
        }
    }

    fn parse_invocation(&mut self) -> Result<Expression, Error> {
        let mut identifier = self.parse_identifier()?;
        // If we have a function
        if self.peek().kind == TokenKind::LeftParen {
            // Consume the left paren.
            self.advance();
            let mut arguments = Vec::new();
            // If the function parameters are non-empty.
            while self.peek().kind != TokenKind::RightParen {
                let expression = self.parse_expression()?;
                arguments.push(expression);
                // If we hit a comma, skip and loop for the next argument.
                if self.peek().kind == TokenKind::Comma {
                    self.advance();
                }
            }

            // Consume the right paren.
            self.advance();
            identifier = Expression::FunctionCall {
                object: None,
                function: identifier.to_string(),
                arguments,
            }
        }

        Ok(identifier)
    }


    fn parse_identifier(&mut self) -> Result<Expression, Error> {
        if self.peek().kind == TokenKind::Identifier {
            let token = self.advance();
            let text = self.token_text(&token);
            Ok(Expression::Identifier(text.to_string()))
        } else {
            let token = self.peek();
            Err(Error::Parse(format!(
                "Couldn't parse identifier. Received: {token}"
            )))
        }
    }

    fn match_tokens(&mut self, tokens: Vec<TokenKind>) -> bool {
        for token_kind in tokens {
            if self.check(&token_kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == *token_kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.str_position += self.peek().length;
            self.position += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn previous(&self) -> Token {
        self.tokens[self.position - 1]
    }

    fn peek(&self) -> Token {
        self.tokens[self.position]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenKind};

    // Helper function to create a parser with given tokens
    fn create_parser<'a>(tokens: &'a Vec<Token>, input: &'a str) -> FhirParser<'a> {
        FhirParser::new(tokens, input)
    }

    // Helper functions to create tokens
    fn create_token(kind: TokenKind, start: usize, end: usize) -> Token {
        Token::new(kind, end - start)
    }

    fn create_identifier_token(start: usize, end: usize) -> Token {
        create_token(TokenKind::Identifier, start, end)
    }

    fn create_string_token(start: usize, end: usize) -> Token {
        create_token(TokenKind::String, start, end)
    }

    fn create_integer_token(value: i64, start: usize, end: usize) -> Token {
        create_token(TokenKind::Integer(value), start, end)
    }

    fn create_number_token(value: f64, start: usize, end: usize) -> Token {
        create_token(TokenKind::Number(value), start, end)
    }

    fn create_boolean_token(value: bool, start: usize, end: usize) -> Token {
        create_token(TokenKind::Boolean(value), start, end)
    }

    fn create_eof_token(pos: usize) -> Token {
        create_token(TokenKind::Eof, pos, pos)
    }

    #[test]
    fn test_parse_identifier_string() {
        let input = "test";
        let tokens = vec![create_string_token(0, 4), create_eof_token(4)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::String("test".to_string()));
    }

    #[test]
    fn test_parse_identifier_integer() {
        let input = "42";
        let tokens = vec![create_integer_token(42, 0, 2), create_eof_token(2)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::Integer(42));
    }

    #[test]
    fn test_parse_identifier_number() {
        let input = "3.14";
        let tokens = vec![create_number_token(3.14, 0, 4), create_eof_token(4)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::Number(3.14));
    }

    #[test]
    fn test_parse_identifier_boolean() {
        let input = "true";
        let tokens = vec![create_boolean_token(true, 0, 4), create_eof_token(4)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_term().unwrap();
        assert_eq!(result, Expression::Boolean(true));
    }

    #[test]
    fn test_parse_identifier_name() {
        let input = "Patient";
        let tokens = vec![create_identifier_token(0, 7), create_eof_token(8)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_identifier().unwrap();
        assert_eq!(result, Expression::Identifier("Patient".to_string()));
    }

    #[test]
    fn test_parse_identifier_invalid_token() {
        let input = ".";
        let tokens = vec![create_token(TokenKind::Dot, 0, 1), create_eof_token(1)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_identifier();
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Couldn't parse identifier"));
    }

    #[test]
    fn test_parse_simple_function_call() {
        let input = "count()";
        let tokens = vec![
            create_identifier_token(0, 5),
            create_token(TokenKind::LeftParen, 5, 6),
            create_token(TokenKind::RightParen, 6, 7),
            create_eof_token(7),
        ];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_invocation().unwrap();
        match result {
            Expression::FunctionCall {
                object,
                function,
                arguments,
            } => {
                assert!(object.is_none());
                assert_eq!(function, "count");
                assert_eq!(arguments.len(), 0);
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_function_call_with_arguments() {
        let input = "substring(0,5)";
        let tokens = vec![
            create_identifier_token(0, 9),
            create_token(TokenKind::LeftParen, 9, 10),
            create_integer_token(0, 10, 11),
            create_token(TokenKind::Comma, 11, 12),
            create_integer_token(5, 12, 13),
            create_token(TokenKind::RightParen, 13, 14),
            create_eof_token(14),
        ];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_invocation().unwrap();
        match result {
            Expression::FunctionCall {
                object,
                function,
                arguments,
            } => {
                assert!(object.is_none());
                assert_eq!(function, "substring");
                assert_eq!(arguments.len(), 2);
                assert_eq!(arguments[0], Expression::Integer(0));
                assert_eq!(arguments[1], Expression::Integer(5));
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_member_access() {
        let input = "Patient.name";
        let tokens = vec![
            create_identifier_token(0, 7),
            create_token(TokenKind::Dot, 7, 8),
            create_identifier_token(8, 12),
            create_eof_token(12),
        ];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_expression().unwrap();
        match result {
            Expression::MemberAccess { object, member } => {
                assert_eq!(*object, Expression::Identifier("Patient".to_string()));
                assert_eq!(member, "name");
            }
            _ => panic!("Expected MemberAccess, got: {:?}", result),
        }
    }

    #[test]
    fn test_parse_chained_member_access() {
        let input = "Patient.name.family";
        let tokens = vec![
            create_identifier_token(0, 7),
            create_token(TokenKind::Dot, 7, 8),
            create_identifier_token(8, 12),
            create_token(TokenKind::Dot, 12, 13),
            create_identifier_token(13, 19),
            create_eof_token(19),
        ];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_expression().unwrap();
        match result {
            Expression::MemberAccess { object, member } => {
                assert_eq!(member, "family");
                match *object {
                    Expression::MemberAccess {
                        object: inner_object,
                        member: inner_member,
                    } => {
                        assert_eq!(*inner_object, Expression::Identifier("Patient".to_string()));
                        assert_eq!(inner_member, "name");
                    }
                    _ => panic!("Expected nested MemberAccess"),
                }
            }
            _ => panic!("Expected MemberAccess"),
        }
    }

    #[test]
    fn test_parse_function_call_on_object() {
        let input = "Patient.count()";
        let tokens = vec![
            create_identifier_token(0, 7),
            create_token(TokenKind::Dot, 7, 8),
            create_identifier_token(8, 13),
            create_token(TokenKind::LeftParen, 13, 14),
            create_token(TokenKind::RightParen, 14, 15),
            create_eof_token(15),
        ];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_expression().unwrap();
        match result {
            Expression::FunctionCall {
                object,
                function,
                arguments,
            } => {
                assert!(object.is_some());
                assert_eq!(
                    *object.unwrap(),
                    Expression::Identifier("Patient".to_string())
                );
                assert_eq!(function, "count");
                assert_eq!(arguments.len(), 0);
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parse_index_access() {
        let input = "Patient[0]";
        let tokens = vec![
            create_identifier_token(0, 7),
            create_token(TokenKind::LeftBracket, 7, 8),
            create_integer_token(0, 8, 9),
            create_token(TokenKind::RightBracket, 9, 10),
            create_eof_token(10),
        ];
        let mut parser = create_parser(&tokens, input);
        let result = parser.parse_expression().unwrap();
        println!("{}", result);
        match result {
            Expression::Index { object, index } => {
                assert_eq!(*object, Expression::Identifier("Patient".to_string()));
                assert_eq!(*index, Expression::Integer(0));
            }
            _ => panic!("Expected Index"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        // Patient.name[0].family.count()
        let input = "Patient.name[0].family.count()";
        let tokens = vec![
            create_identifier_token(0, 7),
            create_token(TokenKind::Dot, 7, 8),
            create_identifier_token(8, 12),
            create_token(TokenKind::LeftBracket, 12, 13),
            create_integer_token(0, 13, 14),
            create_token(TokenKind::RightBracket, 14, 15),
            create_token(TokenKind::Dot, 15, 16),
            create_identifier_token(16, 22),
            create_token(TokenKind::Dot, 22, 23),
            create_identifier_token(23, 28),
            create_token(TokenKind::LeftParen, 28, 29),
            create_token(TokenKind::RightParen, 29, 30),
            create_eof_token(30),
        ];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_expression().unwrap();
        // The result should be a function call with a complex object
        match result {
            Expression::FunctionCall {
                object,
                function,
                arguments,
            } => {
                assert!(object.is_some());
                assert_eq!(function, "count");
                assert_eq!(arguments.len(), 0);

                // Verify the nested structure
                match *object.unwrap() {
                    Expression::MemberAccess {
                        object: family_object,
                        member,
                    } => {
                        assert_eq!(member, "family");
                        match *family_object {
                            Expression::Index {
                                object: index_object,
                                index,
                            } => {
                                assert_eq!(*index, Expression::Integer(0));
                                match *index_object {
                                    Expression::MemberAccess {
                                        object: patient_object,
                                        member: name_member,
                                    } => {
                                        assert_eq!(
                                            *patient_object,
                                            Expression::Identifier("Patient".to_string())
                                        );
                                        assert_eq!(name_member, "name");
                                    }
                                    _ => panic!("Expected MemberAccess for Patient.name"),
                                }
                            }
                            _ => panic!("Expected Index"),
                        }
                    }
                    _ => panic!("Expected MemberAccess for .family"),
                }
            }
            _ => panic!("Expected FunctionCall"),
        }
    }

    #[test]
    fn test_parser_helper_methods() {
        let input = "test.";
        let tokens = vec![
            create_identifier_token(0, 4),
            create_token(TokenKind::Dot, 4, 5),
            create_eof_token(5),
        ];
        let mut parser = create_parser(&tokens, input);

        // Test peek
        assert_eq!(parser.peek().kind, TokenKind::Identifier);

        // Test advance
        let advanced = parser.advance();
        assert_eq!(advanced.kind, TokenKind::Identifier);
        assert_eq!(parser.peek().kind, TokenKind::Dot);

        // Test previous
        assert_eq!(parser.previous().kind, TokenKind::Identifier);

        // Test check
        assert!(parser.check(&TokenKind::Dot));
        assert!(!parser.check(&TokenKind::Comma));

        // Test match_tokens
        assert!(parser.match_tokens(vec![TokenKind::Dot, TokenKind::Comma]));

        // Should now be at EOF
        assert!(parser.is_at_end());
    }

    #[test]
    fn test_parse_expression() {
        let input = "Patient";
        let tokens = vec![create_identifier_token(0, 7), create_eof_token(7)];
        let mut parser = create_parser(&tokens, input);

        let result = parser.parse_expression().unwrap();
        assert_eq!(result, Expression::Identifier("Patient".to_string()));
    }

    #[test]
    fn test_empty_token_list() {
        let input = "";
        let tokens = vec![create_eof_token(0)];
        let parser = create_parser(&tokens, input);

        // Should immediately be at end
        assert!(parser.is_at_end());
    }
}
