#[derive(Debug)]
pub struct Program {
    pub body: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Function {
        name: String,
        body: Vec<Instruction>,
    },
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(u32),
    Register,
}
