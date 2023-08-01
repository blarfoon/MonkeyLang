use std::str::CharIndices;

use crate::token::Token;

pub struct Lexer {
    input: String,
    pub position: usize,
    pub ch: Option<char>,
}

// Convert to Iterator implementation
impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        Lexer {
            input: input.into(),
            position: 0,
            ch: None,
        }
    }

    pub fn read_char(&mut self) -> &Self {
        match self.input.char_indices().nth(self.position) {
            Some((pos, ch)) => {
                self.position = pos + 1;
                self.ch = Some(ch);
            }
            None => {
                self.input = String::new();
                self.position = 0;
                self.ch = None;
            }
        }

        self
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.read_char().ch.map(Token::from)
    }
}
