use std::fmt;

use crate::parser::Token;

#[derive(Debug)]
pub enum CompilerError {
    IoError(std::io::Error),
    ParseError(String),
    LexerError(Token),
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
            CompilerError::LexerError(tok) => write!(f, "Lexer error: Unexpected token: {:?}", tok),
        }
    }
}

impl std::error::Error for CompilerError {}
