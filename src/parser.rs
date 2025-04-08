use crate::lexer::{Token, TokenKind};


#[derive(Debug)]
pub enum Expr {
    Constant(u32),
}

#[derive(Debug)]
pub enum Stmt {
    FnDefinition(String, Box<Stmt>),
    Program(Box<Stmt>),
    Return(Expr),
}

type ParseResult<T> = Result<T, String>;


fn expect<T: Iterator<Item = Token>>(expected: TokenKind, tokens: &mut T) -> ParseResult<Token> {
    match tokens.next() {
        Some(token) if token.kind == expected => {
            return Ok(token);
        }
        None => Err("Unexpectedly reached end of input".to_owned()),
        Some(unexpected) => Err(format!("Unexpectedly got '{}'", unexpected.value)),
    }
}

fn parse_statement<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Stmt> {
    expect(TokenKind::Return, tokens)?;
    let return_val: Result<Expr, String> = parse_expr(tokens);
    expect(TokenKind::Semicolon, tokens)?;
    return Ok(Stmt::Return(return_val?));
}

fn parse_expr<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Expr> {
    let value = expect(TokenKind::Constant, tokens)?;
    
    return Ok(Expr::Constant(value.value.parse().unwrap()));
}

fn parse_fn_definition<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Stmt> {
    expect(TokenKind::Int, tokens)?;
    let name = expect(TokenKind::Identifier, tokens)?;

    expect(TokenKind::LParen, tokens)?;
    expect(TokenKind::Void, tokens)?;
    expect(TokenKind::RParen, tokens)?;

    expect(TokenKind::LBrace, tokens)?;
    let body = parse_statement(tokens)?;
    expect(TokenKind::RBrace, tokens)?;

    return Ok(Stmt::FnDefinition(name.value, Box::new(body)));
}

pub fn parse<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Stmt> {
    let def = parse_fn_definition(tokens)?;

    return Ok(Stmt::Program(Box::new(def)));
}
