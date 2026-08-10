use std::collections::HashMap;

use crate::ast::Symbol;

pub type Reg = usize;

#[derive(Debug)]
pub enum Type {
    I64,
    Ptr(Box<Type>),
    FPtr(Box<Type>, Vec<Type>),
    Pack(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Word(i64),
    Ident(Reg),
    GIdent(Symbol),
    Null, // careful! no NPEs should be possible
}

#[derive(Debug)]
pub enum Bop {
    Add,
    Sub,
    Mult,
    Div,
    AddF,
    SubF,
    MultF,
    DivF,
    DivU,
    DivUF,
    Mod,
    ModU,
    BitOr,
    BitAnd,
    LogOr,
    LogAnd,
    Eq,
    Neq,
    Leq,
    Geq,
    Gt,
    Lt,
}

#[derive(Debug)]
pub enum StmtKind {
    Bop(Bop, Operand, Operand),
    Alloca(Type),
    Load(Operand, Operand),
    Store(Operand, Operand),
    Call(Operand, Vec<Operand>),
    Gep(Operand, Vec<Operand>),
    Phi(String, Reg, String, Reg),
}

#[derive(Debug)]
pub struct Stmt {
    pub dest: Option<Reg>, // we represent a reg as a int
    pub stmt: StmtKind,
}

#[derive(Debug)]
pub enum Term {
    Ret(Operand),
    Br(String),
    CBr(Reg, String, String),
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub term: Option<Term>,
}

#[derive(Debug)]
pub struct FParam {
    pub ident: Reg,
}

#[derive(Debug)]
pub struct Func {
    pub ident: Symbol,
    pub args: Vec<FParam>,
    pub entry: Block,
    pub body: HashMap<String, Block>,
}

#[derive(Debug)]
pub struct Global {
    pub ident: Symbol,
    pub val: Operand,
}

#[derive(Debug)]
pub enum Decl {
    FDecl(Func),
    GDecl(Global),
}

pub type Prog = Vec<Decl>;
