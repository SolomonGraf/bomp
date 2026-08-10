use std::collections::HashMap;

use crate::ast::Lhs;
use crate::ast::{AstNode, Bop as ABop, Expr, Function, Symbol};
use crate::bir::Operand::GIdent;
use crate::bir::{Block, FParam, Func};
use crate::bir::{Bop as BBop, Decl, Global as BGlobal, Operand, Prog, Stmt, StmtKind, Term};
use crate::err::AstBirError;
use crate::global_cycle::get_global_graph;
use petgraph::algo::toposort;

// Compile to BIR

struct CompileCtx {
    next_uid: usize,
    sym_uids: HashMap<Symbol, Operand>,
}

impl CompileCtx {
    pub fn new() -> Self {
        Self {
            next_uid: 0,
            sym_uids: HashMap::new(),
        }
    }

    #[inline]
    pub fn next_id(self: &mut Self) -> usize {
        let tmp = self.next_uid;
        self.next_uid += 1;
        tmp
    }

    #[inline]
    pub fn get_uid(self: &Self, sym: &Symbol) -> Option<Operand> {
        self.sym_uids.get(sym).copied()
    }

    #[inline]
    pub fn has_uid(self: &Self, sym: &Symbol) -> bool {
        self.sym_uids.contains_key(sym)
    }

    #[inline]
    pub fn add_local(self: &mut Self, sym: Symbol, reg: Operand) -> Option<Operand> {
        self.sym_uids.insert(sym, reg)
    }
}

enum Elem {
    S(Stmt),
    L(String),
    T(Term),
}

pub fn compile(ast: Vec<AstNode>) -> Result<Prog, AstBirError> {
    let global_deps = get_global_graph(&ast);
    let mut gdecls: HashMap<Symbol, Expr> = HashMap::new();
    let mut fdecls: HashMap<Symbol, Function> = HashMap::new();

    for node in ast {
        match node {
            AstNode::Global(s, n) => {
                gdecls.insert(s, *n);
            }
            AstNode::Function(s, fun) => {
                fdecls.insert(s, *fun);
            }
        }
    }

    let Ok(order) = toposort(&global_deps, None) else {
        panic!("impossible")
    };

    let mut global_vals = HashMap::new();

    let mut prog = Vec::new();

    for s in order {
        let op = fold_cst(
            gdecls.get(&s).expect("should be there 33"),
            &mut global_vals,
        );
        global_vals.insert(s, op);
        prog.push(Decl::GDecl(BGlobal { ident: s, val: op }));
    }

    let mut ctx = CompileCtx::new();

    for k in global_vals.keys() {
        ctx.add_local(*k, GIdent(*k));
    }

    for k in fdecls.keys() {
        ctx.add_local(*k, GIdent(*k));
    }

    for (sym, fun) in fdecls {
        prog.push(Decl::FDecl(compile_fun(&mut ctx, sym, fun)?));
    }

    return Ok(prog);
}

fn fold_cst(e: &Expr, g: &mut HashMap<Symbol, Operand>) -> Operand {
    match e {
        Expr::Word(s) => Operand::Word(*s),
        Expr::Lhs(Lhs::Ident(s)) => *g.get(&s).unwrap(),
        Expr::Binop(b, e1, e2) => todo!("bop fold not done"),
        Expr::Cond(c, e1, e2) => todo!("cond not done"),
        Expr::Let(Lhs::Ident(s), e1, e2) => {
            let mut g = g.clone();
            let cst = fold_cst(e1, &mut g);
            g.insert(*s, cst);
            fold_cst(e1, &mut g)
        }
        Expr::Unop(u, e) => todo!("unop fold not done"),
        // should run lazily
        Expr::Call(func, args) => todo!("call fold not done"),
    }
}

fn compile_fun(ctx: &mut CompileCtx, s: Symbol, fun: Function) -> Result<Func, AstBirError> {
    let mut params = vec![];

    for a in fun.args {
        let ident = ctx.next_id();
        ctx.add_local(a.ident, Operand::Ident(ident));
        params.push(FParam { ident });
    }

    let (ent, blks) = blocks_of_stream(compile_expr(ctx, *fun.body)?)?;

    Ok(Func {
        ident: s,
        args: params,
        entry: ent,
        body: blks,
    })
}

