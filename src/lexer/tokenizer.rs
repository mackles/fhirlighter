use super::token::FhirPathToken;  

pub struct FhirPathLexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> FhirPathLexer<'a> {
    
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    /// # Errors
    /// 
    /// Returns an error if the input contains invalid tokens or malformed syntax.
    pub fn tokenize(&mut self) -> Result<Vec<FhirPathToken>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        tokens.push(FhirPathToken::Eof);
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<FhirPathToken, String> {
        let ch = self.current_char();
        
        match ch {
            '.' => {
                self.advance();
                Ok(FhirPathToken::Dot)
            },
            '+' => {
                self.advance();
                Ok(FhirPathToken::Plus)
            },
            '-' => {
                self.advance();
                Ok(FhirPathToken::Minus)
            },
            '*' => {
                self.advance();
                Ok(FhirPathToken::Multiply)
            },
            '/' => {
                self.advance();
                Ok(FhirPathToken::Divide)
            },
            '=' => {
                self.advance();
                Ok(FhirPathToken::Equals)
            },
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(FhirPathToken::NotEquals)
                } else {
                    Err(format!("Unexpected character '!' at line {}, column {}", self.line, self.column))
                }
            },
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(FhirPathToken::LessThanOrEqual)
                } else {
                    Ok(FhirPathToken::LessThan)
                }
            },
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(FhirPathToken::GreaterThanOrEqual)
                } else {
                    Ok(FhirPathToken::GreaterThan)
                }
            },
            '(' => {
                self.advance();
                Ok(FhirPathToken::LeftParen)
            },
            ')' => {
                self.advance();
                Ok(FhirPathToken::RightParen)
            },
            '[' => {
                self.advance();
                Ok(FhirPathToken::LeftBracket)
            },
            ']' => {
                self.advance();
                Ok(FhirPathToken::RightBracket)
            },
            ',' => {
                self.advance();
                Ok(FhirPathToken::Comma)
            },
            '|' => {
                self.advance();
                Ok(FhirPathToken::Pipe)
            },
            '$' => {
                self.advance();
                Ok(FhirPathToken::Dollar)
            },
            '%' => {
                self.advance();
                Ok(FhirPathToken::Percent)
            },
            '@' => {
                self.advance();
                Ok(FhirPathToken::At)
            },
            '\'' | '"' => self.parse_string(),
            _ if ch.is_ascii_digit() => self.parse_number(),
            _ if ch.is_ascii_alphabetic() || ch == '_' => Ok(self.parse_identifier_or_keyword()),
            _ => Err(format!("Unexpected character '{ch}' at line {}, column {}", self.line, self.column)),
        }
    }
    
    fn parse_string(&mut self) -> Result<FhirPathToken, String> {
        let quote_char = self.current_char();
        // Consume quote
        self.advance();
        
        let mut value = String::new();
        
        while !self.is_at_end() && self.current_char() != quote_char {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err("Unterminated string literal".to_string());
                }
                
                match self.current_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '\'' => value.push('\''),
                    '"' => value.push('"'),
                    _ => {
                        value.push('\\');
                        value.push(self.current_char());
                    }
                }
            } else {
                value.push(self.current_char());
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return Err("Unterminated string literal".to_string());
        }
    
        // Consume quote
        self.advance();
        Ok(FhirPathToken::String(value))
    }
    
    fn parse_number(&mut self) -> Result<FhirPathToken, String> {
        let mut value = String::new();
        let mut is_float = false;
        
        while !self.is_at_end() && (self.current_char().is_ascii_digit() || self.current_char() == '.') {
            if self.current_char() == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            value.push(self.current_char());
            self.advance();
        }
        
        if is_float {
            value.parse::<f64>()
                .map(FhirPathToken::Number)
                .map_err(|_| format!("Invalid number: {value}"))
        } else {
            value.parse::<i64>()
                .map(FhirPathToken::Integer)
                .map_err(|_| format!("Invalid integer: {value}"))
        }
    }
    
    fn parse_identifier_or_keyword(&mut self) -> FhirPathToken {
        let mut value = String::new();
        
        while !self.is_at_end() && (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_') {
            value.push(self.current_char());
            self.advance();
        }
        
        match value.as_str() {
            "and" => FhirPathToken::And,
            "or" => FhirPathToken::Or,
            "xor" => FhirPathToken::Xor,
            "not" => FhirPathToken::Not,
            "is" => FhirPathToken::Is,
            "as" => FhirPathToken::As,
            "mod" => FhirPathToken::Mod,
            "where" => FhirPathToken::Where,
            "select" => FhirPathToken::Select,
            "all" => FhirPathToken::All,
            "any" => FhirPathToken::Any,
            "empty" => FhirPathToken::Empty,
            "exists" => FhirPathToken::Exists,
            "true" => FhirPathToken::Boolean(true),
            "false" => FhirPathToken::Boolean(false),
            _ => FhirPathToken::Identifier(value),
        }
    }
    
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }
    
    fn advance(&mut self) {
        if !self.is_at_end() {
            if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }
    }
    
    const fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() {
            self.advance();
        }
    }
}
// Usage example
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokenization() {
        let mut lexer = FhirPathLexer::new("Patient.name.family");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0], FhirPathToken::Identifier("Patient".to_string()));
        assert_eq!(tokens[1], FhirPathToken::Dot);
        assert_eq!(tokens[2], FhirPathToken::Identifier("name".to_string()));
        assert_eq!(tokens[3], FhirPathToken::Dot);
        assert_eq!(tokens[4], FhirPathToken::Identifier("family".to_string()));
        assert_eq!(tokens[5], FhirPathToken::Eof);
    }
    
    #[test]
    fn test_complex_expression() {
        let mut lexer = FhirPathLexer::new("Patient.name.where(use = 'official').family");
        let tokens = lexer.tokenize().unwrap();
        
        println!("Tokens: {:?}", tokens);
        assert!(!tokens.is_empty());
    }
    
    #[test]
    fn test_numbers_and_strings() {
        let mut lexer = FhirPathLexer::new("age > 18 and name = 'John'");
        let tokens = lexer.tokenize().unwrap();
        
        println!("Tokens: {:?}", tokens);
        assert!(!tokens.is_empty());
    }
}
