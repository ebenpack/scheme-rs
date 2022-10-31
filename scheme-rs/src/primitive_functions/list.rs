use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter;
use std::rc::Rc;

use itertools::Itertools;

use crate::environment::{Bindings, Env, Ports};
use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::{Func, LispVal};
use crate::primitive_functions::boolean::{and, not};
use crate::primitive_functions::util::check_arity;
use crate::primitive_functions::util::mk_prim_fn_binding;

fn is_empty(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::Min(1))?;
    match &args[..] {
        [LispVal::List(xs)] => Ok(LispVal::Bool(xs.len() == 0)),
        _ => Ok(LispVal::Bool(false)),
    }
}

fn is_pair(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::Min(1))?;
    match &args[..] {
        [LispVal::List(xs)] => Ok(LispVal::Bool(xs.len() != 0)),
        [LispVal::DottedList(_, _)] => Ok(LispVal::Bool(true)),
        _ => Ok(LispVal::Bool(false)),
    }
}

fn car(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::List(xs)] => {
            if let [val, ..] = &xs[..] {
                return Ok(val.clone());
            }
        }
        [LispVal::DottedList(xs, _)] => {
            if let [val, ..] = &xs[..] {
                return Ok(val.clone());
            }
        }
        _ => (),
    }
    //  TODO: ContractViolation error?
    Err(LispError::GenericError(format!(
        "car: contract violation\nexpected: pair?\ngiven: {}",
        args[0]
    )))
}

fn cdr(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::DottedList(xs, val)] => match &xs[..] {
            [_] => return Ok((&**val).clone()),
            [_, xs @ ..] => return Ok(LispVal::DottedList(Rc::new(xs.to_vec()), val.clone())),
            _ => (),
        },
        [LispVal::List(xs)] => match &xs[..] {
            [] => (),
            [_] => return Ok(LispVal::List(Rc::new(vec![]))),
            [_, xs @ ..] => return Ok(LispVal::List(Rc::new(xs.to_vec()))),
        },
        _ => (),
    }
    //  TODO: ContractViolation error?
    Err(LispError::GenericError(format!(
        "car: contract violation\nexpected: pair?\ngiven: {}",
        args[0]
    )))
}

fn cons(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [a, LispVal::List(b)] => {
            let mut new_list = (&**b).clone().to_vec();
            new_list.insert(0, a.clone());
            Ok(LispVal::List(Rc::new(new_list)))
        }
        [a, LispVal::DottedList(b, c)] => {
            let mut new_list = (&**b).clone().to_vec();
            new_list.insert(0, a.clone());
            Ok(LispVal::DottedList(Rc::new(new_list), c.clone()))
        }
        [a, b] => Ok(LispVal::DottedList(
            Rc::new(vec![a.clone()]),
            Rc::new(b.clone()),
        )),
        _ => Err(LispError::GenericError(format!(
            "cond: contract violation\nexpected: pair?\ngiven: {}",
            args[0]
        ))),
    }
}

fn list(args: Vec<LispVal>) -> LispResult<LispVal> {
    Ok(LispVal::List(Rc::new(args)))
}

fn is_list(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::List(_)] => Ok(LispVal::Bool(true)),
        [LispVal::DottedList(_, tail)] => {
            // TODO: Is this correct?
            let tail: &LispVal = tail.borrow();
            is_list(vec![tail.clone()])
        },
        _ => Ok(LispVal::Bool(false)),
    }
}


#[cfg(test)]
mod tests {

    use super::{LispVal};
    use super::*;
    use std::rc::Rc;


    #[test]
    fn stuff_and_junk() {
        // TODO: Remove
        assert_eq!(
            is_list(vec![LispVal::DottedList(
                Rc::new(vec![LispVal::Atom("aa".to_string())]),
                Rc::new(LispVal::Atom("aa".to_string())), 
            )]),
            Ok(LispVal::Bool(false))
        );
        assert_eq!(
            is_list(vec![LispVal::DottedList(
                Rc::new(vec![LispVal::Atom("aa".to_string())]),
                Rc::new(LispVal::List(Rc::new(vec![LispVal::Atom("aa".to_string())]))), 
            )]),
            Ok(LispVal::Bool(true))
        );
    }
}

fn atom(args: Vec<LispVal>) -> LispResult<LispVal> {
    let not_pair = not(vec![is_pair(args.clone())?])?;
    let not_empty = not(vec![is_empty(args)?])?;
    and(vec![not_pair, not_empty])
}

fn length(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::List(xs)] => {
            Ok(LispVal::Number(xs.len().try_into().map_err(|_| {
                LispError::GenericError("weird list length".to_string())
            })?))
        }
        _ => Err(LispError::GenericError(format!(
            "length: contract violation\nexpected: list?\ngiven: {}",
            args[0]
        ))),
    }
}

fn append(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [] => Ok(LispVal::List(Rc::new(vec![]))),
        [v] => Ok(v.clone()),
        [LispVal::List(xs), LispVal::List(ys)] => {
            let mut new_list = xs.to_vec();
            new_list.append(&mut ys.to_vec());
            Ok(LispVal::List(Rc::new(new_list)))
        }
        xs => {
            let mut acc = vec![];
            for (index, val) in xs.iter().enumerate() {
                if index == xs.len() - 1 {
                    if let LispVal::DottedList(ls, v) = val {
                        acc.append(&mut ls.to_vec());
                        return Ok(LispVal::DottedList(Rc::new(acc), v.clone()));
                    }
                    return Ok(LispVal::DottedList(Rc::new(acc), Rc::new(val.clone())));
                } else if let LispVal::List(xs) = val {
                    acc.append(&mut xs.to_vec());
                } else {
                    return Err(LispError::GenericError(format!(
                        "append: contract violation\nexpected: list?\ngiven: {}",
                        val.clone()
                    )));
                }
            }
            Ok(LispVal::List(Rc::new(acc)))
        }
    }
}

