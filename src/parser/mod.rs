mod ast;
mod lexer;

use std::iter::{self};

pub use ast::*;
pub use lexer::*;

type ParseResult<T> = Result<T, String>;

struct Parser<'a, I: Iterator<Item = Token>> {
    tokens: &'a mut iter::Peekable<I>,
}

impl<I: iter::Iterator<Item = Token>> Parser<'_, I> {
    fn take(&mut self) -> ParseResult<Token> {
        return self
            .tokens
            .next()
            .ok_or("Unexpectedly reach end of file".to_owned());
    }

    fn peek(&mut self) -> ParseResult<&Token> {
        return self
            .tokens
            .peek()
            .ok_or("Unexpectedly reach end of file".to_owned());
    }

    fn expect(&mut self, expected: TokenKind) -> ParseResult<Token> {
        match self.tokens.next() {
            Some(token) if token.kind == expected => Ok(token),
            None => Err("Unexpectedly reached end of input".to_owned()),
            Some(unexpected) => Err(format!("Unexpectedly got '{}'", unexpected.value)),
        }
    }

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(TokenKind::Return)?;
        let return_val: Result<Expr, String> = self.parse_expr();
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(return_val?))
    }

    fn parse_unaryop(&mut self) -> ParseResult<UnaryOp> {
        match self.tokens.next() {
            Some(tok) => match tok.kind {
                TokenKind::Complement => todo!(),
                TokenKind::Negate => todo!(),
                TokenKind::Error(msg) => Err(msg.to_owned()),
                _ => Err("".to_owned()),
            },
            None => todo!(),
        }
    }

    fn parse_expr(&mut self) -> ParseResult<Expr> {
        let expr = match self.peek()?.kind {
            TokenKind::Constant => {
                let token = self.expect(TokenKind::Constant)?;
                Expr::Constant(token.value.parse().unwrap())
            }
            TokenKind::Negate | TokenKind::Complement => {
                let op = self.parse_unaryop()?;
                let inner_expr = self.parse_expr()?;
                Expr::Unary(op, Box::new(inner_expr))
            }
            TokenKind::LParen => {
                self.expect(TokenKind::LParen)?;
                let inner_expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                inner_expr
            }
            TokenKind::Error(msg) => return Err(msg.to_owned()),
            _ => return Err("Malformed expression".to_owned()),
        };

        Ok(expr)
    }

    fn parse_function(&mut self) -> ParseResult<Decl> {
        self.expect(TokenKind::Int)?;
        let name = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::LParen)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::RParen)?;

        self.expect(TokenKind::LBrace)?;
        let body = self.parse_statement()?;
        self.expect(TokenKind::RBrace)?;

        Ok(Decl::Function(name.value, Box::new(body)))
    }

    fn parse_program(&mut self) -> ParseResult<Program> {
        let func = self.parse_function()?;

        Ok(Program { body: func })
    }
}

pub fn parse<T: Iterator<Item = Token>>(tokens: &mut T) -> ParseResult<Program> {
    let mut tokens = tokens.peekable();
    let mut parser = Parser {
        tokens: &mut tokens,
    };
    let prg = parser.parse_program()?;

    if let Some(tok) = tokens.next() {
        return Err(format!(
            "Expected end of program, but got {:?} '{}'",
            tok.kind, tok.value
        ));
    }

    Ok(prg)
}
