use crate::{ast::evaluate, parser::Parser};

mod token;
mod scanning;
mod ast;
mod parser;
mod errors;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    match scanning::scan(input) {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Some(expr) => println!("Result: {}", evaluate(&expr)),
                None => eprintln!("Error: Failed to parse expression"),
            }
        }
        Err(e) => eprintln!("Lexing error: {}", e),
    }
}
