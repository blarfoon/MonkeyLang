#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(String),

    Letter,
    Underscore,

    Illegal,

    // Identifiers + literals
    Ident(String),
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
    Function,
    Let,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "=" => Token::Assign,
            ";" => Token::Semicolon,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "," => Token::Comma,
            "+" => Token::Plus,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            "fn" => Token::Function,
            "let" => Token::Let,
            _ if value.parse::<i32>().is_ok() => Token::Int,
            _ if value.len() == 1 && value.chars().next().map_or(false, |c| c.is_alphabetic()) => {
                Token::Letter
            }
            _ if value.len() == 1 && value.chars().next().map_or(false, |c| c == '_') => {
                Token::Underscore
            }
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

    #[test]
    fn test_real_tokens() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
    "#;

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int,
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int,
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input);
        for test in tests {
            let token = lexer.next_token();
            assert_eq!(token, Some(test));
        }
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_next_token(b: &mut Bencher) {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;

        b.iter(|| {
            let mut lexer = crate::lexer::Lexer::new(input);
            while lexer.next_token().is_some() {}
        });
    }
}
