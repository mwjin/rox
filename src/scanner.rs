use std::collections::HashMap;

use crate as Rox;
use crate::{token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token<'a>>,
    keywords: HashMap<&'static str, TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source,
            tokens: vec![],
            keywords: Scanner::keywords(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn keywords() -> HashMap<&'static str, TokenType> {
        let mut result = HashMap::new();
        result.insert("and", TokenType::AND);
        result.insert("class", TokenType::CLASS);
        result.insert("else", TokenType::ELSE);
        result.insert("false", TokenType::FALSE);
        result.insert("for", TokenType::FOR);
        result.insert("fun", TokenType::FUN);
        result.insert("if", TokenType::IF);
        result.insert("nil", TokenType::NIL);
        result.insert("or", TokenType::OR);
        result.insert("print", TokenType::PRINT);
        result.insert("return", TokenType::RETURN);
        result.insert("super", TokenType::SUPER);
        result.insert("this", TokenType::THIS);
        result.insert("true", TokenType::TRUE);
        result.insert("var", TokenType::VAR);
        result.insert("while", TokenType::WHILE);
        result
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
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.peek() == '/' {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            '\n' => self.line += 1,
            ' ' | '\t' | '\r' => (),
            '"' => self.scan_string(),
            _ => {
                if c.is_digit(10) {
                    self.scan_number();
                } else if c.is_alphabetic() {
                    self.scan_identifier();
                } else {
                    Rox::error(self.line, "Unexpected character.")
                }
            }
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

    fn peek_next(&self) -> char {
        if self.current >= self.source.len() - 1 {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
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

    fn scan_string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            Rox::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();
        self.add_token(TokenType::STRING);
    }

    fn scan_number(&mut self) {
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        // Scan the fractional part
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.add_token(TokenType::NUMBER);
    }

    fn scan_identifier(&mut self) {
        while self.is_allowed_char_for_id(self.peek()) {
            self.advance();
        }

        let id_literal = &self.source[self.start..self.current];
        match self.get_keyword(id_literal) {
            Some(keyword_token) => self.add_token(keyword_token),
            _ => self.add_token(TokenType::IDENTIFIER),
        }
    }

    fn is_allowed_char_for_id(&self, c: char) -> bool {
        c.is_digit(10) || c.is_alphabetic() || c == '_'
    }

    fn get_keyword(&self, key_str: &str) -> Option<TokenType> {
        self.keywords.get(key_str).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::Scan;

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
    fn test_scan_tokens_starts_with_bang() {
        let source = String::from("!!=");
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::BANG, "!", 1),
                Token::new(TokenType::BANG_EQUAL, "!=", 1),
                Token::new(TokenType::EOF, "", 1)
            ]
        );
    }

    #[test]
    fn test_scan_tokens_operators() {
        let source = "<<=>>====".to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::LESS, "<", 1),
                Token::new(TokenType::LESS_EQUAL, "<=", 1),
                Token::new(TokenType::GREATER, ">", 1),
                Token::new(TokenType::GREATER_EQUAL, ">=", 1),
                Token::new(TokenType::EQUAL_EQUAL, "==", 1),
                Token::new(TokenType::EQUAL, "=", 1),
                Token::new(TokenType::EOF, "", 1),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_skip_comment() {
        let source = "\
// Here is a comment
<>="
        .to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::LESS, "<", 2),
                Token::new(TokenType::GREATER_EQUAL, ">=", 2),
                Token::new(TokenType::EOF, "", 2),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_skip_comment_and_whitespace() {
        let source = "\
// Here is a comment
< > = // inline comment
()"
        .to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::LESS, "<", 2),
                Token::new(TokenType::GREATER, ">", 2),
                Token::new(TokenType::EQUAL, "=", 2),
                Token::new(TokenType::LEFT_PAREN, "(", 3),
                Token::new(TokenType::RIGHT_PAREN, ")", 3),
                Token::new(TokenType::EOF, "", 3),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_string_literals() {
        let source = "=\"Hello, world!\"=\"Here is 
a newline\"<"
            .to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::EQUAL, "=", 1),
                Token::new(TokenType::STRING, "\"Hello, world!\"", 1),
                Token::new(TokenType::EQUAL, "=", 1),
                Token::new(TokenType::STRING, "\"Here is \na newline\"", 2),
                Token::new(TokenType::LESS, "<", 2),
                Token::new(TokenType::EOF, "", 2),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_number_literals() {
        let source = "1.2 + 13 = 14.2".to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::NUMBER, "1.2", 1),
                Token::new(TokenType::PLUS, "+", 1),
                Token::new(TokenType::NUMBER, "13", 1),
                Token::new(TokenType::EQUAL, "=", 1),
                Token::new(TokenType::NUMBER, "14.2", 1),
                Token::new(TokenType::EOF, "", 1),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_number_start_with_dot() {
        let source = ".12.34".to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::DOT, ".", 1),
                Token::new(TokenType::NUMBER, "12.34", 1),
                Token::new(TokenType::EOF, "", 1),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_number_end_with_dot() {
        let source = "1234.".to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::NUMBER, "1234", 1),
                Token::new(TokenType::DOT, ".", 1),
                Token::new(TokenType::EOF, "", 1),
            ]
        );
    }

    #[test]
    fn test_scan_tokens_identifiers() {
        let source = "\
var a = 1;
if (a == 1) a = 2;
"
        .to_string();
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::VAR, "var", 1),
                Token::new(TokenType::IDENTIFIER, "a", 1),
                Token::new(TokenType::EQUAL, "=", 1),
                Token::new(TokenType::NUMBER, "1", 1),
                Token::new(TokenType::SEMICOLON, ";", 1),
                Token::new(TokenType::IF, "if", 2),
                Token::new(TokenType::LEFT_PAREN, "(", 2),
                Token::new(TokenType::IDENTIFIER, "a", 2),
                Token::new(TokenType::EQUAL_EQUAL, "==", 2),
                Token::new(TokenType::NUMBER, "1", 2),
                Token::new(TokenType::RIGHT_PAREN, ")", 2),
                Token::new(TokenType::IDENTIFIER, "a", 2),
                Token::new(TokenType::EQUAL, "=", 2),
                Token::new(TokenType::NUMBER, "2", 2),
                Token::new(TokenType::SEMICOLON, ";", 2),
                Token::new(TokenType::EOF, "", 3),
            ]
        );
    }
}
