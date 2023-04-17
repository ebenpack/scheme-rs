extern crate nom;

use std::fmt;

use nom::error::ErrorKind;
use nom::error::ParseError;

use crate::lisp_val::LispVal;

#[derive(Debug, PartialEq, Eq)]
pub enum Arity {
    Min(u16),
    MinMax(u16, u16),
}

#[derive(Debug, PartialEq)]
pub enum LispError {
    NumArgs(Arity, u16, Vec<LispVal>),
    TypeMismatch(String, LispVal),
    Parser(String), // TODO
    BadSpecialForm(String, LispVal),
    NotFunction(String, String),
    UnboundVar(String, String),
    // TODO: Change this?
    GenericError(String),
}

impl fmt::Display for LispError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LispError::TypeMismatch(expected, found) => {
                write!(f, "Invalid type: expected {}, found {}", expected, found)
            }
            LispError::Parser(parse_error) => write!(f, "Parse error at {}", parse_error),
            LispError::BadSpecialForm(message, form) => write!(f, "{}: {}", message, form),
            LispError::NotFunction(message, func) => write!(f, "{}: {}", message, func),
            LispError::UnboundVar(message, varname) => write!(f, "{}: {}", message, varname),
            LispError::GenericError(message) => write!(f, "{}", message),
            LispError::NumArgs(expected, found, args) => {
                let args_error = match args[..] {
                    [] => "".to_string(),
                    _ => format!(
                        "\narguments:\n{}",
                        args.iter()
                            .map(|x| format!("{}", x))
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                };
                match &expected {
                    Arity::Min(min) => write!(f, "arity mismatch;\nthe expected number of arguments does not match the given number\nexpected: at least {}\ngiven: {}{}", min, found, args_error),
                    Arity::MinMax(min, max) => {
                        write!(f, "arity mismatch;\nthe expected number of arguments does not match the given number\nexpected: {}\ngiven: {}{}", 
                            if min == max { format!("{}", min) } else { format!("between {} and {}", min, max) },
                            format!("{}", found),
                            args_error
                        )
                    }
                }
            }
        }
    }
}

// TODO: Should this be generic? Or just bind LispVal?
pub type LispResult<T> = Result<T, LispError>;

impl<'a> From<(&'a [u8], ErrorKind)> for LispError {
    fn from((_input, _kind): (&'a [u8], ErrorKind)) -> Self {
        // TODO
        LispError::Parser("Error".to_string())
    }
}

impl<I> ParseError<I> for LispError {
    fn from_error_kind(_input: I, _kind: ErrorKind) -> Self {
        // TODO
        LispError::Parser("Error".to_string())
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
