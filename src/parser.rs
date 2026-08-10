use crate::ast::Namespace::{Binding, Global};
use crate::ast::{AstNode, Bop, Expr, Function, Lhs, Namespace, Param, Symbol, Uop};
use crate::constructs::Construct;
use crate::err::{ParserError, p_err_reftok, p_err_tok};
use crate::token::{
    Token,
    TokenKind::{self, *},
};
use std::collections::HashMap;
use std::iter::Peekable;

// HELPERS

// Binary Ops Helper Functions
fn precedence(bop: Bop) -> usize {
    match bop {
        Bop::Add | Bop::Sub | Bop::AddF | Bop::SubF => 1,
        Bop::Mult
        | Bop::Div
        | Bop::MultF
        | Bop::DivF
        | Bop::DivU
        | Bop::DivUF
        | Bop::Mod
        | Bop::ModU => 2,
        Bop::BitOr | Bop::BitAnd | Bop::BitXor => 3,
        Bop::LogOr | Bop::LogAnd => 4,
        Bop::Leq | Bop::Geq | Bop::Gt | Bop::Lt => 5,
        Bop::Eq | Bop::Neq => 6,
    }
}
enum Fix {
    Prefix,
    Postfix,
    Infix,
}

// Parsing Helper Functions

type ParseIter<'a> = Peekable<std::vec::IntoIter<Token<'a>>>;

fn check_next(iter: &mut ParseIter, kind_exp: TokenKind) -> Result<(), ParserError> {
    match iter.next() {
        Some(Token {
            kind: kind_actual, ..
        }) if kind_actual == kind_exp => Ok(()),
        t => Err(p_err_tok(t)),
    }
}

fn check_peek(iter: &mut ParseIter, kind_exp: TokenKind) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token {
            kind: kind_actual, ..
        }) if *kind_actual == kind_exp => Ok(()),
        t => Err(p_err_reftok(t)),
    }
}

fn check_peek_ident(iter: &mut ParseIter) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token {
            kind: Identifier(_),
            ..
        }) => Ok(()),
        t => Err(p_err_reftok(t)),
    }
}

fn check_peek_word(iter: &mut ParseIter) -> Result<(), ParserError> {
    match iter.peek() {
        Some(Token { kind: Word(_), .. }) => Ok(()),
        t => Err(p_err_reftok(t)),
    }
}

fn get_ident(iter: &mut ParseIter) -> Result<String, ParserError> {
    match iter.next() {
        Some(Token {
            kind: Identifier(s),
            ..
        }) => Ok(s),
        tok => Err(p_err_tok(tok)),
    }
}

fn get_word(iter: &mut ParseIter) -> Result<i64, ParserError> {
    match iter.next() {
        Some(Token {
            kind: Word(val), ..
        }) => Ok(val),
        tok => Err(p_err_tok(tok)),
    }
}

fn get_peek<'a>(iter: &mut ParseIter<'a>) -> Option<Token<'a>> {
    return iter.peek().map(|u| u.clone());
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
        if let Some(&sym) = self.map.get(s)
            && sym.1 == namespace
        {
            return sym;
        }
        let sym = Symbol(self.strings.len() as u32, namespace);
        self.strings.push(s.to_owned());
        self.map.insert(s.to_owned(), sym);
        sym
    }

    fn resolve(&mut self, s: &str, namespace: Namespace) -> Result<Symbol, ParserError> {
        if let Some(&sym) = self.map.get(s)
            && sym.1 == namespace
        {
            return Ok(sym);
        } else {
            return Err(p_err_tok(None));
        }
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

fn parse_toplevel(ctx: &mut ParseCtx) -> Result<Vec<AstNode>, ParserError> {
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

fn parse_global(ctx: &mut ParseCtx) -> Result<AstNode, ParserError> {
    let ident = get_ident(ctx.iter)?;
    check_next(ctx.iter, Eq())?;
    let expr = parse_expr(ctx)?;

    return Ok(AstNode::Global(
        ctx.interner.intern(&ident, Namespace::Global),
        Box::from(expr),
    ));
}

fn parse_fun(ctx: &mut ParseCtx) -> Result<AstNode, ParserError> {
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
        ctx.interner.intern(&ident, Namespace::Global),
        Box::from(Function {
            args,
            body: Box::from(expr),
        }),
    ))
}

fn bop_of_tok(tok: &Token) -> Option<Bop> {
    match tok.kind {
        Plus() => Some(Bop::Add),
        Minus() => Some(Bop::Sub),
        Eq() => Some(Bop::Eq),
        Xor() => Some(Bop::BitXor),
        And() => Some(Bop::BitAnd),
        Or() => Some(Bop::BitOr),
        OrOr() => Some(Bop::LogOr),
        AndAnd() => Some(Bop::LogAnd),
        _ => None,
    }
}

