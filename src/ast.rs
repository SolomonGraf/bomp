use crate::constructs::Construct;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub u32, pub Namespace);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Namespace {
    Variant,
    Binding,
}

// Definition

#[derive(Debug)]
pub enum AstNode {
    Global(Symbol, Box<Expr>),
    Function(Symbol, Box<Function>),
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<Param>,
    pub body: Box<Expr>,
}

#[derive(Debug)]
pub struct Param {
    pub ident: Symbol,
    pub cons: Construct,
}

#[derive(Debug)]
pub enum Expr {
    Word(i64),
    Lhs(Lhs),
    Binop(Bop, Box<Expr>, Box<Expr>),
    Unop(Uop, Box<Expr>),
    Let(Lhs, Box<Expr>, Box<Expr>),
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Lhs {
    Ident(Symbol),
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
pub enum Uop {
    Not(),
    Neg(),
}