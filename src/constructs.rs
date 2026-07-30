use crate::ast::{Function, Param, Symbol};
use crate::err::WCError;
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
    Structure(Vec<(Symbol, Construct)>),
    Fun(Box<Construct>, Vec<Construct>),
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
        g.insert(**s, wc_expr(e, &g)?);
        Ok(())
    })?;

    fdecls.iter().try_for_each(|f| {
        let (s, f) = f;
        g.insert(**s, wc_fun(f, &g)?);
        Ok(())
    })?;

    Ok(())
}

fn wc_fun(fun: &Function, ctx: &HashMap<Symbol, Construct>) -> Result<Construct> {
    let mut f_ctx = ctx.clone();

    let Function { args, body } = fun;

    args.iter().for_each(|Param { ident, cons }| {
        f_ctx.insert(*ident, cons.clone());
    });

    wc_expr(body, &f_ctx)
}

pub fn wc_expr(e: &Expr, w_env: &HashMap<Symbol, Construct>) -> Result<Construct> {
    let mut env = w_env.clone();
    wc_expr_aux(e, &mut env)
}

pub fn wc_expr_aux(e: &Expr, w_env: &mut HashMap<Symbol, Construct>) -> Result<Construct> {
    match e {
        Expr::Word(_) => Ok(Construct::Word()),
        Expr::Lhs(Lhs::Ident(s)) => w_env.get(&s).cloned().ok_or(WCError::UnboundIdentifier()),
        Expr::Binop(bop, e1, e2) => todo!("cons of bop"),
        Expr::Unop(uop, e) => todo!("cons of unop"),
        Expr::Let(lhs, e1, e2) => {
            match lhs {
                Lhs::Ident(s) => {
                    let cons = wc_expr_aux(e, w_env)?;
                    w_env.insert(*s, cons)
                }
            };
            wc_expr_aux(e2, w_env)
        }
        Expr::Cond(c, if_expr, else_expr) => {
            if wc_expr_aux(c, w_env)? != Construct::Word() {
                return Err(WCError::UnexpectedType(Construct::Word()));
            }
            let if_cons = wc_expr_aux(if_expr, w_env)?;
            let else_cons = wc_expr_aux(else_expr, w_env)?;
            if if_cons != else_cons {
                return Err(WCError::MismatchedTypes(if_cons, else_cons));
            }
            return Ok(if_cons);
        }
    }
}
