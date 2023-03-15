use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        let token = Token::new(TokenType::Identifier, "x1", 10);
        assert_eq!(token.to_string(), "Identifier x1 10");

        let token = Token::new(TokenType::Plus, "+", 10);
        assert_eq!(token.to_string(), "Plus + 10");
    }
}
