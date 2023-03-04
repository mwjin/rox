use crate::token::Token;

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: &source,
            tokens: vec![],
        }
    }
}
