use std::collections::HashMap;

use radix_fmt::radix;

use crate::environment::Bindings;
use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::LispVal;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

// TODO: macro?
fn num_add(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    let mut result = 0;
    for val in args {
        if let LispVal::Integer(n) = val {
            result += n;
        } else {
            // TODO: Typeerror?
            return Err(LispError::GenericError("Unexpected error in +".to_string()));
        }
    }
    Ok(LispVal::Integer(result))
}

fn num_sub(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::Min(1))?;
    if let LispVal::Integer(mut result) = args[0] {
        for val in &args[1..] {
            if let LispVal::Integer(n) = val {
                result -= n;
            } else {
                // TODO: Typeerror?
                return Err(LispError::GenericError("Unexpected error in +".to_string()));
            }
        }
        Ok(LispVal::Integer(result))
    } else {
        Err(LispError::GenericError("Unexpected error in +".to_string()))
    }
}

fn num_mul(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    let mut result = 1;
    for val in args {
        if let LispVal::Integer(n) = val {
            result *= n;
        } else {
            // TODO: Typeerror?
            return Err(LispError::GenericError("Unexpected error in +".to_string()));
        }
    }
    Ok(LispVal::Integer(result))
}

fn num_div(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::Min(1))?;
    if let LispVal::Integer(mut result) = args[0] {
        for val in args.iter().skip(1) {
            if let LispVal::Integer(n) = val {
                result /= n;
            } else {
                // TODO: Typeerror?
                return Err(LispError::GenericError("Unexpected error in +".to_string()));
            }
        }
        Ok(LispVal::Integer(result))
    } else {
        // TODO: Typeerror?
        Err(LispError::GenericError("Unexpected error in +".to_string()))
    }
}

fn num_eq(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    let mut result = None;
    for val in args {
        if let LispVal::Integer(n) = val {
            match result {
                None => result = Some(n),
                Some(m) => {
                    if m != n {
                        return Ok(LispVal::Bool(false));
                    }
                }
            }
        } else {
            // TODO: Typeerror?
            return Err(LispError::GenericError("Unexpected error in +".to_string()));
        }
    }
    Ok(LispVal::Bool(true))
}

fn num_neq(args: Vec<LispVal>) -> LispResult<LispVal> {
    match num_eq(args) {
        e @ Err(_) => e,
        Ok(LispVal::Bool(true)) => Ok(LispVal::Bool(false)),
        _ => Ok(LispVal::Bool(true)),
    }
}

fn num_gt(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::Min(1))?;
    if let LispVal::Integer(mut result) = args[0] {
        for val in args.iter().skip(1) {
            if let LispVal::Integer(n) = val {
                if result < *n {
                    return Ok(LispVal::Bool(false));
                }
                result = *n;
            } else {
                // TODO: Typeerror?
                return Err(LispError::GenericError("Unexpected error in +".to_string()));
            }
        }
        Ok(LispVal::Bool(true))
    } else {
        // TODO: Typeerror?
        Err(LispError::GenericError("Unexpected error in +".to_string()))
    }
}

fn num_lt(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::Min(1))?;
    if let LispVal::Integer(mut result) = args[0] {
        for val in args.iter().skip(1) {
            if let LispVal::Integer(n) = val {
                if result < *n {
                    return Ok(LispVal::Bool(false));
                }
                result = *n;
            } else {
                // TODO: Typeerror?
                return Err(LispError::GenericError("Unexpected error in +".to_string()));
            }
        }
        Ok(LispVal::Bool(true))
    } else {
        // TODO: Typeerror?
        Err(LispError::GenericError("Unexpected error in +".to_string()))
    }
}

fn num_gte(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::Min(1))?;
    if let LispVal::Integer(mut result) = args[0] {
        for val in args.iter().skip(1) {
            if let LispVal::Integer(n) = val {
                if !result >= *n {
                    return Ok(LispVal::Bool(false));
                }
                result = *n;
            } else {
                // TODO: Typeerror?
                return Err(LispError::GenericError("Unexpected error in +".to_string()));
            }
        }
        Ok(LispVal::Bool(true))
    } else {
        // TODO: Typeerror?
        Err(LispError::GenericError("Unexpected error in +".to_string()))
    }
}

fn num_lte(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::Min(1))?;
    if let LispVal::Integer(mut result) = args[0] {
        for val in args.iter().skip(1) {
            if let LispVal::Integer(n) = val {
                if !result <= *n {
                    return Ok(LispVal::Bool(false));
                }
                result = *n;
            } else {
                // TODO: Typeerror?
                return Err(LispError::GenericError("Unexpected error in +".to_string()));
            }
        }
        Ok(LispVal::Bool(true))
    } else {
        // TODO: Typeerror?
        Err(LispError::GenericError("Unexpected error in +".to_string()))
    }
}

fn num_mod(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [LispVal::Integer(n), LispVal::Integer(m)] => Ok(LispVal::Integer(n % m)),
        _ =>
        // TODO: Typeerror?
        {
            Err(LispError::GenericError(
                "Unexpected error in modulo".to_string(),
            ))
        }
    }
}

fn num_to_string(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::MinMax(1, 2))?;
    match &args[..] {
        [LispVal::Integer(n)] => Ok(LispVal::String(format!("{}", n))),
        [LispVal::Integer(n), LispVal::Integer(base)] => {
            match base {
                2 | 8 | 10 | 16 => {
                    let base = u8::try_from(*base).map_err(|_| LispError::GenericError("Unexpected error in number->string".to_string()))?;
                    Ok(
                        LispVal::String(format!("{}", radix(*n, base)))
                    )
                }
                _ => Err(LispError::GenericError(
                    format!("number->string: contract violation\nexpected: (or/c 2 8 10 16)\ngiven: {}\nargument position: 2nd", base),
                ))
            }
        }
        _ =>
        // TODO: Typeerror?
            {
                Err(LispError::GenericError(
                    "Unexpected error in number->string".to_string(),
                ))
            }
    }
}

fn is_number(args: Vec<LispVal>) -> LispResult<LispVal> {
    // TODO: Other number types
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::Integer(_)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

pub fn numeric_primitives() -> Bindings {
    HashMap::from([
        mk_prim_fn_binding("+", num_add),
        mk_prim_fn_binding("-", num_sub),
        mk_prim_fn_binding("*", num_mul),
        mk_prim_fn_binding("/", num_div),
        mk_prim_fn_binding("=", num_eq),
        mk_prim_fn_binding("/=", num_neq),
        mk_prim_fn_binding(">", num_gt),
        mk_prim_fn_binding("<", num_lt),
        mk_prim_fn_binding(">=", num_gte),
        mk_prim_fn_binding("<=", num_lte),
        mk_prim_fn_binding("modulo", num_mod),
        mk_prim_fn_binding("number->string", num_to_string),
        mk_prim_fn_binding("number?", is_number),
        // TODO!
        // mk_prim_fn_binding("quotient", num_quotient),
        // mk_prim_fn_binding("remainder", num_rem),
        // mk_prim_fn_binding("sin", num_sine),
        // mk_prim_fn_binding("cos", num_cos),
        // mk_prim_fn_binding("complex?", is_complex),
        // mk_prim_fn_binding("real?", is_real),
        // mk_prim_fn_binding("rational?", is_rational),
        // mk_prim_fn_binding("integer?", is_integer),
    ])
}
