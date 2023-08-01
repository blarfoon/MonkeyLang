#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(String),
    Illegal,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Funcion,
    Let,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            _ => Token::Illegal,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    use super::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);
        for test in tests {
            let token = lexer.next_token();

            assert_eq!(token, Some(test));
        }
    }
}
