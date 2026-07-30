use constructs::width_check;
use lexer::Lexer;
use parser::parse;
use petgraph::algo::is_cyclic_directed;
use std::env;
use std::fs::File;
use token::Token;

mod ast;
mod bir;
mod constructs;
mod err;
mod global_cycle;
mod lexer;
mod parser;
mod token;
mod ast_lower;

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
                tokens.push(token);
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    let ast = match parse(tokens) {
        Ok(t) => t,
        Err(e) => panic!("Error: {:?}", e),
    };

    println!("{:?}", ast);

    if is_cyclic_directed(&global_cycle::get_global_graph(&ast)) {
        panic!("Global cycle check failed")
    }

    width_check(&ast).expect("Width Check error");

    // let bir = compile(ast);
}
