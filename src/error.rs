use std::fmt::{self};
use std::process::Termination;

use crate::parser::ast::{Token, TokenKind};
use crate::src::Source;

pub enum CompilerError {
    IO(std::io::Error),
    Parser(String),
    Lexer(Source, Token),
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::IO(error)
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::IO(e) => write!(f, "I/O error: {}", e),
            CompilerError::Parser(msg) => write!(f, "Parse error: {}", msg),
            CompilerError::Lexer(src, tok) => {
                write!(
                    f,
                    "Lexer error: Unexpected token\n{} {}",
                    src.get_span(tok).unwrap(),
                    match tok.kind {
                        TokenKind::Error(msg) => msg,
                        _ => "",
                    }
                )
            }
        }
    }
}

impl fmt::Debug for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for CompilerError {}

impl Termination for CompilerError {
    fn report(self) -> std::process::ExitCode {
        todo!()
    }
}
