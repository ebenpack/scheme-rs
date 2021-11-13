use std::collections::HashMap;
use std::iter;

use crate::environment::Bindings;
use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

#[macros::string_to_bool_binop("string=?")]
fn string_eq(args: Vec<LispVal>) -> LispResult<LispVal> {
    s1 != s2
}

#[macros::string_to_bool_binop("string<?")]
fn string_lt(args: Vec<LispVal>) -> LispResult<LispVal> {
    s1 >= s2
}

#[macros::string_to_bool_binop("string>?")]
fn string_gt(args: Vec<LispVal>) -> LispResult<LispVal> {
    s1 <= s2
}

#[macros::string_to_bool_binop("string<=?")]
fn string_lte(args: Vec<LispVal>) -> LispResult<LispVal> {
    s1 > s2
}

#[macros::string_to_bool_binop("string>=?")]
fn string_gte(args: Vec<LispVal>) -> LispResult<LispVal> {
    s1 < s2
}

fn is_string(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match args[..] {
        [LispVal::String(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

fn string_to_symbol(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::String(s)] => Ok(LispVal::Atom(s.to_string())),
        [arg] => Err(LispError::GenericError(format!(
            "string->symbol: contract violation\nexpected: string?\ngiven: {}",
            arg
        ))),
        _ => unreachable!(),
    }
}

fn string_ref(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [LispVal::String(s), LispVal::Number(n)] => {
            let n: usize = usize::try_from(*n).map_err(|_| {
                LispError::GenericError(format!("string-ref: index is out of range\nindex: {}\nvalid range: [0, {}]\nstring: \"{}\"", n, s.len(), &s))
            })?;
            match s.chars().nth(n) {
                Some(c) => {
                    Ok(LispVal::Char(c))
                },
                _ => Err(LispError::GenericError(format!("string-ref: index is out of range\nindex: {}\nvalid range: [0, {}]\nstring: \"{}\"", n, s.len(), &s)))
            }
        },
        [LispVal::String(_), arg] => Err(LispError::GenericError(format!("string-ref: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}\nargument position: 2nd", arg))),
        [arg, LispVal::Number(_)] => Err(LispError::GenericError(format!("string-ref: contract violation\nexpected: string?\ngiven: {}\nargument position: 1st", arg))),
        _ => unreachable!()
    }
}

fn make_string(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [LispVal::Number(n), LispVal::Char(c)] => {
            let n: usize = usize::try_from(*n).map_err(|_| {
                LispError::GenericError(format!("make-string: out of memory making string of length {}", n))
            })?;
            Ok(LispVal::String(
                iter::repeat(c).take(n).collect::<String>()
            ))
        },
        [LispVal::Number(_), arg] => Err(LispError::GenericError(format!("make-string: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}\nargument position: 2nd", arg))),
        [arg, LispVal::Char(_)] => Err(LispError::GenericError(format!("make-string: contract violation\nexpected: char?\ngiven: {}\nargument position: 1st", arg))),
        _ => unreachable!()
    }
}

fn str_len(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::String(s)] => {
            Ok(LispVal::Number(s.len().try_into().map_err(|_| {
                LispError::GenericError("weird string length".to_string())
            })?))
        }
        [arg] => Err(LispError::GenericError(format!(
            "string-length: contract violation\nexpected: string?\ngiven: {}",
            arg
        ))),

        _ => unreachable!(),
    }
}

fn str_append(args: Vec<LispVal>) -> LispResult<LispVal> {
    let mut result = vec![];
    for arg in args {
        match arg {
            LispVal::String(s) => result.push(s),
            arg => {
                return Err(LispError::GenericError(format!(
                    "string-append: contract violation\nexpected: string?\ngiven: {}",
                    arg
                )))
            }
        }
    }
    Ok(LispVal::String(result.join("")))
}

fn substring(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 3))?;
    match &args[..] {
        [LispVal::String(s), LispVal::Number(n)] => {
            let n: usize = usize::try_from(*n).map_err(|_| {
                LispError::GenericError(format!("substring: starting index is out of range\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", n, s.len(), &s))
            })?;
            if n > s.len() {
                Err(LispError::GenericError(format!("substring: starting index is out of range\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", n, s.len(), &s)))
            } else {
                Ok(LispVal::String(s.chars().skip(n).collect()))
            }
        }
        [LispVal::String(s), LispVal::Number(m), LispVal::Number(n)] => {
            let n: usize = usize::try_from(*n).map_err(|_| {
                LispError::GenericError(format!("substring: starting index is out of range\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", n, s.len(), &s))
            })?;
            let m: usize = usize::try_from(*m).map_err(|_| {
                LispError::GenericError(format!("substring: ending index is out of range\nending index: {}\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", m, n, s.len(), &s))
            })?;
            let len = s.len();
            if m > n {
                Err(LispError::GenericError(format!("substring: ending index is smaller than starting index\nending index: {}\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", m, n, len, &s)))
            } else if n > len {
                Err(LispError::GenericError(format!("substring: starting index is out of range\nending index: {}\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", m, n, len, &s)))
            } else if m > len {
                Err(LispError::GenericError(format!("substring: ending index is out of range\nending index: {}\nstarting index: {}\nvalid range: [0, {}]\nstring: {}", m, n, len, &s)))
            } else {
                Ok(LispVal::String(s.chars().skip(m).take(n - m).collect()))
            }
        }
        [arg1, arg2] => {
            if !matches!(arg1, LispVal::String(_)) {
                Err(LispError::GenericError(format!("substring: contract violation\nexpected: string?\ngiven: {}\nargument position: 1st", arg2)))
            } else {
                Err(LispError::GenericError(format!("substring: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}\nargument position: 2nd", arg2)))
            }
        }
        [arg1, arg2, arg3] => {
            if !matches!(arg1, LispVal::String(_)) {
                Err(LispError::GenericError(format!("substring: contract violation\nexpected: string?\ngiven: {}\nargument position: 1st", arg2)))
            } else if !matches!(arg2, LispVal::Number(_)) {
                Err(LispError::GenericError(format!("substring: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}\nargument position: 2nd", arg2)))
            } else {
                Err(LispError::GenericError(format!("substring: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}\nargument position: 3rd", arg3)))
            }
        }
        _ => unreachable!(),
    }
}

pub fn string_primitives() -> Bindings {
    HashMap::from([
        mk_prim_fn_binding("string=?", string_eq),
        mk_prim_fn_binding("string<?", string_lt),
        mk_prim_fn_binding("string>?", string_gt),
        mk_prim_fn_binding("string<=?", string_lte),
        mk_prim_fn_binding("string>=?", string_gte),
        mk_prim_fn_binding("string?", is_string),
        mk_prim_fn_binding("string->symbol", string_to_symbol),
        mk_prim_fn_binding("string-ref", string_ref),
        mk_prim_fn_binding("make-string", make_string),
        mk_prim_fn_binding("string-length", str_len),
        mk_prim_fn_binding("string-append", str_append),
        mk_prim_fn_binding("substring", substring),
    ])
}
