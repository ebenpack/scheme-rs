use std::collections::HashMap;

use crate::environment::Bindings;
use crate::error::{Arity, LispResult};
use crate::lisp_val::LispVal;

use super::boolean::boolean_primitives;
use super::character::character_primitives;
use super::list::list_primitives;
use super::numeric::numeric_primitives;
use super::procedure::procedure_primitives;
use super::string::string_primitives;
use super::symbol::symbol_primitives;
use super::util::{check_arity, mk_prim_fn_binding};
use super::vector::vector_primitives;

fn void(_args: Vec<LispVal>) -> LispResult<LispVal> {
    Ok(LispVal::Void)
}

fn eq(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [LispVal::Bool(arg1), LispVal::Bool(arg2)] => Ok(LispVal::Bool(arg1 == arg2)),
        [LispVal::Number(arg1), LispVal::Number(arg2)] => Ok(LispVal::Bool(arg1 == arg2)),
        [LispVal::String(arg1), LispVal::String(arg2)] => Ok(LispVal::Bool(arg1 == arg2)),
        [LispVal::Char(arg1), LispVal::Char(arg2)] => Ok(LispVal::Bool(arg1 == arg2)),
        [LispVal::Atom(arg1), LispVal::Atom(arg2)] => Ok(LispVal::Bool(arg1 == arg2)),
        [LispVal::DottedList(xs, x), LispVal::DottedList(ys, y)] => {
            if let LispVal::Bool(false) = eq(vec![(&**x).clone(), (&**y).clone()])? {
                return Ok(LispVal::Bool(false));
            }
            if xs.len() != ys.len() {
                return Ok(LispVal::Bool(false));
            }
            for (x, y) in xs.iter().zip(ys.iter()) {
                if let LispVal::Bool(false) = eq(vec![x.clone(), y.clone()])? {
                    return Ok(LispVal::Bool(false));
                }
            }
            Ok(LispVal::Bool(true))
        }

        [LispVal::List(xs), LispVal::List(ys)] => {
            if xs.len() != ys.len() {
                return Ok(LispVal::Bool(false));
            }
            for (x, y) in xs.iter().zip(ys.iter()) {
                if let LispVal::Bool(false) = eq(vec![x.clone(), y.clone()])? {
                    return Ok(LispVal::Bool(false));
                }
            }
            Ok(LispVal::Bool(true))
        }
        [LispVal::Func(f), LispVal::Func(g)] => Ok(LispVal::Bool(f == g)),
        [LispVal::PrimitiveFunc(f), LispVal::PrimitiveFunc(g)] => Ok(LispVal::Bool(f == g)),
        [_, _] => Ok(LispVal::Bool(false)),

        _ => unreachable!(),
    }
}

pub fn primitive_functions() -> Bindings {
    let mut bindings = HashMap::new();
    bindings.extend(boolean_primitives());
    bindings.extend(character_primitives());
    bindings.extend(numeric_primitives());
    bindings.extend(list_primitives());
    bindings.extend(vector_primitives());
    bindings.extend(procedure_primitives());
    bindings.extend(string_primitives());
    bindings.extend(symbol_primitives());
    bindings.extend([
        mk_prim_fn_binding("void", void),
        mk_prim_fn_binding("eq?", eq),
        mk_prim_fn_binding("eqv?", eq),
        mk_prim_fn_binding("equal?", eq),
    ]);
    bindings
}