#[inline]
fn starts_atom(tok: &Token) -> bool {
    matches!(
        tok.kind,
        TokenKind::Identifier(_) | TokenKind::Word(_) | LParam()
    )
}

// atom := ident | word | '(' expr ')'
// Used for arguments and grouped expressions. Note: a bare identifier here is
// just a variable reference, not a call — call syntax is only recognized in
// parse_apply. So `f (g x) y` requires the parens around `g x`, same as OCaml.
fn parse_atom(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    if check_peek(ctx.iter, LParam()).is_ok() {
        check_next(ctx.iter, LParam())?;
        let e = parse_expr(ctx)?;
        check_next(ctx.iter, RParam())?;
        return Ok(e);
    }

    let tok = ctx.iter.next();
    match tok {
        Some(Token {
            kind: TokenKind::Identifier(s),
            ..
        }) => Ok(Expr::Lhs(Lhs::Ident(
            ctx.interner
                .resolve(&s, Binding)
                .or(ctx.interner.resolve(&s, Global))?,
        ))),
        Some(Token {
            kind: TokenKind::Word(val),
            ..
        }) => Ok(Expr::Word(val)),
        t => Err(p_err_tok(t)),
    }
}

// apply := ident atom+   (function call, e.g. f x y -> Call(f, [x, y]))
//        | atom          (plain variable / literal / parenthesized expr)
fn parse_apply(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    if check_peek_ident(ctx.iter).is_ok() {
        let ident = get_ident(ctx.iter)?;
        let sym = ctx
            .interner
            .resolve(&ident, Binding)
            .or(ctx.interner.resolve(&ident, Global))?;

        let mut args = Vec::new();
        while let Some(tok) = get_peek(ctx.iter) {
            if !starts_atom(&tok) {
                break;
            }
            args.push(parse_atom(ctx)?);
        }

        return if args.is_empty() {
            Ok(Expr::Lhs(Lhs::Ident(sym)))
        } else {
            Ok(Expr::Call(Lhs::Ident(sym), args))
        };
    }

    parse_atom(ctx)
}

// unary := '-' unary | apply
fn parse_unary(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    if check_peek(ctx.iter, Minus()).is_ok() {
        check_next(ctx.iter, Minus())?;
        let operand = parse_unary(ctx)?; // recursive: allows `--x`
        return Ok(Expr::Unop(Uop::Neg(), Box::new(operand)));
    }

    parse_apply(ctx)
}

fn parse_primary(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    parse_unary(ctx)
}

fn parse_pratt(ctx: &mut ParseCtx, mut lhs: Expr, min_pred: usize) -> Result<Expr, ParserError> {
    while let Some(tok) = get_peek(ctx.iter) {
        let bop = match bop_of_tok(&tok) {
            Some(b) => b,
            None => break,
        };
        let pred = precedence(bop);
        if pred < min_pred {
            break;
        }
        ctx.iter.next(); // consume operator

        let mut rhs = parse_primary(ctx)?;

        while let Some(next_tok) = get_peek(ctx.iter) {
            let next_bop = match bop_of_tok(&next_tok) {
                Some(b) => b,
                None => break,
            };
            let next_pred = precedence(next_bop);
            if next_pred > pred {
                rhs = parse_pratt(ctx, rhs, next_pred)?;
            } else {
                break;
            }
        }

        lhs = Expr::Binop(bop, Box::new(lhs), Box::new(rhs));
    }

    Ok(lhs)
}

fn parse_expr(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    if check_peek(ctx.iter, If()).is_ok() {
        return parse_cond(ctx);
    }

    if check_peek(ctx.iter, TokenKind::Let()).is_ok() {
        check_next(ctx.iter, TokenKind::Let())?;
        let ident = get_ident(ctx.iter)?;
        check_next(ctx.iter, Eq())?;
        let e1 = parse_expr(ctx)?;
        check_next(ctx.iter, In())?;
        let e2 = parse_expr(ctx)?;
        return Ok(Expr::Let(
            Lhs::Ident(ctx.interner.intern(&ident, Namespace::Binding)),
            Box::from(e1),
            Box::from(e2),
        ));
    }

    let lhs = parse_primary(ctx)?;
    parse_pratt(ctx, lhs, 0)
}

fn parse_cons(ctx: &mut ParseCtx) -> Result<Construct, ParserError> {
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

fn parse_cond(ctx: &mut ParseCtx) -> Result<Expr, ParserError> {
    check_next(ctx.iter, If())?;
    let c = parse_expr(ctx)?;
    check_next(ctx.iter, Then())?;
    let iff = parse_expr(ctx)?;
    check_next(ctx.iter, Else())?;
    let elze = parse_expr(ctx)?;
    Ok(Expr::Cond(Box::new(c), Box::new(iff), Box::new(elze)))
}
