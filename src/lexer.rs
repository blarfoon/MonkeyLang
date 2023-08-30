use std::str::CharIndices;

use crate::token::Token;

pub struct Lexer {
    input: String,
    pub position: usize,
    pub ch: Option<char>,
    pub next_ch: Option<char>,
}

// Convert to Iterator implementation
impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        let input = input.into().replace('\n', "");

        let mut lexer = Lexer {
            input,
            position: 0,
            ch: None,
            next_ch: None,
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) -> &Self {
        match self.input.char_indices().nth(self.position) {
            Some((pos, ch)) => {
                self.position = pos + 1;
                self.ch = Some(ch);
                self.next_ch = self
                    .input
                    .char_indices()
                    .nth(self.position)
                    .map(|(_, ch)| ch);
            }
            None => {
                self.input = String::new();
                self.position = 0;
                self.ch = None;
                self.next_ch = None;
            }
        }

        self
    }

    pub fn is_char_valid_identifier(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    pub fn is_char_valid_integer(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    pub fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.ch {
            if !Lexer::is_char_valid_identifier(ch) {
                break;
            }

            identifier.push(ch);
            self.read_char();
        }

        identifier
    }

    pub fn read_number(&mut self) -> String {
        let mut number = String::new();

        while let Some(ch) = self.ch {
            if !Lexer::is_char_valid_integer(ch) {
                break;
            }

            number.push(ch);
            self.read_char();
        }

        number
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if !ch.is_whitespace() {
                break;
            }

            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let Lexer { ch, next_ch, .. } = self;
        let Some(ch) = ch else {
            return None;
        };

        let res = match ch {
            '=' if next_ch == &Some('=') => {
                self.read_char();
                Token::Eq
            }
            '!' if next_ch == &Some('=') => {
                self.read_char();
                Token::NotEq
            }
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => Token::Bang,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            _ if Lexer::is_char_valid_integer(*ch) => {
                let number = self.read_number();
                return Some(Token::Int(number));
            }
            _ if Lexer::is_char_valid_identifier(*ch) => {
                let ident = self.read_identifier();
                return Some(Token::lookup_ident(&ident));
            }
            _ => Token::Illegal,
        };

        self.read_char();

        Some(res)
    }
}
