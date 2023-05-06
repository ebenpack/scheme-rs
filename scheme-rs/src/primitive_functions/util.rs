use crate::error::{Arity, LispError, LispResult};
use crate::lisp_val::{prim_func, LispVal};

pub fn mk_prim_fn_binding(
    name: &str,
    func: fn(Vec<LispVal>) -> LispResult<LispVal>,
) -> (String, LispVal) {
    (name.to_string(), prim_func(name.to_string(), func))
}

pub fn check_arity(args: &[LispVal], arity: Arity) -> LispResult<()> {
    let len = i8::try_from(args.len())
        .map_err(|_| LispError::GenericError("weird argument length".to_string()))?;
    match arity {
        Arity::Min(min) => {
            if len < min {
                Err(LispError::NumArgs(arity, len, args.to_vec()))
            } else {
                Ok(())
            }
        }
        Arity::MinMax(min, max) => {
            if len < min || len > max {
                Err(LispError::NumArgs(arity, len, args.to_vec()))
            } else {
                Ok(())
            }
        }
    }
}
