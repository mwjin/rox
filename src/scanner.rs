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
        result.insert("and", TokenType::And);
        result.insert("class", TokenType::Class);
        result.insert("else", TokenType::Else);
        result.insert("false", TokenType::False);
        result.insert("for", TokenType::For);
        result.insert("fun", TokenType::Fun);
        result.insert("if", TokenType::If);
        result.insert("nil", TokenType::Nil);
        result.insert("or", TokenType::Or);
        result.insert("print", TokenType::Print);
        result.insert("return", TokenType::Return);
        result.insert("super", TokenType::Super);
        result.insert("this", TokenType::This);
        result.insert("true", TokenType::True);
        result.insert("var", TokenType::Var);
        result.insert("while", TokenType::While);
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
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.peek() == '/' {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
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
            TokenType::Eof,
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
        self.add_token(TokenType::String);
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

        self.add_token(TokenType::Number);
    }

    fn scan_identifier(&mut self) {
        while self.is_allowed_char_for_id(self.peek()) {
            self.advance();
        }

        let id_literal = &self.source[self.start..self.current];
        match self.get_keyword(id_literal) {
            Some(keyword_token) => self.add_token(keyword_token),
            _ => self.add_token(TokenType::Identifier),
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
    use super::*;

    #[test]
    fn test_scan_tokens_single_chars() {
        let source = String::from("*+-");
        let mut scanner = Scanner::new(&source);

        assert_eq!(
            scanner.scan_tokens(),
            &vec![
                Token::new(TokenType::Star, "*", 1),
                Token::new(TokenType::Plus, "+", 1),
                Token::new(TokenType::Minus, "-", 1),
                Token::new(TokenType::Eof, "", 1),
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
                Token::new(TokenType::Bang, "!", 1),
                Token::new(TokenType::BangEqual, "!=", 1),
                Token::new(TokenType::Eof, "", 1)
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
                Token::new(TokenType::Less, "<", 1),
                Token::new(TokenType::LessEqual, "<=", 1),
                Token::new(TokenType::Greater, ">", 1),
                Token::new(TokenType::GreaterEqual, ">=", 1),
                Token::new(TokenType::EqualEqual, "==", 1),
                Token::new(TokenType::Equal, "=", 1),
                Token::new(TokenType::Eof, "", 1),
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
                Token::new(TokenType::Less, "<", 2),
                Token::new(TokenType::GreaterEqual, ">=", 2),
                Token::new(TokenType::Eof, "", 2),
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
                Token::new(TokenType::Less, "<", 2),
                Token::new(TokenType::Greater, ">", 2),
                Token::new(TokenType::Equal, "=", 2),
                Token::new(TokenType::LeftParen, "(", 3),
                Token::new(TokenType::RightParen, ")", 3),
                Token::new(TokenType::Eof, "", 3),
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
                Token::new(TokenType::Equal, "=", 1),
                Token::new(TokenType::String, "\"Hello, world!\"", 1),
                Token::new(TokenType::Equal, "=", 1),
                Token::new(TokenType::String, "\"Here is \na newline\"", 2),
                Token::new(TokenType::Less, "<", 2),
                Token::new(TokenType::Eof, "", 2),
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
                Token::new(TokenType::Number, "1.2", 1),
                Token::new(TokenType::Plus, "+", 1),
                Token::new(TokenType::Number, "13", 1),
                Token::new(TokenType::Equal, "=", 1),
                Token::new(TokenType::Number, "14.2", 1),
                Token::new(TokenType::Eof, "", 1),
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
                Token::new(TokenType::Dot, ".", 1),
                Token::new(TokenType::Number, "12.34", 1),
                Token::new(TokenType::Eof, "", 1),
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
                Token::new(TokenType::Number, "1234", 1),
                Token::new(TokenType::Dot, ".", 1),
                Token::new(TokenType::Eof, "", 1),
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
                Token::new(TokenType::Var, "var", 1),
                Token::new(TokenType::Identifier, "a", 1),
                Token::new(TokenType::Equal, "=", 1),
                Token::new(TokenType::Number, "1", 1),
                Token::new(TokenType::Semicolon, ";", 1),
                Token::new(TokenType::If, "if", 2),
                Token::new(TokenType::LeftParen, "(", 2),
                Token::new(TokenType::Identifier, "a", 2),
                Token::new(TokenType::EqualEqual, "==", 2),
                Token::new(TokenType::Number, "1", 2),
                Token::new(TokenType::RightParen, ")", 2),
                Token::new(TokenType::Identifier, "a", 2),
                Token::new(TokenType::Equal, "=", 2),
                Token::new(TokenType::Number, "2", 2),
                Token::new(TokenType::Semicolon, ";", 2),
                Token::new(TokenType::Eof, "", 3),
            ]
        );
    }
}
