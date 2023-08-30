#![feature(test)]

mod lexer;
mod repl;
mod token;

fn main() {
    repl::start();
}
