use lorax::PassManager;

mod emit;
mod from_arith;
mod from_func;
mod ops;
mod state;

pub fn rules<'ctx>() -> PassManager {
    PassManager::new()
        .add_rule(from_arith::lower_unop)
        .add_rule(from_func::lower_func)
}
