use std::collections::HashMap;
use std::rc::Rc;

use crate::environment::Env;
use crate::error::{LispError, LispResult};
use crate::lisp_val::{Func, LispVal};

use super::util::{bind_vars, define_var, ensure_atoms, get_heads, get_tails};

// TODO: Could eval consume val?
pub fn eval(env: &Env, val: &LispVal) -> LispResult<LispVal> {
    let val_clone = val.clone();
    match val {
        v @ LispVal::Void => Ok(v.clone()),
        v @ LispVal::String(_) => Ok(v.clone()),
        v @ LispVal::Char(_) => Ok(v.clone()),
        v @ LispVal::Integer(_) => Ok(v.clone()),
        v @ LispVal::Float(_) => Ok(v.clone()),
        v @ LispVal::Rational(_) => Ok(v.clone()),
        v @ LispVal::Complex(_) => Ok(v.clone()),
        v @ LispVal::Vector(_) => Ok(v.clone()),
        v @ LispVal::Bool(_) => Ok(v.clone()),
        LispVal::Atom(ident) => match env.lookup(ident) {
            None => Err(LispError::UnboundVar(
                "Getting an unbound variable".to_string(),
                ident.to_string(),
            )),
            Some(val) => Ok(val),
        },
        // TODO: Clone... gross :(
        LispVal::Quote(v) => Ok(LispVal::clone(v)),
        // TODO: Unquote
        LispVal::Unquote(v) => unquote(env, v),
        LispVal::QuasiQuote(v) => unquote(env, v),
        LispVal::List(xs) => match &xs[..] {
            [LispVal::Atom(ref s), ref xs] if s == "quote" => {
                eval(env, &LispVal::Quote(Rc::new(xs.clone())))
            }
            [LispVal::Atom(ref s), ref xs] if s == "quasiquote" => {
                eval(env, &LispVal::QuasiQuote(Rc::new(xs.clone())))
            }
            // [LispVal::Atom(ref s), ref xs] if s == "unquote" => {
            //     Ok(LispVal::Unquote(Rc::new(xs.clone())))
            // }
            // [LispVal::Atom(ref s), ref xs] if s == "unquote-splicing" => {
            //     Ok(LispVal::UnquoteSplicing(Rc::new(xs.clone())))
            // }
            [LispVal::Atom(ref s), ref xs @ ..] if s == "or" => {
                for x in xs.iter() {
                    let result = eval(&env.clone(), x)?;
                    if !matches!(result, LispVal::Bool(false)) {
                        return Ok(result);
                    }
                }
                Ok(LispVal::Bool(false))
            }
            [LispVal::Atom(ref s), ref xs @ ..] if s == "and" => {
                let len = xs.len();
                for (i, x) in xs.iter().enumerate() {
                    let result = eval(&env.clone(), x)?;
                    if matches!(result, LispVal::Bool(false)) {
                        return Ok(result);
                    }
                    if i == len - 1 {
                        return Ok(result);
                    }
                }
                Ok(LispVal::Bool(true))
            }
            [LispVal::Atom(ref s), predicate, consequent, alternative] if s == "if" => {
                let result = eval(&env.clone(), predicate)?;
                match result {
                    LispVal::Bool(false) => eval(&env.clone(), alternative),
                    _ => eval(&env.clone(), consequent),
                }
            }
            [LispVal::Atom(ref s), ref xs @ ..] if s == "cond" => eval_cond(env, xs),
            [LispVal::Atom(ref s), LispVal::Atom(var), form] if s == "define" => {
                let value = eval(&env.clone(), form)?;
                define_var(env.clone(), var, value)
            }
            [LispVal::Atom(ref s), val] if s == "eval" => {
                // TODO: Is that all there is?
                let val = eval(&env.clone(), val)?;
                eval(&env.clone(), &val)
            }

            [LispVal::Atom(ref s), LispVal::List(params), body @ ..] if s == "define" => {
                match &params[..] {
                    [LispVal::Atom(name), ref params @ ..] => {
                        env.bind(
                            name,
                            LispVal::Func(Func::new(
                                name.to_string(),
                                params.iter().map(|param| format!("{}", param)).collect(),
                                None, // TODO?
                                body.to_owned(),
                                env.clone(),
                            )),
                        );
                        Ok(LispVal::Void)
                    }
                    _ => Err(LispError::GenericError("TODO".to_string())),
                }
            }

            [LispVal::Atom(ref s), LispVal::List(params), body @ ..] if s == "lambda" => {
                Ok(LispVal::Func(Func::new(
                    "λ".to_string(),
                    params.iter().map(|val| format!("{}", val)).collect(),
                    None,
                    body.to_vec(),
                    env.clone(),
                )))
            }

            [LispVal::Atom(ref s), varargs @ LispVal::Atom(_), body @ ..] if s == "lambda" => {
                Ok(LispVal::Func(Func::new(
                    "λ".to_string(),
                    vec![],
                    Some(format!("{}", varargs)),
                    body.to_vec(),
                    env.clone(),
                )))
            }

            [LispVal::Atom(ref s), LispVal::DottedList(params, varargs), body @ ..]
                if s == "lambda" =>
            {
                Ok(LispVal::Func(Func::new(
                    "λ".to_string(),
                    params.iter().map(|val| format!("{}", val)).collect(),
                    Some(format!("{}", varargs)),
                    body.to_vec(),
                    env.clone(),
                )))
            }

            [LispVal::Atom(ref s), LispVal::List(pairs), body @ ..] if s == "let" => {
                let atoms = get_heads(pairs)?;
                let atoms = ensure_atoms(&atoms)?;
                let vals = get_tails(pairs)?;
                let args = eval_args(env, &vals)?;
                let bindings =
                    HashMap::from_iter(atoms.iter().zip(args).map(|(a, b)| (a.to_string(), b)));
                let env = bind_vars(env, bindings);
                eval_list(&env, body)
            }

            [LispVal::Atom(ref s), LispVal::List(pairs), body @ ..] if s == "let*" => {
                let atoms = get_heads(pairs)?;
                let atoms = ensure_atoms(&atoms)?;
                let vals = get_tails(pairs)?;

                // TODO: Ensure equal lengths
                let env = bind_vars(env, HashMap::new());
                for (atom, val) in atoms.iter().zip(&vals) {
                    let val = eval(&env, val)?;
                    env.bind(atom, val);
                }

                eval_list(&env, body)
            }

            [LispVal::Atom(ref s), LispVal::List(pairs), body @ ..] if s == "letrec" => {
                let atoms = get_heads(pairs)?;
                let atoms = ensure_atoms(&atoms)?;
                let vals = get_tails(pairs)?;

                let env = bind_vars(env, HashMap::new());
                for atom in &atoms {
                    env.bind(atom, LispVal::Void);
                }

                let args = eval_args(&env, &vals)?;

                for (atom, val) in atoms.iter().zip(&args) {
                    env.set_var(atom, val.clone());
                }

                eval_list(&env, body)
            }

            [LispVal::Atom(ref s), val] if s == "write" => {
                let val = eval(env, val)?;
                // TODO: This is supposed to take an optional third port param
                // TODO: Ports probably shouldn't really be anything too special... maybe a normal LispVal?
                let port = env
                    .ports
                    .get("default")
                    .ok_or_else(|| LispError::GenericError("Port not found".to_string()))?;
                {
                    let mut port = port.borrow_mut();
                    port.push(val);
                    port.signal();
                }
                Ok(LispVal::Void)
            }

            // TODO: Better error handling
            [LispVal::Atom(ref s), var, n, object] if s == "vector-set!" => {
                let vec = match var {
                    LispVal::Atom(var) => {
                        if let Some(LispVal::Vector(ref vec)) = env.lookup(var) {
                            // TODO: blech
                            vec.clone()
                        } else {
                            return Err(LispError::GenericError(format!(
                                "Unbound identifier: {}",
                                var
                            )))
                        }
                    }
                    LispVal::Vector(vec) => vec.clone(),
                    _ => return Err(LispError::GenericError(format!("vector-set!: contract violation\nexpected: (and/c vector? (not/c immutable?))\ngiven: {}\nargument position: 1st", var)))
                };

                let n = eval(env, n)?;
                let n = match n {
                    LispVal::Integer(n) => {
                        usize::try_from(n).map_err(|_| {
                            LispError::GenericError(format!("vector-set!: index is out of range\nindex: {}\nvalid range: [0, {}]\nvector: {}", n, s.len(), var))
                        })?
                    }
                    arg => return Err(LispError::GenericError(format!("vector-set!: contract violation\nexpected: exact-nonnegative-integer?\ngiven: {}\nargument position: 2nd", arg)))
                };
                let mut vec = (*vec).clone();
                let _ = std::mem::replace(&mut vec[n], object.clone());
                if let LispVal::Atom(var) = var {
                    env.bind(var, LispVal::Vector(Rc::new(vec)));
                }
                Ok(LispVal::Void)
            }
            // eval env (List (Atom "vector-set!":args)) =
            //   case args of
            //     [Atom var, Integer n, v] -> do
            //       Vector vec <- getVar env var
            //       if n < (fromIntegral $ length vec)
            //         then setVar env var $ Vector $ vec // [(n, v)]
            //         else throwError $ outOfBoundsError "vector-ref" n vec
            //     [a, b, c] ->
            //       throwError $ TypeMismatch "vector, integer, integer" $ List [a, b, c]
            //     a -> throwError $ NumArgs (MinMax 3 3) (length args) a
            [function, args @ ..] => {
                let function = eval(env, function)?;
                let args = args
                    .iter()
                    .map(|arg| eval(env, arg))
                    .collect::<Result<Vec<LispVal>, LispError>>()?;
                apply(function, args)
            }

            _ => Err(LispError::BadSpecialForm(
                "Unrecognized special form".to_string(),
                val_clone,
            )),
        },

        // TODO
        _ => Err(LispError::BadSpecialForm(
            "Unrecognized special form".to_string(),
            val.clone(),
        )),
    }
}

