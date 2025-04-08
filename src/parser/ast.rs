// tokens
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier, // a-z
    Constant,   // 0-9

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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

// ast nodes
#[derive(Debug)]
pub enum Expr {
    Constant(u32),
}

#[derive(Debug)]
pub enum Stmt {
    FnSignature(String, Box<Stmt>),
    Program(Box<Stmt>),
    Return(Expr),
}
