use crate::environment::Bindings;
use crate::error::{Arity, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;
use std::collections::HashMap;

fn is_char(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match args[..] {
        [LispVal::Char(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

pub fn character_primitives() -> Bindings {
    HashMap::from([mk_prim_fn_binding("char?", is_char)])
}
