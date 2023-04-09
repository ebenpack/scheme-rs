use std::fmt;
use std::rc::Rc;
use uuid::Uuid;

use crate::environment::Env;
use crate::error::LispResult;

// TODO: Constructor funcs for IFunc & EnvCtx?

#[derive(Debug, PartialEq, Clone)]
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
    // TODO: Add Integer, Float, Complex/Rational, etc.
    Number(i64),
    String(String),
    Char(char), // TODO: Need this?
    PrimitiveFunc(PrimitiveFunc),
    Func(Func),
    Bool(bool),
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

        fn format_helper(val: &LispVal) -> String {
            match val {
                LispVal::Atom(s) => s.clone(),
                LispVal::List(xs) => format!("({})", format_list(xs)),
                LispVal::DottedList(h, t) => format!("({} . {})", format_list(h), t),
                LispVal::Vector(xs) => format!("#({})", format_list(xs)),
                LispVal::Number(n) => n.to_string(),
                LispVal::String(s) => format!("\"{}\"", s),
                LispVal::Char(c) => format_char(c),
                LispVal::PrimitiveFunc(f) => format!("#<procedure:{}>", f.name),
                LispVal::Func(f) => format!("#<procedure:{}>", f.name),
                LispVal::Nil => "Nil".to_owned(),
                LispVal::Bool(true) => "#t".to_owned(),
                LispVal::Bool(false) => "#f".to_owned(),
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
            format!("#\\{}", s.to_string())
        }
        write!(f, "{}", format_helper(self))
    }
}
