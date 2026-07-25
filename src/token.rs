#[derive(Debug)]
pub enum Token {
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
}
