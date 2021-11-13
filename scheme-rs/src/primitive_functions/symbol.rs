use std::collections::HashMap;

use crate::environment::Bindings;
use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

fn is_symbol(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match args[..] {
        [LispVal::Atom(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

fn symbol_to_string(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::Atom(s)] => Ok(LispVal::String(s.to_string())),
        [arg] => Err(LispError::GenericError(format!(
            "symbol->string: contract violation\nexpected: symbol?\ngiven: {}",
            arg
        ))),
        _ => unreachable!(),
    }
}

pub fn symbol_primitives() -> Bindings {
    HashMap::from([
        mk_prim_fn_binding("symbol?", is_symbol),
        mk_prim_fn_binding("symbol->string", symbol_to_string),
    ])
}
