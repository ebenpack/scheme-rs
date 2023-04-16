use std::rc::Rc;

use super::*;
use crate::{
    environment::Env,
    error::LispError,
    eval::eval::eval_expression_list,
    lisp_val::LispVal,
    parser::{expression, expression_list},
};

#[test]
fn eval_void() {
    let env = Env::new();
    assert_eq!(eval(&env, &LispVal::Void), Ok(LispVal::Void))
}

#[test]
fn eval_string() {
    let env = Env::new();
    assert_eq!(
        eval(&env, &LispVal::String("foo".to_string())),
        Ok(LispVal::String("foo".to_string()))
    )
}

#[test]
fn eval_char() {
    let env = Env::new();
    assert_eq!(eval(&env, &LispVal::Char('c')), Ok(LispVal::Char('c')))
}

#[test]
fn eval_number() {
    let env = Env::new();
    assert_eq!(
        eval(&env, &LispVal::Integer(1729)),
        Ok(LispVal::Integer(1729))
    )
}

#[test]
fn eval_bool() {
    let env = Env::new();
    assert_eq!(eval(&env, &LispVal::Bool(true)), Ok(LispVal::Bool(true)));
    assert_eq!(eval(&env, &LispVal::Bool(false)), Ok(LispVal::Bool(false)))
}

#[test]
fn eval_vector() {
    let env = Env::new();
    assert_eq!(
        eval(
            &env,
            &LispVal::Vector(Rc::new(vec![
                LispVal::Integer(1729),
                LispVal::String("foo".to_string())
            ]))
        ),
        Ok(LispVal::Vector(Rc::new(vec![
            LispVal::Integer(1729),
            LispVal::String("foo".to_string())
        ])))
    )
}

#[test]
fn eval_cond() {
    let env = Env::new();

    let (_, exp) = expression("(cond [#t 1] [#f 2] (else 3))").unwrap();
    assert_eq!(eval(&env, &exp), Ok(LispVal::Integer(1)));

    let (_, exp) = expression("(cond [#f 1] [#t 2] (else 3))").unwrap();
    assert_eq!(eval(&env, &exp), Ok(LispVal::Integer(2)));

    let (_, exp) = expression("(cond [#f 1] [#f 2] (else 3))").unwrap();
    assert_eq!(eval(&env, &exp), Ok(LispVal::Integer(3)));

    let (_, exp) = expression("(cond (1) [#f 2] (else 3))").unwrap();
    assert_eq!(eval(&env, &exp), Ok(LispVal::Integer(1)));

    let (_, exp) = expression("(cond (else 1729))").unwrap();
    assert_eq!(eval(&env, &exp), Ok(LispVal::Integer(1729)));

    // TODO: Fix implementation
    // let (_, exp) = expression("(cond [#t 1] [else 2] (else 3))").unwrap();
    // assert_eq!(
    //     eval(&env, exp),
    //     Err(LispError::GenericError(
    //         "Duplicate definition for identifier in: foo".to_string()
    //     ))
    // );
}

#[test]
fn eval_define_var() {
    let env = Env::new();
    assert_eq!(env.lookup("foo"), None);
    let (_, exp) = expression("(define foo 1)").unwrap();
    eval(&env, &exp).unwrap();
    assert_eq!(env.lookup("foo"), Some(LispVal::Integer(1)));

    let (_, exp) = expression("(define foo 1)").unwrap();
    assert_eq!(
        eval(&env, &exp),
        Err(LispError::GenericError(
            "Duplicate definition for identifier in: foo".to_string()
        ))
    );
}

#[test]
fn eval_lambda() {
    let env = Env::new();
    let (_, exprs) = expression_list("(define (first a b) a) (first 1 2)").unwrap();
    assert_eq!(
        eval_expression_list(&env, exprs).unwrap().last().unwrap(),
        &LispVal::Integer(1)
    );

    let env = Env::new();
    let (_, exprs) = expression_list(
        "(define (nested a b) (define (more-nested a b) a) (more-nested b a)) (nested 1 2)",
    )
    .unwrap();
    assert_eq!(
        eval_expression_list(&env, exprs).unwrap().last().unwrap(),
        &LispVal::Integer(2)
    );
}
