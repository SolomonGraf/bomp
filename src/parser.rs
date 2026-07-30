use crate::ast::{AstNode, Expr, Function, Namespace, Param, Symbol};
use crate::constructs::Construct;
use crate::err::{ParserError, p_err_reftok, p_err_tok};
use crate::token::{
    Token,
    TokenKind::{self, *},
};
use std::collections::HashMap;
use std::iter::Peekable;

// HELPERS

type ParseIter<'a> = Peekable<std::vec::IntoIter<Token<'a>>>;

pub fn check_next<'a>(iter: &mut ParseIter, kind_exp: TokenKind) -> Result<(), ParserError> {
    match iter.next() {
        Some(Token {
            kind: kind_actual, ..
        }) if kind_actual == kind_exp => Ok(()),
        t => Err(p_err_tok(t)),
    }
}

pub fn check_peek<'a>(iter: &mut ParseIter, kind_exp: TokenKind) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token {
            kind: kind_actual, ..
        }) if *kind_actual == kind_exp => Ok(()),
        t => Err(p_err_reftok(t)),
    }
}

pub fn check_peek_ident<'a>(iter: &mut ParseIter) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token {
            kind: Identifier(_),
            ..
        }) => Ok(()),
        t => Err(p_err_reftok(t)),
    }
}

pub fn check_peek_word<'a>(iter: &mut ParseIter) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token { kind: Word(_), .. }) => Ok(()),
        t => Err(p_err_reftok(t)),
    }
}

pub fn get_ident<'a>(iter: &mut ParseIter) -> Result<String, ParserError> {
    match iter.next() {
        Some(Token {
            kind: Identifier(s),
            ..
        }) => Ok(s),
        tok => Err(p_err_tok(tok)),
    }
}

pub fn get_word<'a>(iter: &mut ParseIter) -> Result<i64, ParserError> {
    match iter.next() {
        Some(Token {
            kind: Word(val), ..
        }) => Ok(val),
        tok => Err(p_err_tok(tok)),
    }
}

// Symbol Table

struct Interner {
    strings: Vec<String>,
    map: HashMap<String, Symbol>,
}

impl Interner {
    fn new() -> Self {
        Self {
            strings: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn intern(&mut self, s: &str, namespace: Namespace) -> Symbol {
        if let Some(&sym) = self.map.get(s) {
            return sym;
        }
        let sym = Symbol(self.strings.len() as u32, namespace);
        self.strings.push(s.to_owned());
        self.map.insert(s.to_owned(), sym);
        sym
    }
}

// PARSING

struct ParseCtx<'a> {
    iter: &'a mut ParseIter<'a>,
    interner: &'a mut Interner,
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<AstNode>, ParserError> {
    let mut parse_iter = tokens.into_iter().peekable();
    let mut interner = Interner::new();

    let mut ctx = ParseCtx {
        iter: &mut parse_iter,
        interner: &mut interner,
    };

    return parse_toplevel(&mut ctx);
}

fn parse_toplevel<'a>(ctx: &mut ParseCtx<'a>) -> Result<Vec<AstNode>, ParserError> {
    let mut prog = Vec::new();

    while let Some(token) = ctx.iter.peek() {
        let node = match token.kind {
            Identifier(_) => parse_global(ctx),
            Fun() => parse_fun(ctx),
            _ => return Err(p_err_reftok(ctx.iter.peek())),
        };

        prog.push(node?);
    }

    return Ok(prog);
}

fn parse_global<'a>(ctx: &mut ParseCtx) -> Result<AstNode, ParserError> {
    let ident = get_ident(ctx.iter)?;
    check_next(ctx.iter, Eq())?;
    let expr = parse_expr(ctx)?;

    return Ok(AstNode::Global(
        ctx.interner.intern(&ident, Namespace::Binding),
        Box::from(expr),
    ));
}

fn parse_fun<'a>(ctx: &mut ParseCtx) -> Result<AstNode, ParserError> {
    check_next(ctx.iter, Fun())?;
    let ident = get_ident(ctx.iter)?;
    let mut args = Vec::new();

    while !check_peek(ctx.iter, Eq()).is_ok() {
        check_next(ctx.iter, LParam())?;
        let param_ident = get_ident(ctx.iter)?;
        check_next(ctx.iter, Colon())?;

        let param_cons = parse_cons(ctx)?;

        check_next(ctx.iter, RParam())?;

        args.push(Param {
            ident: ctx.interner.intern(&param_ident, Namespace::Binding),
            cons: param_cons,
        });
    }

    check_next(ctx.iter, Eq())?;

    let expr = parse_expr(ctx)?;

    Ok(AstNode::Function(
        ctx.interner.intern(&ident, Namespace::Binding),
        Box::from(Function {
            args,
            body: Box::from(expr),
        }),
    ))
}

fn parse_expr<'a>(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    let word = get_word(ctx.iter)?;

    return Ok(Expr::Word(word));
}

fn parse_cons<'a>(ctx: &mut ParseCtx) -> Result<Construct, ParserError> {
    // assume that if ident, its a construct alias

    if check_peek_ident(ctx.iter).is_ok() {
        let cons_alias = get_ident(ctx.iter)?;
        return Ok(Construct::Alias(
            ctx.interner.intern(&cons_alias, Namespace::Binding),
        ));
    };

    match ctx.iter.next() {
        Some(Token { kind: TWord(), .. }) => Ok(Construct::Word()),
        Some(Token { kind: TPack(), .. }) => {
            check_next(ctx.iter, LBracket())?;
            let size = get_word(ctx.iter)?;
            check_next(ctx.iter, RBracket())?;
            Ok(Construct::Pack(size))
        }
        // No support right for structures, requires LR parsing
        tok => Err(p_err_tok(tok)),
    }
}
