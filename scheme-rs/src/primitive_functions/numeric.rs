use std::collections::HashMap;

use num::rational::Ratio;
use num::Zero;
use radix_fmt::radix;

use crate::environment::Bindings;
use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::LispVal;
use crate::numbers::cast;
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

fn num_add(args: Vec<LispVal>) -> LispResult<LispVal> {
    args.iter().fold(Ok(LispVal::Integer(0)), |res, y| {
        if let Ok(m) = res {
            match cast(&m, y)? {
                (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Integer(m + n)),
                (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Float(m + n)),
                (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Rational(m + n)),
                (LispVal::Complex(m), LispVal::Complex(n)) => Ok(LispVal::Complex(m + n)),
                _ => Err(LispError::GenericError("Unexpected error in +".to_string())),
            }
        } else {
            res
        }
    })
}

fn num_sub(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::Min(1))?;
    let first = args.first().unwrap();
    args.iter().skip(1).fold(Ok(first.clone()), |res, y| {
        if let Ok(m) = res {
            match cast(&m, y)? {
                (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Integer(m - n)),
                (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Float(m - n)),
                (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Rational(m - n)),
                (LispVal::Complex(m), LispVal::Complex(n)) => Ok(LispVal::Complex(m - n)),
                _ => Err(LispError::GenericError("Unexpected error in -".to_string())),
            }
        } else {
            res
        }
    })
}

fn num_mul(args: Vec<LispVal>) -> LispResult<LispVal> {
    args.iter().fold(Ok(LispVal::Integer(1)), |res, y| {
        if let Ok(m) = res {
            match cast(&m, y)? {
                (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Integer(m * n)),
                (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Float(m * n)),
                (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Rational(m * n)),
                (LispVal::Complex(m), LispVal::Complex(n)) => Ok(LispVal::Complex(m * n)),
                _ => Err(LispError::GenericError("Unexpected error in +".to_string())),
            }
        } else {
            res
        }
    })
}

fn num_div(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::Min(1))?;
    let first = args.first().unwrap();
    args.iter().skip(1).fold(Ok(first.clone()), |res, y| {
        if let Ok(m) = res {
            match cast(&m, y)? {
                (LispVal::Integer(m), LispVal::Integer(n)) => {
                    if n == 0 {
                        Err(LispError::GenericError("Divide by zero".to_owned()))
                    } else {
                        Ok(LispVal::Rational(Ratio::new(m, n)))
                    }
                }
                (LispVal::Float(m), LispVal::Float(n)) => {
                    if n == 0.0 {
                        Err(LispError::GenericError("Divide by zero".to_owned()))
                    } else {
                        Ok(LispVal::Float(m / n))
                    }
                }
                (LispVal::Rational(m), LispVal::Rational(n)) => {
                    if n.is_zero() {
                        Err(LispError::GenericError("Divide by zero".to_owned()))
                    } else {
                        Ok(LispVal::Rational(m / n))
                    }
                }
                (LispVal::Complex(m), LispVal::Complex(n)) => {
                    if n.is_zero() {
                        Err(LispError::GenericError("Divide by zero".to_owned()))
                    } else {
                        Ok(LispVal::Complex(m / n))
                    }
                }
                _ => Err(LispError::GenericError("Unexpected error in -".to_string())),
            }
        } else {
            res
        }
    })
}

fn num_eq(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    if let [m, n] = &args[..] {
        match cast(m, n)? {
            (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Bool(m == n)),
            (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Bool(m == n)),
            (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Bool(m == n)),
            (LispVal::Complex(m), LispVal::Complex(n)) => Ok(LispVal::Bool(m == n)),
            _ => Err(LispError::GenericError("Unexpected error in =".to_string())),
        }
    } else {
        unreachable!()
    }
}

fn num_neq(args: Vec<LispVal>) -> LispResult<LispVal> {
    match num_eq(args) {
        e @ Err(_) => e,
        Ok(LispVal::Bool(true)) => Ok(LispVal::Bool(false)),
        _ => Ok(LispVal::Bool(true)),
    }
}

