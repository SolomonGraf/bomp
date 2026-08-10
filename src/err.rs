use crate::ast::Symbol;
use crate::constructs::Construct;
use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    ReadAfterEnd(),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::ReadAfterEnd() => write!(f, "LexerError: read after end occurred"),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    ParsingError(String, usize, usize),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::ParsingError(s, row, col) => write!(f, "Parser error at {s}:{row}:{col}"),
        }
    }
}

#[derive(Debug)]
pub enum WCError {
    UnboundIdentifier(),
    UnexpectedConstruct(Construct),
    MismatchedConstructs(Construct, Construct),
    ArgumentConstructMismatch(Vec<Construct>, Vec<Construct>),
}

impl fmt::Display for WCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WCError::UnboundIdentifier() => write!(f, "WCError: Unbound identifier"),
            WCError::MismatchedConstructs(c1, c2) => {
                write!(f, "WCError: mismatched construct {c1:?} {c2:?}")
            }
            WCError::UnexpectedConstruct(c) => write!(f, "WCError: unexpected construct {c:?}"),
            WCError::ArgumentConstructMismatch(v1, v2) => {
                write!(f, "WCError: argument construct mismatch {v1:?} {v2:?}")
            }
        }
    }
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
pub enum AstBirError {
    UnboundSymbol(Symbol),
    DevBlockingError(),
}

impl fmt::Display for AstBirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstBirError::UnboundSymbol(s) => write!(f, "Unbound symbol {s:?}"),
            AstBirError::DevBlockingError() => write!(f, "Error while stream => blocks"),
        }
    }
}
