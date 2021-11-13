mod lisp_val;
#[cfg(test)]
mod tests;
pub use lisp_val::{prim_func, Func, LispVal, PrimitiveFunc};