fn reverse(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(1, 1))?;
    match &args[..] {
        [LispVal::List(xs)] => {
            let mut new_list = xs.to_vec();
            new_list.reverse();
            Ok(LispVal::List(Rc::new(new_list)))
        }
        [val] => Err(LispError::GenericError(format!(
            "reverse: contract violation\nexpected: list?\ngiven: {}",
            val.clone()
        ))),
        _ => Err(LispError::GenericError(
            "reverse: unknown error".to_string(),
        )),
    }
}

fn member(args: Vec<LispVal>) -> LispResult<LispVal> {
    check_arity(&args, Arity::MinMax(2, 2))?;
    match &args[..] {
        [needle, LispVal::List(xs)] => Ok(LispVal::Bool(xs.contains(needle))),
        [_, val] => Err(LispError::GenericError(format!(
            "member: not a proper list: {}",
            val.clone()
        ))),
        _ => Err(LispError::GenericError("member: unknown error".to_string())),
    }
}

fn replicate_m(count: u8, xs: Vec<char>) -> Vec<Vec<char>> {
    let repeated = iter::repeat(xs)
        .take(count.into())
        .collect::<Vec<Vec<char>>>();

    repeated.iter().rfold(vec![vec![]], |tails, heads| {
        heads
            .iter()
            .map(|head| {
                tails
                    .iter()
                    .map(|tail| {
                        let mut tail = tail.to_vec();
                        tail.insert(0, *head);
                        tail.to_vec()
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<Vec<char>>>()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_replicate_m() {
        assert_eq!(
            replicate_m(2, vec!['a', 'd']),
            vec![
                vec!['a', 'a'],
                vec!['a', 'd'],
                vec!['d', 'a'],
                vec!['d', 'd'],
            ]
        );
        assert_eq!(
            replicate_m(3, vec!['a', 'd']),
            vec![
                vec!['a', 'a', 'a'],
                vec!['a', 'a', 'd'],
                vec!['a', 'd', 'a'],
                vec!['a', 'd', 'd'],
                vec!['d', 'a', 'a'],
                vec!['d', 'a', 'd'],
                vec!['d', 'd', 'a'],
                vec!['d', 'd', 'd'],
            ]
        );
    }
}

fn make_accessor(accessors: &[char]) -> Vec<LispVal> {
    match accessors {
        ['d'] => vec![LispVal::List(Rc::new(vec![
            LispVal::Atom("cdr".to_string()),
            LispVal::Atom("xs".to_string()),
        ]))],
        ['a'] => vec![LispVal::List(Rc::new(vec![
            LispVal::Atom("car".to_string()),
            LispVal::Atom("xs".to_string()),
        ]))],
        ['d', xs @ ..] => {
            let mut rest = make_accessor(xs);
            rest.insert(0, LispVal::Atom("cdr".to_string()));
            vec![LispVal::List(Rc::new(rest))]
        }
        ['a', xs @ ..] => {
            let mut rest = make_accessor(xs);
            rest.insert(0, LispVal::Atom("car".to_string()));
            vec![LispVal::List(Rc::new(rest))]
        }
        _ => unreachable!(),
    }
}

pub fn accessors() -> Bindings {
    fn noop(_port: &mut Vec<LispVal>) {}

    let mut caaaaars = vec![];
    caaaaars.append(&mut replicate_m(2, vec!['a', 'd']));
    caaaaars.append(&mut replicate_m(3, vec!['a', 'd']));
    caaaaars.append(&mut replicate_m(4, vec!['a', 'd']));

    // This is a dirty, dirty hack. Rather than figure out how to dynamically
    // make these primitive functions, they'll just be regular functions.
    // As a result, we need to provide them with an environment which binds
    // everything (car, cdr) they need to do their dirty business.
    let caaaaars = caaaaars
        .iter()
        .map(|accessor| {
            let accessor_string = format!("c{}r", accessor.iter().join(""));
            (
                accessor_string.clone(),
                LispVal::Func(Func::new(
                    accessor_string,
                    vec!["xs".to_string()],
                    None,
                    make_accessor(accessor),
                    Env::with_bindings(
                        HashMap::from_iter([
                            mk_prim_fn_binding("car", car),
                            mk_prim_fn_binding("cdr", cdr),
                        ]),
                        Ports::new(Box::new(noop)),
                    ),
                )),
            )
        })
        .collect::<Vec<(String, LispVal)>>();

    HashMap::from_iter(caaaaars)
}

pub fn list_primitives() -> Bindings {
    let mut bindings = HashMap::from([
        mk_prim_fn_binding("empty?", is_empty),
        mk_prim_fn_binding("null?", is_empty),
        mk_prim_fn_binding("pair?", is_pair),
        mk_prim_fn_binding("car", car),
        mk_prim_fn_binding("cdr", cdr),
        mk_prim_fn_binding("cons", cons),
        mk_prim_fn_binding("list", list),
        mk_prim_fn_binding("list?", is_list),
        mk_prim_fn_binding("length", length),
        mk_prim_fn_binding("append", append),
        mk_prim_fn_binding("reverse", reverse),
        mk_prim_fn_binding("member?", member),
        mk_prim_fn_binding("atom?", atom),
    ]);
    bindings.extend(accessors());
    bindings
}
