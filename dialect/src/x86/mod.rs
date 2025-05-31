use lorax::PassManager;

use crate::mach;

mod emit;
mod from_arith;
mod from_func;
mod from_mach;
mod ops;
mod state;

pub use emit::EmitX86;

pub fn rules() -> PassManager {
    PassManager::new()
        .add_rule(from_arith::lower_unop)
        .add_rule(from_func::lower_func)
        .add_rule(mach::add_fn_frame)
        .add_rule(from_mach::lower_frame)
}