// returns map of labels to blocks, entry label, and exit label.
// INVARIANT: Only one return. We can optimize in the future
fn compile_expr_aux(ctx: &mut CompileCtx, e: Expr) -> Result<(Vec<Elem>, Operand), AstBirError> {
    Ok(match e {
        Expr::Word(i) => (vec![], Operand::Word(i)),
        Expr::Lhs(Lhs::Ident(s)) => (
            vec![],
            ctx.get_uid(&s).ok_or(AstBirError::UnboundSymbol(s))?,
        ),
        Expr::Binop(b, e1, e2) => {
            let (mut v1, o1) = compile_expr_aux(ctx, *e1)?;
            let (mut v2, o2) = compile_expr_aux(ctx, *e2)?;
            let dest = ctx.next_id();
            v1.append(&mut v2);
            v1.push(Elem::S(Stmt {
                dest: Some(dest),
                stmt: StmtKind::Bop(compile_bop(b), o1, o2),
            }));
            (v1, Operand::Ident(dest))
        }
        Expr::Unop(u, e) => todo!("cmp unop"),
        Expr::Let(Lhs::Ident(s), e1, e2) => todo!("cmp let"),
        Expr::Cond(c, iff, elze) => todo!("cmp cond"),
        Expr::Call(func, args) => todo!("cmp call"),
    })
}

fn compile_expr(ctx: &mut CompileCtx, e: Expr) -> Result<Vec<Elem>, AstBirError> {
    let (mut v, o) = compile_expr_aux(ctx, e)?;
    v.push(Elem::T(Term::Ret(o)));
    Ok(v)
}

fn compile_bop(b: ABop) -> BBop {
    match b {
        ABop::Add => BBop::Add,
        ABop::Sub => todo!(),
        ABop::Mult => todo!(),
        ABop::Div => todo!(),
        ABop::AddF => todo!(),
        ABop::SubF => todo!(),
        ABop::MultF => todo!(),
        ABop::DivF => todo!(),
        ABop::DivU => todo!(),
        ABop::DivUF => todo!(),
        ABop::Mod => todo!(),
        ABop::ModU => todo!(),
        ABop::BitOr => todo!(),
        ABop::BitAnd => todo!(),
        ABop::BitXor => todo!(),
        ABop::LogOr => todo!(),
        ABop::LogAnd => todo!(),
        ABop::Eq => todo!(),
        ABop::Neq => todo!(),
        ABop::Leq => todo!(),
        ABop::Geq => todo!(),
        ABop::Gt => todo!(),
        ABop::Lt => todo!(),
    }
}

fn blocks_of_stream(stream: Vec<Elem>) -> Result<(Block, HashMap<String, Block>), AstBirError> {
    let mut elem_stream = stream.into_iter().peekable();

    let mut ent_blk = Block {
        stmts: Vec::new(),
        term: None,
    };

    let mut blks = HashMap::new();

    loop {
        match elem_stream.peek() {
            None => {
                return Ok((ent_blk, HashMap::new()));
            }
            Some(Elem::L(..)) => break,
            Some(Elem::S(..)) => {
                let Some(Elem::S(s)) = elem_stream.next() else {
                    return Err(AstBirError::DevBlockingError());
                };
                ent_blk.stmts.push(s);
            }
            Some(Elem::T(..)) => {
                let Some(Elem::T(t)) = elem_stream.next() else {
                    return Err(AstBirError::DevBlockingError());
                };
                ent_blk.term = Some(t);
            }
        }
    }

    let mut next_lbl = None;
    let mut next_stmts = Vec::new();

    while let Some(e) = elem_stream.next() {
        match e {
            Elem::L(s) => {
                if let Some(lbl) = next_lbl {
                    blks.insert(
                        lbl,
                        Block {
                            stmts: next_stmts,
                            term: None,
                        },
                    );
                };
                next_lbl = Some(s);
                next_stmts = vec![];
            }
            Elem::S(stmt) => {
                next_stmts.push(stmt);
            }
            Elem::T(term) => {
                if let Some(lbl) = next_lbl {
                    blks.insert(
                        lbl,
                        Block {
                            stmts: next_stmts,
                            term: Some(term),
                        },
                    );
                    next_lbl = None;
                    next_stmts = Vec::new();
                } else {
                    return Err(AstBirError::DevBlockingError());
                }
            }
        }
    }

    Ok((ent_blk, blks))
}
