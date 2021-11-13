use crate::lisp_val::LispVal;

pub fn is_num(val: &LispVal) -> bool {
    matches!(val, LispVal::Number(_))
}
