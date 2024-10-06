use crate::environment::{Bindings, Env};
use crate::error::{LispError, LispResult};
use crate::lisp_val::LispVal;

pub fn define_var(env: Env, key: &str, value: LispVal) -> LispResult<LispVal> {
    let already_locally_bound = env.is_bound_local(key);
    if already_locally_bound {
        Err(LispError::GenericError(format!(
            "Duplicate definition for identifier in: {}",
            key
        )))
    } else {
        env.bind(key, value);
        Ok(LispVal::Void)
    }
}

pub fn bind_vars(env: &Env, bindings: Bindings) -> Env {
    env.push_frame(bindings)
}

pub fn get_heads(xs: &[LispVal]) -> LispResult<Vec<LispVal>> {
    match xs {
        [] => Ok(vec![]),
        [LispVal::List(xs), ys @ ..] => match &xs[..] {
            [x, ..] => {
                let mut result = get_heads(ys)?;
                result.insert(0, x.clone());
                Ok(result)
            }
            _ => Err(LispError::GenericError(
                "Unexpected error (getHeads)".to_string(),
            )),
        },
        _ => Err(LispError::GenericError(
            "Unexpected error (getHeads)".to_string(),
        )),
    }
}

pub fn get_tails(xs: &[LispVal]) -> LispResult<Vec<LispVal>> {
    match xs {
        [] => Ok(vec![]),
        [LispVal::List(xs), ys @ ..] => match &xs[..] {
            [_, xs @ ..] => {
                let mut result = get_tails(ys)?;
                let mut new_list = xs.to_vec();
                new_list.append(&mut result);
                Ok(new_list)
            }
            _ => Err(LispError::GenericError(
                "Unexpected error (getHeads)".to_string(),
            )),
        },
        _ => Err(LispError::GenericError(
            "Unexpected error (getHeads)".to_string(),
        )),
    }
}

pub fn ensure_atoms(atoms: &[LispVal]) -> LispResult<Vec<String>> {
    atoms
        .iter()
        .map(extract_var)
        .collect::<LispResult<Vec<String>>>()
}

pub fn extract_var(val: &LispVal) -> LispResult<String> {
    match val {
        LispVal::Atom(atom) => Ok(atom.to_string()),
        _ => Err(LispError::TypeMismatch(
            "Expected atom".to_string(),
            val.clone(),
        )),
    }
}