fn unquote(env: &Env, val: &LispVal) -> LispResult<LispVal> {
    match val {
        LispVal::Unquote(v) => {
            let evaled = eval(env, v)?;
            Ok(evaled)
        }
        LispVal::List(xs) => {
            let xs = &xs[..];
            let unquoted = xs
                .iter()
                .map(|x| unquote(env, x))
                .collect::<LispResult<Vec<LispVal>>>()?;
            Ok(LispVal::List(Rc::new(unquoted)))
        }
        LispVal::DottedList(xs, x) => {
            let unqouted_xs = xs
                .iter()
                .map(|x| unquote(env, x))
                .collect::<LispResult<Vec<LispVal>>>()?;
            let unquoted_x = unquote(env, x)?;

            Ok(LispVal::DottedList(
                Rc::new(unqouted_xs),
                Rc::new(unquoted_x),
            ))
        }
        x => Ok(x.clone()),
    }
}

fn eval_list(env: &Env, val: &[LispVal]) -> LispResult<LispVal> {
    for (i, x) in val.iter().enumerate() {
        let result = eval(&env.clone(), x);
        if i == val.len() - 1 {
            return result;
        }
    }
    Ok(LispVal::Void)
}

fn eval_args(env: &Env, vals: &[LispVal]) -> LispResult<Vec<LispVal>> {
    vals.iter()
        .map(|val| eval(env, val))
        .collect::<LispResult<Vec<LispVal>>>()
}

