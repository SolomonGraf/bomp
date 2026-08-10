use crate::token::TokenKind::Identifier;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub file: &'a str,
    pub row: usize,
    pub col: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, file: &'a str, row: usize, col: usize) -> Self {
        Self {
            kind: kind,
            file: file,
            row: row,
            col: col,
        }
    }

    pub fn dummy() -> Token<'a> {
        Self {
            kind: Identifier(String::new()),
            file: "NO FILE",
            row: 0,
            col: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Plus(),
    Minus(),
    Word(i64),
    Identifier(String),
    Fun(),
    Arrow(),
    Eq(),
    Xor(),
    And(),
    Not(),
    Or(),
    In(),
    LParam(),
    RParam(),
    LBrace(),
    RBrace(),
    LBracket(),
    RBracket(),
    Colon(),
    TWord(),
    TPack(),
    Of(),
    If(),
    Else(),
    Then(),
    Let(),
    OrOr(),
    AndAnd(),
}