fn num_gt(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    if let [m, n] = &args[..] {
        match cast(m, n)? {
            (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Bool(m > n)),
            (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Bool(m > n)),
            (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Bool(m > n)),
            (LispVal::Complex(_), LispVal::Complex(_)) => Err(LispError::GenericError(
                "> not defined for complex numbers".to_string(),
            )),
            _ => Err(LispError::GenericError("Unexpected error in =".to_string())),
        }
    } else {
        unreachable!()
    }
}

fn num_lt(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    if let [m, n] = &args[..] {
        match cast(m, n)? {
            (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Bool(m < n)),
            (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Bool(m < n)),
            (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Bool(m < n)),
            (LispVal::Complex(_), LispVal::Complex(_)) => Err(LispError::GenericError(
                "< not defined for complex numbers".to_string(),
            )),
            _ => Err(LispError::GenericError("Unexpected error in =".to_string())),
        }
    } else {
        unreachable!()
    }
}

fn num_gte(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    if let [m, n] = &args[..] {
        match cast(m, n)? {
            (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Bool(m >= n)),
            (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Bool(m >= n)),
            (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Bool(m >= n)),
            (LispVal::Complex(_), LispVal::Complex(_)) => Err(LispError::GenericError(
                ">= not defined for complex numbers".to_string(),
            )),
            _ => Err(LispError::GenericError("Unexpected error in =".to_string())),
        }
    } else {
        unreachable!()
    }
}

fn num_lte(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    if let [m, n] = &args[..] {
        match cast(m, n)? {
            (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Bool(m <= n)),
            (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Bool(m <= n)),
            (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Bool(m <= n)),
            (LispVal::Complex(_), LispVal::Complex(_)) => Err(LispError::GenericError(
                "<= not defined for complex numbers".to_string(),
            )),
            _ => Err(LispError::GenericError("Unexpected error in =".to_string())),
        }
    } else {
        unreachable!()
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
        [LispVal::Float(n)] => Ok(LispVal::String(format!("{}", n))),
        [LispVal::Float(_), LispVal::Integer(_)] => {
            Err(LispError::GenericError(
                "number->string: inexact numbers can only be printed in base 10".to_string(),
            ))
        },
        [LispVal::Rational(r)] => {
            if *r.denom() == 0 {
                Ok(LispVal::String(format!("{}", r.numer())))
            } else {
                Ok(LispVal::String(format!("{}/{}", r.numer(), r.denom())))
            }
        },
        [LispVal::Rational(r), LispVal::Integer(base)] => {
            match base {
                2 | 8 | 10 | 16 => {
                    let base = u8::try_from(*base).map_err(|_| LispError::GenericError("Unexpected error in number->string".to_string()))?;
                    if *r.denom() == 0 {
                        Ok(
                            LispVal::String(format!("{}", radix(*r.numer(), base)))
                        )
                    } else {
                        Ok(
                            LispVal::String(format!("{}/{}", radix(*r.numer(), base), radix(*r.denom(), base)))
                        )
                    }
                }
                _ => Err(LispError::GenericError(
                    format!("number->string: contract violation\nexpected: (or/c 2 8 10 16)\ngiven: {}\nargument position: 2nd", base),
                ))
            }
        }
        [LispVal::Complex(c)] => {
            if c.im == 0.0 {
                Ok(LispVal::String(format!("{}", c.re)))
            } else {
                Ok(LispVal::String(format!("{}+{}i", c.re, c.im)))
            }
        },
        [LispVal::Complex(_), LispVal::Integer(_)] => {
            // TODO: Technically we should be able to format exact complex #s
            Err(LispError::GenericError(
                "number->string: inexact numbers can only be printed in base 10".to_string(),
            ))
        },
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
    check_arity(&args, Arity::MinMax(1, 1))?;
    Ok(LispVal::Bool(matches!(
        args.first(),
        Some(LispVal::Integer(_))
            | Some(LispVal::Float(_))
            | Some(LispVal::Rational(_))
            | Some(LispVal::Complex(_))
    )))
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
