use crate as Rox;
use crate::{token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_eof();
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            _ => Rox::error(self.line, "Unexpected character."),
        };
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        ));
    }

    fn add_eof(&mut self) {
        self.tokens.push(Token::new(
            TokenType::EOF,
            &self.source[self.current..self.source.len()],
            self.line,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens_single_chars() {
        let source = String::from("*+-");
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::STAR, "*", 1),
                Token::new(TokenType::PLUS, "+", 1),
                Token::new(TokenType::MINUS, "-", 1),
                Token::new(TokenType::EOF, "", 1),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_bang_equal() {
        let source = String::from("!=");
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::BANG_EQUAL, "!=", 1),
                Token::new(TokenType::EOF, "", 1)
            ]
        );
    }
}
