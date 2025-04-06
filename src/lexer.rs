use std::iter::{self, Peekable};
use std::str;

#[derive(Debug)]
pub enum Token {
    Identifier(String), // a-z
    Constant(u32),      // 0-9

    // Keywords
    Int,
    Void,
    Return,

    // ( )
    LParen,
    RParen,

    // { }
    LBrace,
    RBrace,

    Semicolon,
}

fn eat_while<'a, P>(
    src: &'a mut Peekable<str::Chars>,
    mut predicate: P,
) -> Peekable<impl Iterator<Item = char> + 'a>
where
    P: FnMut(&char) -> bool + 'a,
{
    iter::from_fn(move || src.next_if(&mut predicate)).peekable()
}

pub fn next_token(src: &mut Peekable<str::Chars>) -> Option<Token> {
    use Token::*;

    let token = match src.next() {
        Some(c) => match c {
            '(' => LParen,
            ')' => RParen,

            '{' => LBrace,
            '}' => RBrace,

            ';' => Semicolon,

            c if matches!(c, 'a'..='z' | 'A'..='Z' | '_') => {
                let rest: String = eat_while(
                    src,
                    |&c| matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_'),
                )
                .collect();
                let id = c.to_string() + &rest;

                // handle keywords
                match id.as_str() {
                    "void" => Void,
                    "int" => Int,
                    "return" => Return,

                    _ => Identifier(id),
                }
            }

            c if c.is_ascii_digit() => {
                let v: String = eat_while(src, |&c| c.is_ascii_digit()).collect();
                Constant((c.to_string() + &v).parse().unwrap())
            }

            c if c.is_whitespace() => next_token(src)?,

            _ => panic!("lexer error"),
        },
        None => {
            return None;
        }
    };

    return Some(token);
}

pub fn tokenize(src: String) -> Vec<Token> {
    let mut src = src.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(tok) = next_token(&mut src) {
        tokens.push(tok);
    }

    return tokens;
}
