use std::str::CharIndices;

use crate::token::Token;

struct Lexer<'a> {
    input: String,
    iter: CharIndices<'a>,
    pub position: usize,
    pub ch: Option<char>,
}

// Convert to Iterator implementation
impl<'a> Lexer<'a> {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            iter: input.char_indices(),
            position: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) -> &Self {
        let next = self.iter.next();

        match next {
            Some((pos, ch)) => {
                self.position = pos;
                self.ch = Some(ch);
            }
            None => {
                self.position = self.position;
                self.ch = None;
            }
        }

        self
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.read_char().ch.map(|c| c.into())
    }
}
