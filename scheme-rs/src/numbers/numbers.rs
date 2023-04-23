use num::complex::Complex64;
use num::rational::Rational64;

use crate::{
    error::{LispError, LispResult},
    lisp_val::LispVal,
};

pub fn is_num(val: &LispVal) -> bool {
    matches!(
        val,
        LispVal::Integer(_) | LispVal::Float(_) | LispVal::Rational(_) | LispVal::Complex(_)
    )
}

fn rational_cast(n: &Rational64) -> Option<f64> {
    if *n.denom() == 0 {
        None
    } else {
        Some((*n.numer() as f64) / (*n.denom() as f64))
    }
}

pub fn cast(m: &LispVal, n: &LispVal) -> LispResult<(LispVal, LispVal)> {
    match (m, n) {
        (a @ LispVal::Integer(_), b @ LispVal::Integer(_)) => Ok((a.clone(), b.clone())),
        (a @ LispVal::Rational(_), b @ LispVal::Rational(_)) => Ok((a.clone(), b.clone())),
        (a @ LispVal::Float(_), b @ LispVal::Float(_)) => Ok((a.clone(), b.clone())),
        (a @ LispVal::Complex(_), b @ LispVal::Complex(_)) => Ok((a.clone(), b.clone())),
        // -- Integer
        (LispVal::Integer(a), b @ LispVal::Rational(_)) => {
            Ok((LispVal::Rational(Rational64::new(*a, 1)), b.clone()))
        }
        (LispVal::Integer(a), b @ LispVal::Float(_)) => {
            // TODO: Conversion :S
            Ok((LispVal::Float(*a as f64), b.clone()))
        }
        // TODO: Conversion :S
        (LispVal::Integer(a), b @ LispVal::Complex(_)) => {
            Ok((LispVal::Complex(Complex64::from(*a as f64)), b.clone()))
        }
        // -- Rational
        (a @ LispVal::Rational(_), LispVal::Integer(b)) => {
            Ok((a.clone(), LispVal::Rational(Rational64::new(*b, 1))))
        }
        (LispVal::Rational(a), b @ LispVal::Float(_)) => match rational_cast(a) {
            None => Err(LispError::GenericError(
                "Unexpected error converting rational".to_string(),
            )),
            Some(n) => Ok((LispVal::Float(n), b.clone())),
        },
        (LispVal::Rational(a), b @ LispVal::Complex(_)) => match rational_cast(a) {
            None => Err(LispError::GenericError(
                "Unexpected error converting rational".to_string(),
            )),
            Some(n) => Ok((LispVal::Complex(Complex64::new(n, 0.0)), b.clone())),
        },
        // -- Float
        (a @ LispVal::Float(_), LispVal::Rational(b)) => match rational_cast(b) {
            None => Err(LispError::GenericError(
                "Unexpected error converting rational".to_string(),
            )),
            Some(n) => Ok((a.clone(), LispVal::Float(n))),
        },
        // TODO: Cast
        (a @ LispVal::Float(_), LispVal::Integer(b)) => Ok((a.clone(), LispVal::Float(*b as f64))),
        (LispVal::Float(a), b @ LispVal::Complex(_)) => {
            Ok((LispVal::Complex(Complex64::new(*a, 0.0)), b.clone()))
        }
        // -- Complex
        (a @ LispVal::Complex(_), LispVal::Rational(b)) => match rational_cast(b) {
            None => Err(LispError::GenericError(
                "Unexpected error converting rational".to_string(),
            )),
            Some(n) => Ok((a.clone(), LispVal::Complex(Complex64::new(n, 0.0)))),
        },
        (a @ LispVal::Complex(_), LispVal::Float(b)) => {
            Ok((a.clone(), LispVal::Complex(Complex64::new(*b, 0.0))))
        }
        // TODO: Cast
        (a @ LispVal::Complex(_), LispVal::Integer(b)) => {
            Ok((a.clone(), LispVal::Complex(Complex64::new(*b as f64, 0.0))))
        }
        _ => Err(LispError::GenericError(
            "Unexpected error in cast".to_string(),
        )),
    }
}
