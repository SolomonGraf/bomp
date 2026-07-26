#[derive(Debug)]
pub enum AstNode {
    Global(String, Box<Expr>),
    Function(String, Box<Function>),
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<Param>,
    pub body: Box<Expr>,
}

#[derive(Debug)]
pub struct Param {
    pub ident: String,
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
    Ident(String),
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
