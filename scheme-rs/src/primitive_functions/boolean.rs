use std::collections::HashMap;

use crate::environment::Bindings;
use crate::error::{Arity, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

fn is_boolean(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match args[..] {
        [LispVal::Bool(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

pub fn and(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::Min(1))?;
    for (i, val) in args.iter().enumerate() {
        if i == args.len() - 1 {
            return Ok(val.clone());
        }
        if let LispVal::Bool(false) = val {
            return Ok(LispVal::Bool(false));
        }
    }
    Ok(LispVal::Bool(true))
}

pub fn or(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::Min(1))?;
    for (i, val) in args.iter().enumerate() {
        if i == args.len() - 1 {
            return Ok(val.clone());
        }
        if let LispVal::Bool(true) = val {
            return Ok(LispVal::Bool(true));
        }
    }
    Ok(LispVal::Bool(false))
}

pub fn not(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match args[..] {
        [LispVal::Bool(false)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

pub fn boolean_primitives() -> Bindings {
    HashMap::from([
        mk_prim_fn_binding("boolean?", is_boolean),
        mk_prim_fn_binding("and", and),
        mk_prim_fn_binding("or", or),
        mk_prim_fn_binding("not", not),
    ])
}
