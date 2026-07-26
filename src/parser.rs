use crate::ast::AstNode::Function;
use crate::ast::{AstNode, Expr, Function as Fun, Param};
use crate::err::ParserError::{self, *};
use crate::token::{
    Token,
    TokenKind::{self, *},
};
use std::iter::Peekable;

// HELPERS

type ParseIter<'a> = Peekable<std::vec::IntoIter<Token<'a>>>;

pub fn check_next<'a>(iter: &mut ParseIter, kind_exp: TokenKind) -> Result<(), ParserError> {
    match iter.next() {
        Some(Token {
            kind: kind_actual, ..
        }) if kind_actual == kind_exp => Ok(()),
        _ => Err(ParsingError()),
    }
}

pub fn check_peek<'a>(iter: &mut ParseIter, kind_exp: TokenKind) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token {
            kind: kind_actual, ..
        }) if *kind_actual == kind_exp => Ok(()),
        _ => Err(ParsingError()),
    }
}

pub fn check_peek_ident<'a>(iter: &mut ParseIter) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token {
            kind: Identifier(_),
            ..
        }) => Ok(()),
        _ => Err(ParsingError()),
    }
}

pub fn check_peek_word<'a>(iter: &mut ParseIter) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token { kind: Word(_), .. }) => Ok(()),
        _ => Err(ParsingError()),
    }
}

pub fn get_ident<'a>(iter: &mut ParseIter) -> Result<String, ParserError> {
    match iter.next() {
        Some(Token {
            kind: Identifier(s),
            ..
        }) => Ok(s),
        _ => Err(ParsingError()),
    }
}

pub fn get_word<'a>(iter: &mut ParseIter) -> Result<i64, ParserError> {
    match iter.next() {
        Some(Token {
            kind: Word(val), ..
        }) => Ok(val),
        _ => Err(ParsingError()),
    }
}

// PARSING

pub fn parse(tokens: Vec<Token>) -> Result<Vec<AstNode>, ParserError> {
    let mut token_iter = tokens.into_iter().peekable();

    return parse_toplevel(&mut token_iter);
}

fn parse_toplevel<'a>(iter: &mut ParseIter) -> Result<Vec<AstNode>, ParserError> {
    let mut prog = Vec::new();

    while let Some(token) = iter.peek() {
        let node = match &token.kind {
            Identifier(_) => parse_global(iter),
            Fun() => parse_fun(iter),
            _ => return Err(ParserError::ParsingError()),
        };

        prog.push(node?);
    }

    return Ok(prog);
}

fn parse_global<'a>(iter: &mut ParseIter) -> Result<AstNode, ParserError> {
    let ident = match iter.next() {
        Some(Token {
            kind: Identifier(s),
            ..
        }) => s,
        _ => return Err(ParserError::ParsingError()),
    };

    match iter.next() {
        Some(Token { kind: Eq(), .. }) => (),
        _ => return Err(ParserError::ParsingError()),
    };

    let expr = parse_expr(iter)?;

    return Ok(AstNode::Global(ident, Box::from(expr)));
}

fn parse_fun<'a>(iter: &mut ParseIter) -> Result<AstNode, ParserError> {
    check_next(iter, Fun())?;
    let ident = get_ident(iter)?;
    let mut args = Vec::new();

    while check_peek_ident(iter).is_ok() {
        let ident = get_ident(iter)?;
        args.push(Param { ident });
    }

    check_next(iter, Eq())?;

    let expr = parse_expr(iter)?;

    Ok(Function(
        ident,
        Box::from(Fun {
            args,
            body: Box::from(expr),
        }),
    ))
}

fn parse_expr<'a>(iter: &mut ParseIter) -> Result<Expr, ParserError> {
    let word = match iter.next() {
        Some(Token {
            kind: Word(val), ..
        }) => val,
        _ => return Err(ParserError::ParsingError()),
    };

    return Ok(Expr::Word(word));
}
