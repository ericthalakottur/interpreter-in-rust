use crate::token::TokenType;

#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<TokenType>,
}

impl Scanner {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn scan_tokens(&mut self, source: &str) {
        let mut line = 1;
        let invalid_source_code = false;
        for b in source.chars() {
            match b {
                '(' => self.tokens.push(TokenType::LeftParen(line)),
                ')' => self.tokens.push(TokenType::RightParen(line)),
                _ => {
                    eprintln!("Unexpected character");
                    let invalid_source_code = true;
                }
            };
        }
        println!("{:?}", self);
    }
}
