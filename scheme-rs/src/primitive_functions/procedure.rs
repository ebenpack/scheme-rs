use std::collections::HashMap;

use crate::environment::Bindings;
use crate::error::{Arity, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

fn is_procedure(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::Func(_)] => Ok(LispVal::Bool(true)),
        [LispVal::PrimitiveFunc(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

pub fn procedure_primitives() -> Bindings {
    HashMap::from([mk_prim_fn_binding("procedure?", is_procedure)])
}
