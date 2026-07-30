use crate::constructs::Construct;
use crate::token::Token;

#[derive(Debug)]
pub enum LexerError {
    ReadAfterEnd(),
}

#[derive(Debug)]
pub enum ParserError {
    ParsingError(String, usize, usize),
}

pub fn p_err_tok(tok: Option<Token>) -> ParserError {
    let t = Option::unwrap_or(tok, Token::dummy());
    ParserError::ParsingError(String::from(t.file), t.row, t.col)
}

pub fn p_err_reftok(tok: Option<&Token>) -> ParserError {
    let t = match tok {
        Some(t) => t,
        None => &Token::dummy(),
    };

    ParserError::ParsingError(String::from(t.file), t.row, t.col)
}

#[derive(Debug)]
pub enum WCError {
    UnboundIdentifier(),
    UnexpectedType(Construct),
    MismatchedTypes(Construct, Construct),
}
