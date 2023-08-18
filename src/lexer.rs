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
        let input = input.into().replace('\n', "");

        Lexer {
            input,
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

    pub fn peek_next_token(&mut self) -> Option<Token> {
        self.input
            .char_indices()
            .nth(self.position)
            .map(|(_, ch)| Token::from(&*ch.to_string()))
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let mut tmp_token = "".to_string();
        loop {
            let peek_next_char = self.peek_next_token();

            if peek_next_char
                .is_some_and(|v| v != Token::Letter && v != Token::Underscore && v != Token::Int)
                && !tmp_token.is_empty()
            {
                let token = Token::from(&*tmp_token);

                match token {
                    Token::Illegal | Token::Letter | Token::Underscore => {
                        return Some(Token::Ident(tmp_token));
                    }
                    _ => {
                        return Some(token);
                    }
                }
            }

            let Some(next_char) = self.read_char().ch else {
                return None;
            };

            if next_char.is_whitespace() {
                continue;
            }

            let next_char = &*next_char.to_string();
            let token = Token::from(next_char);

            match token {
                Token::Letter | Token::Underscore | Token::Int => {
                    tmp_token.push_str(next_char);
                    continue;
                }
                _ => return Some(token),
            };
        }
    }
}
