use crate::ast::{AstNode, Expr};
use crate::err::ParserError::*;
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> AstNode {
    AstNode::Global(String::from("ident"), Box::new(Expr::Word(0)))
}
