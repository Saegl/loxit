use crate::token::Token;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {source: source.chars().collect(), tokens: vec![]}
    }

    pub fn scan_tokens(&self) -> Vec<i32> {
        vec![]
    }
}
