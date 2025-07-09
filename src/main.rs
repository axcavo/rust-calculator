use std::io;

use crate::{ast::evaluate, parser::Parser, token::Token};

mod token;
mod scanning;
mod ast;
mod parser;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");
    let tokens: Vec<Token> = scanning::scan(input);
    let mut parser = Parser::new(tokens);
    println!("{:#?}", evaluate(&parser.parse().unwrap()));
}
