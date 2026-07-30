use std::collections::HashMap;

use crate::bir::{Decl, Global as BGlobal, Prog};
use crate::global_cycle::get_global_graph;
use petgraph::algo::toposort;
use crate::ast::{AstNode, Symbol, Expr, Function};

// Compile to BIR

struct CompileCtx {
    next_uid: usize,
}

pub fn compile(ast: Vec<AstNode>) -> Prog {
    let global_deps = get_global_graph(&ast);
    let mut gdecls: HashMap<Symbol, Expr> = HashMap::new();
    let mut fdecls: HashMap<Symbol, Function> = HashMap::new();

    for node in ast {
        match node {
            AstNode::Global(s, n) => {gdecls.insert(s, *n);},
            AstNode::Function(s, fun ) => {fdecls.insert(s, *fun);}

        }
    };

    let Ok(order) = toposort(&global_deps, None) else {panic!("impossible")};

    for s in order {
        
    }

    return Vec::new();
}
