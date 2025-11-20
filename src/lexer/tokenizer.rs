use super::token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize, // byte position
}

impl<'a> Lexer<'a> {
    #[must_use]
    pub const fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    /// # Errors
    ///
    /// Returns an error if the input contains invalid tokens or malformed syntax.
    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token::new(TokenKind::Eof, self.position, self.position));
        Ok(tokens)
    }

    #[allow(clippy::too_many_lines)]
    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.current_char();

        match ch {
            '.' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Dot, start, self.position))
            }
            '+' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Plus, start, self.position))
            }
            '-' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Minus, start, self.position))
            }
            '*' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Multiply, start, self.position))
            }
            '/' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Divide, start, self.position))
            }
            '=' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Equals, start, self.position))
            }
            '!' => {
                let start = self.position;
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenKind::NotEquals, start, self.position))
                } else {
                    Err(format!(
                        "Unexpected character '!' at position {}",
                        self.position
                    ))
                }
            }
            '<' => {
                let start = self.position;
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(TokenKind::LessThanOrEqual, start, self.position))
                } else {
                    Ok(Token::new(TokenKind::LessThan, start, self.position))
                }
            }
            '>' => {
                let start = self.position;
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::new(
                        TokenKind::GreaterThanOrEqual,
                        start,
                        self.position,
                    ))
                } else {
                    Ok(Token::new(TokenKind::GreaterThan, start, self.position))
                }
            }
            '(' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::LeftParen, start, self.position))
            }
            ')' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::RightParen, start, self.position))
            }
            '[' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::LeftBracket, start, self.position))
            }
            '`' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::BackTick, start, self.position))
            }
            ']' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::RightBracket, start, self.position))
            }
            ',' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Comma, start, self.position))
            }
            '|' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Pipe, start, self.position))
            }
            '$' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Dollar, start, self.position))
            }
            '%' => {
                let start = self.position;
                self.advance();
                Ok(Token::new(TokenKind::Percent, start, self.position))
            }
            '@' => Ok(self.parse_date()),
            '\'' | '"' => self.parse_string(),
            _ if ch.is_ascii_digit() => self.parse_number(),
            _ if ch.is_ascii_alphabetic() || ch == '_' => Ok(self.parse_identifier_or_keyword()),
            _ => Err(format!(
                "Unexpected character '{ch}' at position {}",
                self.position
            )),
        }
    }

    // TODO: Improve error checking.
    fn parse_date(&mut self) -> Token {
        // Consume @
        self.advance();
        let start = self.position;

        while !self.is_at_end()
            && !self.current_char().is_whitespace()
            && self.current_char() != ')'
            && self.current_char() != ','
        {
            self.advance();
        }

        if self.position - start > 10 {
            return Token::new(TokenKind::ISODateTime, start, self.position);
        }
        Token::new(TokenKind::ISODate, start, self.position)
    }

    fn parse_string(&mut self) -> Result<Token, String> {
        let start = self.position;
        let quote_char = self.current_char();
        // Consume quote
        self.advance();

        while !self.is_at_end() && self.current_char() != quote_char {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err("Unterminated string literal".to_string());
                }
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string literal".to_string());
        }

        // Consume quote
        self.advance();

        Ok(Token::new(TokenKind::String, start, self.position))
    }

    fn parse_number(&mut self) -> Result<Token, String> {
        let start = self.position;
        let mut value = String::new();
        let mut is_float = false;

        while !self.is_at_end()
            && (self.current_char().is_ascii_digit() || self.current_char() == '.')
        {
            if self.current_char() == '.' {
                if is_float {
                    break;
                }
                is_float = true;
            }
            value.push(self.current_char());
            self.advance();
        }

        let end = self.position;
        if is_float {
            value
                .parse::<f64>()
                .map(|n| Token::new(TokenKind::Number(n), start, end))
                .map_err(|_| format!("Invalid number: {value}"))
        } else {
            value
                .parse::<i64>()
                .map(|i| Token::new(TokenKind::Integer(i), start, end))
                .map_err(|_| format!("Invalid integer: {value}"))
        }
    }

    fn parse_identifier_or_keyword(&mut self) -> Token {
        let start_pos = self.position;

        while !self.is_at_end()
            && (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_')
        {
            self.advance();
        }

        let end_pos = self.position;
        let value = &self.input[start_pos..end_pos];

        let kind = match value {
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "xor" => TokenKind::Xor,
            "not" => TokenKind::Not,
            "is" => TokenKind::Is,
            "as" => TokenKind::As,
            "mod" => TokenKind::Mod,
            "where" => TokenKind::Where,
            "select" => TokenKind::Select,
            "all" => TokenKind::All,
            "any" => TokenKind::Any,
            "exists" => TokenKind::Exists,
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            _ => TokenKind::Identifier,
        };
        Token::new(kind, start_pos, end_pos)
    }

    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.input[self.position..].chars().next() {
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            None
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
        let lexer = Lexer::new("Patient.name.family");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].kind, TokenKind::Dot);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].kind, TokenKind::Dot);
        assert_eq!(tokens[4].kind, TokenKind::Identifier);
        assert_eq!(tokens[5].kind, TokenKind::Eof);

        // Test spans
        assert_eq!(tokens[0].length(), 7); // "Patient"
        assert_eq!(tokens[1].length(), 1); // "."
        assert_eq!(tokens[2].length(), 4); // "name"
        assert_eq!(tokens[3].length(), 1); // "."
        assert_eq!(tokens[4].length(), 6); // "family"
    }

    #[test]
    fn test_complex_expression() {
        let lexer = Lexer::new("Patient.name.where(use = 'official').family");
        let tokens = lexer.tokenize().unwrap();

        // Verify we have the expected tokens (including EOF)
        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0].kind, TokenKind::Identifier); // Patient
        assert_eq!(tokens[1].kind, TokenKind::Dot); // .
        assert_eq!(tokens[2].kind, TokenKind::Identifier); // name
        assert_eq!(tokens[3].kind, TokenKind::Dot); // .
        assert_eq!(tokens[4].kind, TokenKind::Where); // where
        assert_eq!(tokens[5].kind, TokenKind::LeftParen); // (
        assert_eq!(tokens[6].kind, TokenKind::Identifier); // use
        assert_eq!(tokens[7].kind, TokenKind::Equals); // =
        assert_eq!(tokens[8].kind, TokenKind::String); // 'official'
        assert_eq!(tokens[9].kind, TokenKind::RightParen); // )
        assert_eq!(tokens[10].kind, TokenKind::Dot); // .
        assert_eq!(tokens[11].kind, TokenKind::Identifier); // family
        assert_eq!(tokens[12].kind, TokenKind::Eof); // EOF
    }

    #[test]
    fn test_numbers_and_strings() {
        let lexer = Lexer::new("age > 18 and name = 'John'");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].kind, TokenKind::Identifier); // age
        assert_eq!(tokens[1].kind, TokenKind::GreaterThan); // >
        assert_eq!(tokens[2].kind, TokenKind::Integer(18)); // 18
        assert_eq!(tokens[3].kind, TokenKind::And); // and
        assert_eq!(tokens[4].kind, TokenKind::Identifier); // name
        assert_eq!(tokens[5].kind, TokenKind::Equals); // =
        assert_eq!(tokens[6].kind, TokenKind::String); // 'John'
        assert_eq!(tokens[7].kind, TokenKind::Eof); // EOF
    }

    #[test]
    fn test_operators() {
        let lexer = Lexer::new("+ - * / = != < <= > >= | $ %");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Plus);
        assert_eq!(tokens[1].kind, TokenKind::Minus);
        assert_eq!(tokens[2].kind, TokenKind::Multiply);
        assert_eq!(tokens[3].kind, TokenKind::Divide);
        assert_eq!(tokens[4].kind, TokenKind::Equals);
        assert_eq!(tokens[5].kind, TokenKind::NotEquals);
        assert_eq!(tokens[6].kind, TokenKind::LessThan);
        assert_eq!(tokens[7].kind, TokenKind::LessThanOrEqual);
        assert_eq!(tokens[8].kind, TokenKind::GreaterThan);
        assert_eq!(tokens[9].kind, TokenKind::GreaterThanOrEqual);
        assert_eq!(tokens[10].kind, TokenKind::Pipe);
        assert_eq!(tokens[11].kind, TokenKind::Dollar);
        assert_eq!(tokens[12].kind, TokenKind::Percent);
    }

    #[test]
    fn test_number_parsing() {
        let lexer = Lexer::new("123 45.67");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Integer(123));
        assert_eq!(tokens[1].kind, TokenKind::Number(45.67));
    }
}
