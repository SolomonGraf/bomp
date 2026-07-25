#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    file: &'a str,
    row: usize,
    col: usize,
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
}

#[derive(Debug)]
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
    Or(),
    In(),
}
