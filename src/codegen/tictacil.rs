struct Program {
    body: Function,
}

struct Function {
    name: String,
    body: Vec<Instruction>,
}

enum Instruction {
    Return(u32),
    Unary { op: UnaryOp, src: Value, dst: Value },
}

enum UnaryOp {
    Complement,
    Negate,
}

enum Value {
    Constant(u32),
    Var(String),
}
