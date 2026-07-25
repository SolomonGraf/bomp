use lexer::Lexer;
use parser::parse;
use std::env;
use std::fs::File;
use token::Token;

mod ast;
mod constructs;
mod err;
mod lexer;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: bomp <filepath>");
        return;
    }

    let path = &args[1];
    let file: File = File::open(path).expect("Couldn't open file");

    let mut l: Lexer = Lexer::new(file, path);
    let mut tokens: Vec<Token> = Vec::new();

    while !l.eof() {
        match l.next_token() {
            Ok(token) => {
                println!("{:#?}", token);
                tokens.push(token);
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    let mut ast = parse(tokens);
}
