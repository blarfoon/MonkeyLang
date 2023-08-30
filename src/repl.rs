use std::io::{self, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut lexer = Lexer::new(input);
        while lexer.ch.is_some() {
            let token = lexer.next_token();

            println!("{:?}", token);
        }
    }
}
