#[derive(Debug)]
pub enum LexerError {
    ReadAfterEnd(),
}

#[derive(Debug)]
pub enum ParserError {
    ParsingError(),
}
