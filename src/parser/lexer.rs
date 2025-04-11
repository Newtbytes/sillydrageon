use std::iter::Peekable;
use std::str;

use super::ast::{Token, TokenKind};
use crate::error::CompilerError;

struct Scanner<'a> {
    src: Peekable<str::Chars<'a>>,
    consumed: String,
    offset: usize,
}

impl<'a> From<&'a str> for Scanner<'a> {
    fn from(value: &'a str) -> Self {
        Scanner {
            src: value.chars().peekable(),
            consumed: String::new(),
            offset: 0,
        }
    }
}

impl Scanner<'_> {
    fn eat(&mut self) -> Option<char> {
        let c = self.src.next()?;

        self.consumed.push(c);

        Some(c)
    }

    fn eat_if<P>(&mut self, mut predicate: P) -> Option<char>
    where
        P: FnMut(&char) -> bool,
    {
        let c = self.src.next_if(&mut predicate)?;

        self.consumed.push(c);

        Some(c)
    }

    fn eat_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(&char) -> bool,
    {
        while self.eat_if(&mut predicate).is_some() {
            continue;
        }
    }

    fn one_ahead(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn empty_consumed(&mut self) {
        self.offset += self.consumed.len();
        self.consumed.clear();
    }

    fn emit(&mut self, token: TokenKind) -> Token {
        let tok = Token {
            kind: token,
            value: self.consumed.clone(),
            offset: self.offset,
        };

        self.empty_consumed();

        tok
    }

    fn eat_identifer(&mut self) {
        self.eat_while(is_identifier);
    }

    fn eat_int_literal(&mut self) {
        self.eat_while(char::is_ascii_digit);
    }
}

fn is_identifier(c: &char) -> bool {
    matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_')
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        use TokenKind::*;

        // skip whitespace
        self.eat_while(|&c| c.is_whitespace());
        self.empty_consumed();

        let kind = match self.eat() {
            Some(c) => match c {
                '(' => LParen,
                ')' => RParen,

                '{' => LBrace,
                '}' => RBrace,

                ';' => Semicolon,

                'a'..='z' | 'A'..='Z' | '_' => {
                    self.eat_identifer();

                    // handle keywords
                    match self.consumed.as_str() {
                        "void" => Void,
                        "int" => Int,
                        "return" => Return,

                        _ => Identifier,
                    }
                }

                c if c.is_ascii_digit() => {
                    self.eat_int_literal();

                    match self.one_ahead() {
                        Some(c) if c.is_alphanumeric() => {
                            Error("Identifiers cannot start with a digit")
                        }
                        Some(_) | None => Constant,
                    }
                }

                _ => Error(""),
            },
            None => {
                return None;
            }
        };

        // synchronize by eating until synchronization point
        if let Error(_) = kind {
            self.eat_while(|c| !c.is_whitespace());
        }

        Some(self.emit(kind))
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, CompilerError> {
    Scanner::from(src)
        .map(|tok| match tok.kind {
            TokenKind::Error(_) => Err(CompilerError::Lexer(src.into(), tok)),
            _ => Ok(tok),
        })
        .collect()
}
