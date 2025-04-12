mod ast;
mod lexer;

pub use ast::*;
pub use lexer::*;

type ParseResult<T> = Result<T, String>;

fn expect<T: Iterator<Item = Token>>(expected: TokenKind, tokens: &mut T) -> ParseResult<Token> {
    match tokens.next() {
        Some(token) if token.kind == expected => Ok(token),
        None => Err("Unexpectedly reached end of input".to_owned()),
        Some(unexpected) => Err(format!("Unexpectedly got '{}'", unexpected.value)),
    }
}

fn parse_statement<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Stmt> {
    expect(TokenKind::Return, tokens)?;
    let return_val: Result<Expr, String> = parse_expr(tokens);
    expect(TokenKind::Semicolon, tokens)?;
    Ok(Stmt::Return(return_val?))
}

fn parse_expr<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Expr> {
    let expr = match tokens.next() {
        Some(tok) => match tok.kind {
            TokenKind::Constant => Expr::Constant(tok.value.parse().unwrap()),
            TokenKind::Negate | TokenKind::Complement => {
                let inner_expr = parse_expr(tokens)?;
                Expr::Unary(todo!(), Box::new(inner_expr))
            }
            TokenKind::LParen => {
                tokens.next();
                let inner_expr = parse_expr(tokens)?;
                expect(TokenKind::RParen, tokens)?;
                inner_expr
            }
            TokenKind::Error(msg) => return Err(msg.to_owned()),
            _ => return Err("Malformed expression".to_owned())
        },
        None => todo!(),
    };

    Ok(expr)
}

fn parse_function<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Decl> {
    expect(TokenKind::Int, tokens)?;
    let name = expect(TokenKind::Identifier, tokens)?;

    expect(TokenKind::LParen, tokens)?;
    expect(TokenKind::Void, tokens)?;
    expect(TokenKind::RParen, tokens)?;

    expect(TokenKind::LBrace, tokens)?;
    let body = parse_statement(tokens)?;
    expect(TokenKind::RBrace, tokens)?;

    Ok(Decl::Function(name.value, Box::new(body)))
}

fn parse_program<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Program> {
    let func = parse_function(tokens)?;

    Ok(Program { body: func })
}

pub fn parse<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Program> {
    let prg = parse_program(tokens)?;

    if let Some(tok) = tokens.next() {
        return Err(format!(
            "Expected end of program, but got {:?} '{}'",
            tok.kind, tok.value
        ));
    }

    Ok(prg)
}
