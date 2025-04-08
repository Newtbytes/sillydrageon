pub struct Program {
    pub body: Vec<Decl>,
}

pub enum Decl {
    Function {
        name: String,
        body: Vec<Instruction>,
    },
}

pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

pub enum Operand {
    Imm(u32),
    Register,
}
