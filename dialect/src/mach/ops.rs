use lorax::{Operation, Value, def_op};

def_op! {
    mach.start_frame()
}

def_op! {
    mach.end_frame(frame: Value) -> None
}
