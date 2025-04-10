use std::fmt;

use crate::parser::Token;
use crate::src::Source;

#[derive(Debug)]
pub enum CompilerError {
    IoError(std::io::Error),
    ParseError(String),
    LexerError(Source, Token),
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::IoError(error)
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::IoError(e) => write!(f, "I/O error: {}", e),
            CompilerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CompilerError::LexerError(src, tok) => {
                write!(
                    f,
                    "Lexer error: Unexpected token:\n{}",
                    src.get_span(tok).unwrap()
                )
            }
        }
    }
}

impl std::error::Error for CompilerError {}
