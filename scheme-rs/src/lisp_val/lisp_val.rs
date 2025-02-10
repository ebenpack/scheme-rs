use num::complex::Complex64;
use num::rational::Rational64;
use std::fmt;
use std::rc::Rc;
use uuid::Uuid;

use crate::environment::Env;
use crate::error::LispResult;

// TODO: Constructor funcs for IFunc & EnvCtx?

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrimitiveFunc {
    pub name: String,
    pub func: fn(Vec<LispVal>) -> LispResult<LispVal>,
}

impl PrimitiveFunc {
    pub fn new(func: fn(Vec<LispVal>) -> LispResult<LispVal>, name: String) -> Self {
        Self { func, name }
    }
    pub fn apply(&self, args: Vec<LispVal>) -> LispResult<LispVal> {
        (self.func)(args)
    }
}

pub fn prim_func(name: String, func: fn(Vec<LispVal>) -> LispResult<LispVal>) -> LispVal {
    LispVal::PrimitiveFunc(PrimitiveFunc { name, func })
}

#[derive(Clone)]
pub struct Func {
    pub name: String,
    pub id: u128,
    pub params: Vec<String>,
    pub varargs: Option<String>,
    pub body: Vec<LispVal>,
    pub closure: Env,
}

impl Func {
    pub fn new(
        name: String,
        params: Vec<String>,
        varargs: Option<String>,
        body: Vec<LispVal>,
        closure: Env,
    ) -> Self {
        let id = Uuid::new_v4().as_u128();
        Self {
            name,
            id,
            params,
            varargs,
            body,
            closure,
        }
    }

    pub fn ctx(&self) -> &Env {
        &self.closure
    }
}

impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Func")
            .field("name", &self.name)
            .field("params", &self.params)
            .field("varargs", &self.varargs)
            .field("body", &self.body)
            .finish()
    }
}

impl PartialEq for Func {
    fn eq(&self, other: &Func) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LispVal {
    Atom(String),
    List(Rc<Vec<LispVal>>),
    DottedList(Rc<Vec<LispVal>>, Rc<LispVal>),
    Vector(Rc<Vec<LispVal>>),
    Integer(i64),
    Float(f64),
    Complex(Complex64),
    Rational(Rational64),
    String(String),
    Char(char), // TODO: Need this?
    PrimitiveFunc(PrimitiveFunc),
    Func(Func),
    Bool(bool),
    Quote(Rc<LispVal>),
    QuasiQuote(Rc<LispVal>),
    Unquote(Rc<LispVal>),
    UnquoteSplicing(Rc<LispVal>),
    Nil,
    Void,
}

impl fmt::Display for LispVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn format_list(xs: &[LispVal]) -> String {
            let result = xs
                .iter()
                .map(format_helper)
                .collect::<Vec<String>>()
                .join(" ");
            result
        }

        fn format_number(n: &LispVal) -> String {
            match n {
                LispVal::Integer(n) => format!("{}", n),
                LispVal::Float(n) => format!("{}", n),
                LispVal::Rational(r) => {
                    if *r.denom() == 0 {
                        format!("{}", r.numer())
                    } else {
                        format!("{}/{}", r.numer(), r.denom())
                    }
                }
                LispVal::Complex(c) => {
                    if c.im == 0.0 {
                        format!("{}", c.re)
                    } else {
                        format!("{}+{}i", c.re, c.im)
                    }
                }
                _ => unreachable!(),
            }
        }

        fn format_helper(val: &LispVal) -> String {
            // TODO: Better number formatting
            match val {
                LispVal::Atom(s) => s.clone(),
                LispVal::List(xs) => format!("({})", format_list(xs)),
                LispVal::DottedList(h, t) => format!("({} . {})", format_list(h), t),
                LispVal::Vector(xs) => format!("#({})", format_list(xs)),
                n @ LispVal::Integer(_) => format_number(n),
                n @ LispVal::Float(_) => format_number(n),
                n @ LispVal::Complex(_) => format_number(n),
                n @ LispVal::Rational(_) => format_number(n),
                LispVal::String(s) => format!("\"{}\"", s),
                LispVal::Char(c) => format_char(c),
                LispVal::PrimitiveFunc(f) => format!("#<procedure:{}>", f.name),
                LispVal::Func(f) => format!("#<procedure:{}>", f.name),
                LispVal::Nil => "Nil".to_owned(),
                LispVal::Bool(true) => "#t".to_owned(),
                LispVal::Bool(false) => "#f".to_owned(),
                LispVal::Quote(v) => format!("'{}", v),
                LispVal::QuasiQuote(v) => format!("`{}", v),
                LispVal::UnquoteSplicing(v) => format!(",@{}", v),
                LispVal::Unquote(v) => format!(",{}", v),
                LispVal::Void => "".to_owned(),
            }
        }
        fn format_char(c: &char) -> String {
            let cs = format!("{}", c);
            let s = match c.to_digit(10) {
                Some(27) => "altmode",
                Some(31) => "backnext",
                Some(8) => "backspace",
                Some(26) => "call",
                Some(10) => "linefeed",
                Some(12) => "page",
                Some(13) => "return",
                Some(127) => "rubout",
                Some(9) => "tab",
                _ => {
                    if *c == '\n' {
                        "newline"
                    } else if *c == ' ' {
                        "space"
                    } else {
                        cs.as_str()
                    }
                }
            };
            format!("#\\{}", s)
        }
        write!(f, "{}", format_helper(self))
    }
}
