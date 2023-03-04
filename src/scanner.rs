use crate::token::Token;

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: &source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}