fn eval_cond(env: &Env, xs: &[LispVal]) -> LispResult<LispVal> {
    let len = xs.len();
    for (i, x) in xs.iter().enumerate() {
        match x {
            LispVal::List(xs) => {
                match &xs[..] {
                    // "else" clause is only valid in the final position
                    [LispVal::Atom(ref s), ref xs @ ..] if s == "else" => {
                        if i == len - 1 {
                            return eval_list(env, xs);
                        } else {
                            return Err(LispError::GenericError("TODO: B".to_string()));
                        }
                    }
                    arr @ [predicate, ..] => {
                        let result = eval(env, predicate)?;
                        match &result {
                            LispVal::Bool(false) => {
                                continue;
                            }
                            _ => {
                                return eval_list(env, arr);
                            }
                        }
                    }
                    _ => return Err(LispError::GenericError("TODO: A".to_string())),
                }
            }
            x => {
                return Err(LispError::GenericError(format!(
                    "cond: bad syntax (clause is not a test-value pair) in: {}",
                    x
                )))
            }
        }
    }
    Ok(LispVal::Void)
}

pub fn eval_expression_list(env: &Env, vals: Vec<LispVal>) -> LispResult<Vec<LispVal>> {
    vals.iter()
        .map(|val| eval(env, val))
        .collect::<Result<Vec<LispVal>, LispError>>()
}

fn apply(function: LispVal, args: Vec<LispVal>) -> LispResult<LispVal> {
    match function {
        LispVal::PrimitiveFunc(function) => {
            function.apply(args)
        }
        LispVal::Func(function) => {
            // TODO: Check arg lengths...
            // TODO: check varargs?
            let mut bindings = HashMap::new();
            for (param, value) in function.params.iter().zip(args) {
                bindings.insert(param.to_owned(), value);
            }
            let env = bind_vars(&function.closure, bindings);

            let body = function.body
                .iter()
                .map(|expr|
                    eval(&env, expr)
                )
                .collect::<Result<Vec<LispVal>, LispError>>()?;

            Ok(body.last().cloned().unwrap_or(LispVal::Void))
        }
        _ => {
            Err(LispError::GenericError(
                format!(
                    "application: not a procedure; expected a procedure that can be applied to arguments; given: {}",
                    function,
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::util::{get_heads, get_tails};

    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_get_head() {
        assert_eq!(
            get_heads(&vec![
                LispVal::List(Rc::new(vec![LispVal::Integer(1), LispVal::Integer(2)])),
                LispVal::List(Rc::new(vec![LispVal::Integer(3), LispVal::Integer(4)]))
            ]),
            Ok(vec![LispVal::Integer(1), LispVal::Integer(3)])
        );
    }
    #[test]
    fn test_get_tails() {
        // [List([Atom("a"), Number(5)]), List([Atom("b"), List([Atom("+"), Atom("a"), Number(10)])])]
        assert_eq!(
            get_tails(&vec![
                LispVal::List(Rc::new(vec![
                    LispVal::Atom("a".to_string()),
                    LispVal::Integer(5)
                ])),
                LispVal::List(Rc::new(vec![
                    LispVal::Atom("b".to_string()),
                    LispVal::List(Rc::new(vec![
                        LispVal::Atom("+".to_string()),
                        LispVal::Atom("a".to_string()),
                        LispVal::Integer(10)
                    ]))
                ]))
            ]),
            Ok(vec![
                LispVal::Integer(5),
                LispVal::List(Rc::new(vec![
                    LispVal::Atom("+".to_string()),
                    LispVal::Atom("a".to_string()),
                    LispVal::Integer(10)
                ]))
            ])
        )
    }
}
