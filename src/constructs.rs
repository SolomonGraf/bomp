use crate::ast::{Bop, Function, Param, Symbol};
use crate::constructs::Construct::Fun;
use crate::err::WCError::{self, ArgumentConstructMismatch, UnexpectedConstruct};
use std::collections::HashMap;

type Result<T> = core::result::Result<T, WCError>;

use crate::ast::{
    AstNode,
    Expr::{self},
    Lhs,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Construct {
    Word(),
    Pack(i64),
    // Structure(Vec<(Symbol, Construct)>),
    Fun(Vec<Construct>),
    Alias(Symbol),
}

pub fn width_check(ast: &Vec<AstNode>) -> Result<()> {
    let mut g: HashMap<Symbol, Construct> = HashMap::new();

    let mut gdecls = Vec::new();
    let mut fdecls = Vec::new();

    for node in ast {
        match node {
            AstNode::Function(s, f) => {
                fdecls.push((s, f));
            }
            AstNode::Global(s, e) => {
                gdecls.push((s, e));
            }
        }
    }

    gdecls.iter().try_for_each(|n| {
        let (s, e) = n;
        let e = wc_expr(e, &mut g)?;
        g.insert(**s, e);
        Ok(())
    })?;

    fdecls.iter().try_for_each(|f| {
        let (s, f) = f;
        let e = wc_fun(f, &mut g)?;
        g.insert(**s, e);
        Ok(())
    })?;

    Ok(())
}

fn wc_fun(fun: &Function, ctx: &mut HashMap<Symbol, Construct>) -> Result<Construct> {
    let mut arg_cons = Vec::new();

    let Function { args, body } = fun;

    args.iter().for_each(|Param { ident, cons }| {
        ctx.insert(*ident, cons.clone());
        arg_cons.push(cons.clone());
    });

    let res = wc_expr(body, ctx)?;
    arg_cons.push(res);

    args.iter().for_each(|Param { ident, .. }| {
        ctx.remove(ident);
    });

    Ok(Construct::Fun(arg_cons))
}

fn wc_expr(e: &Expr, w_env: &mut HashMap<Symbol, Construct>) -> Result<Construct> {
    match e {
        Expr::Word(_) => Ok(Construct::Word()),
        Expr::Lhs(Lhs::Ident(s)) => w_env.get(&s).cloned().ok_or(WCError::UnboundIdentifier()),
        Expr::Binop(bop, e1, e2) => wc_bop(bop, e1, e2, w_env),
        Expr::Unop(uop, e) => todo!("cons of unop"),
        Expr::Let(lhs, e1, e2) => match lhs {
            Lhs::Ident(s) => {
                let cons = wc_expr(e1, w_env)?;
                w_env.insert(*s, cons);
                let ret = wc_expr(e2, w_env);
                w_env.remove(s);
                ret
            }
        },
        Expr::Cond(c, if_expr, else_expr) => {
            if wc_expr(c, w_env)? != Construct::Word() {
                return Err(WCError::UnexpectedConstruct(Construct::Word()));
            }
            let if_cons = wc_expr(if_expr, w_env)?;
            let else_cons = wc_expr(else_expr, w_env)?;
            if if_cons != else_cons {
                return Err(WCError::MismatchedConstructs(if_cons, else_cons));
            }
            return Ok(if_cons);
        }
        Expr::Call(Lhs::Ident(s), args) => {
            let mut tys = match w_env.get(s).ok_or(WCError::UnboundIdentifier())? {
                Fun(tys) => Ok(tys.clone()),
                s => Err(UnexpectedConstruct(s.clone())),
            }?;

            let arg_tys: Vec<Construct> = args
                .into_iter()
                .map(|a| wc_expr(a, w_env))
                .collect::<Result<_>>()?;

            if tys.starts_with(&arg_tys) {
                tys.drain(0..arg_tys.len());
            } else {
                return Err(ArgumentConstructMismatch(tys, arg_tys));
            }

            if tys.len() < 1 {
                return Err(ArgumentConstructMismatch(tys, arg_tys));
            } else if tys.len() == 1 {
                return Ok(tys.get(0).unwrap().clone());
            } else {
                return Ok(Fun(tys));
            }
        }
    }
}

fn wc_bop(
    bop: &Bop,
    e1: &Expr,
    e2: &Expr,
    w_env: &mut HashMap<Symbol, Construct>,
) -> Result<Construct> {
    let w1 = wc_expr(e1, w_env)?;
    let w2 = wc_expr(e2, w_env)?;

    match bop {
        Bop::Add
        | Bop::Sub
        | Bop::Mult
        | Bop::Div
        | Bop::AddF
        | Bop::SubF
        | Bop::MultF
        | Bop::DivF
        | Bop::DivU
        | Bop::DivUF
        | Bop::Mod
        | Bop::ModU
        | Bop::BitOr
        | Bop::BitAnd
        | Bop::BitXor
        | Bop::LogOr
        | Bop::LogAnd
        | Bop::Eq
        | Bop::Neq
        | Bop::Leq
        | Bop::Geq
        | Bop::Gt
        | Bop::Lt => {
            if w1 != Construct::Word() || w2 != Construct::Word() {
                Err(WCError::MismatchedConstructs(w1, w2))
            } else {
                Ok(Construct::Word())
            }
        }
    }
}
