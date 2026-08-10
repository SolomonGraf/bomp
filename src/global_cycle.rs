use crate::ast::{
    AstNode::{self, Global},
    Expr, Lhs, Symbol,
};
use petgraph::graphmap::DiGraphMap;

type GlobalDeps = DiGraphMap<Symbol, ()>;

fn get_rhs_syms(e: &Expr) -> Vec<&Symbol> {
    match e {
        Expr::Word(..) => vec![],
        Expr::Lhs(Lhs::Ident(s)) => vec![s],
        Expr::Binop(_, e1, e2) => {
            let mut syms1 = get_rhs_syms(e1);
            let mut syms2 = get_rhs_syms(e2);
            syms1.append(&mut syms2);
            return syms1;
        }
        Expr::Unop(_, e) => get_rhs_syms(e),
        Expr::Let(_, e1, e2) => {
            let mut syms1 = get_rhs_syms(e1);
            let mut syms2 = get_rhs_syms(e2);
            syms1.append(&mut syms2);
            return syms1;
        }
        Expr::Cond(e1, e2, e3) => {
            let mut syms = get_rhs_syms(e1);
            syms.append(&mut get_rhs_syms(e2));
            syms.append(&mut get_rhs_syms(e3));
            return syms;
        }
        Expr::Call(Lhs::Ident(s), args) => {
            let mut syms = vec![s];
            args.iter().for_each(|e| syms.extend(get_rhs_syms(e)));
            syms
        }
    }
}

pub fn get_global_graph(prog: &Vec<AstNode>) -> GlobalDeps {
    let mut graph: DiGraphMap<Symbol, ()> = DiGraphMap::new();
    for node in prog {
        if let Global(src, e) = node {
            graph.add_node(*src);
            for dest in get_rhs_syms(&e) {
                graph.add_edge(*src, *dest, ());
            }
        }
    }

    return graph;
}
