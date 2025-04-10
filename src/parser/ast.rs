// tokens
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier, // a-z
    Constant,   // 0-9

    // Keywords
    Int,
    Void,
    Return,

    // Operations
    Complement,
    Negate,
    Decrement,

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
pub struct Program {
    pub body: Decl,
}

#[derive(Debug)]
pub enum Decl {
    Function(String, Box<Stmt>),
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Constant(u32),
}
