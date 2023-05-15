
#[repr(C)]
pub enum StkType {
    Integer,
    String,
    Function,
    Missing,
    Illegal,
}

#[repr(C)]
pub struct ExecVal {
    typ: StkType,
    lit: i32,
}

#[repr(C)]
pub struct ExecCtx {
    pop_lit1: i32,
    pop_lit2: i32,
    pop_lit3: i32,
    pop_typ1: StkType,
    pop_typ2: StkType,
    pop_typ3: StkType,

    lit_stack: Vec<i32>,
    lit_stk_type: Vec<StkType>,
    lit_stk_ptr: i32,
    lit_stk_size: i32,
}
