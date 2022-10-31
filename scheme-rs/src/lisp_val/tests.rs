use std::rc::Rc;

use crate::{environment::Env, lisp_val::lisp_val::Func};

use super::*;
#[test]
fn atom_displays() {
    assert_eq!(format!("{}", LispVal::Atom("foobar".to_owned())), "foobar");
}

#[test]
fn list_displays() {
    assert_eq!(format!("{}", LispVal::List(Rc::new(vec![]))), "()");
    assert_eq!(
        format!(
            "{}",
            LispVal::List(Rc::new(vec![
                LispVal::Number(1729),
                LispVal::String("str".to_string()),
                LispVal::Nil,
                LispVal::Bool(true),
                LispVal::List(Rc::new(vec![
                    LispVal::Atom("atom".to_string()),
                    LispVal::PrimitiveFunc(PrimitiveFunc {
                        func: |_| Ok(LispVal::Nil),
                        name: "foo".to_string()
                    }),
                    LispVal::Func(Func::new(
                        "foo".to_string(),
                        vec![],
                        None,
                        vec![],
                        Env::new()
                    ))
                ])),
                LispVal::List(Rc::new(vec![])),
            ]))
        ),
        "(1729 \"str\" Nil #t (atom #<procedure:foo> #<procedure:foo>) ())"
    );
}

#[test]
fn number_displays() {
    assert_eq!(format!("{}", LispVal::Number(1729)), "1729");
}

#[test]
fn string_displays() {
    assert_eq!(
        format!("{}", LispVal::String("foobar".to_owned())),
        "\"foobar\""
    );
}

#[test]
fn fun_displays() {
    assert_eq!(
        format!(
            "{}",
            LispVal::PrimitiveFunc(PrimitiveFunc {
                func: |_| Ok(LispVal::Nil),
                name: "foo".to_string()
            }),
        ),
        "#<procedure:foo>"
    );
}

#[test]
fn lambda_displays() {
    assert_eq!(
        format!(
            "{}",
            LispVal::Func(Func::new(
                "foo".to_string(),
                vec![],
                None,
                vec![],
                Env::new()
            ))
        ),
        "#<procedure:foo>"
    );
}

#[test]
fn nil_displays() {
    assert_eq!(format!("{}", LispVal::Nil), "Nil");
}

#[test]
fn bool_displays() {
    assert_eq!(format!("{}", LispVal::Bool(true)), "#t");
    assert_eq!(format!("{}", LispVal::Bool(false)), "#f");
}
