use std::fmt;

use crate::parser::Token;


#[derive(Debug)]
pub enum ErrorKind {
    IoError(std::io::Error),
    ParseError(String),
    LexerError(Token)
}

impl From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        ErrorKind::IoError(error)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::IoError(e) => write!(f, "I/O error: {}", e),
            ErrorKind::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ErrorKind::LexerError(tok) => write!(f, "Lexer error: Unexpected token: {:?}", tok),
        }
    }
}

impl std::error::Error for ErrorKind {}
