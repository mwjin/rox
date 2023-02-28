use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
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
        let token = Token::new(TokenType::IDENTIFIER, "x1".to_string(), 10);
        assert_eq!(token.to_string(), "IDENTIFIER x1 10");

        let token = Token::new(TokenType::PLUS, "+".to_string(), 10);
        assert_eq!(token.to_string(), "PLUS + 10");
    }
}
