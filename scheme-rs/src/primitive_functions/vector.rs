use std::collections::HashMap;
use std::iter;
use std::rc::Rc;

use crate::environment::Bindings;
use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

fn is_vector(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match args[..] {
        [LispVal::Vector(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

fn vector_length(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::Vector(xs)] => {
            Ok(LispVal::Integer(xs.len().try_into().map_err(|_| {
                LispError::GenericError("weird list length".to_string())
            })?))
        }
        [arg] => Err(LispError::GenericError(format!(
            "vector-length: contract violation\nexpected: vector?\ngiven: {}",
            arg
        ))),
        _ => unreachable!(),
    }
}

fn vector_ref(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [v @ LispVal::Vector(xs), LispVal::Integer(n)] => {
            let index = usize::try_from(*n)
                .map_err(|_| LispError::GenericError("weird list length".to_string()))?;
            match xs.get(index ) {
                Some(val) => Ok(val.clone()),
                None => Err(LispError::GenericError(format!("vector-ref: index is out of range\nindex: {}\nvalid range: [0, {}]\nvector: {}", index, xs.len(), v))),
            }
        }
        [arg, LispVal::Integer(_)] => Err(LispError::GenericError(format!(
            "vector-ref: contract violation\nexpected: vector?\ngiven: {}",
            arg
        ))),
        [LispVal::Vector(_), arg] => Err(LispError::GenericError(format!(
            "vector-ref: contract violation\nexpected: vector?\ngiven: {}",
            arg
        ))),
        _ => unreachable!(),
    }
}

fn make_vector(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 2))?;
    match &args[..] {
        [LispVal::Integer(n)] => {
            let n = usize::try_from(*n)
                .map_err(|_| LispError::GenericError("weird vector length".to_string()))?;
            Ok(LispVal::Vector(Rc::new(
                iter::repeat(LispVal::Void).take(n).collect(),
            )))
        }
        [LispVal::Integer(n), arg] => {
            let n = usize::try_from(*n)
                .map_err(|_| LispError::GenericError("weird vector length".to_string()))?;
            Ok(LispVal::Vector(Rc::new(
                iter::repeat(arg.clone()).take(n).collect(),
            )))
        }
        [arg, ..] => Err(LispError::GenericError(format!(
            "make-vector: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}",
            arg
        ))),
        _ => unreachable!(),
    }
}

pub fn vector_primitives() -> Bindings {
    HashMap::from([
        mk_prim_fn_binding("vector?", is_vector),
        mk_prim_fn_binding("vector-length", vector_length),
        mk_prim_fn_binding("vector-ref", vector_ref),
        mk_prim_fn_binding("make-vector", make_vector),
    ])
}
