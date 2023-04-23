extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, NestedMeta};

fn get_ident(arg: &NestedMeta) -> String {
    match arg {
        NestedMeta::Lit(Lit::Str(s)) => s.value(),
        _ => panic!("TODO"),
    }
}

#[proc_macro_attribute]
pub fn string_to_bool_binop(args: TokenStream, stream: TokenStream) -> TokenStream {
    // TODO: This is all a bit of a horrorshow
    let input = parse_macro_input!(stream as ItemFn);

    let attr_args = parse_macro_input!(args as AttributeArgs);

    let s = match &attr_args[..] {
        [m1] => get_ident(m1),
        _ => panic!("TODO"),
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let stmts = &block.stmts;

    TokenStream::from(quote! {
        #(#attrs)* #vis #sig {
            check_arity(&args, Arity::Min(1))?;
            let mut ss: Option<String> = None;
            match &args[..] {
                [LispVal::String(_)] => Ok(LispVal::Bool(true)),
                xs => {
                    for x in xs {
                        if let LispVal::String(s2) = x {
                            let s2 = s2.to_string();
                            match ss.take() {
                                None => {
                                    ss = Some(s2);
                                },
                                Some(s1) => {
                                     if !(#(#stmts)*) {
                                        return Ok(LispVal::Bool(false))
                                     }
                                     ss = Some(s2);
                                }
                            }
                        } else {
                            return Err(LispError::GenericError(format!(
                                "{}: contract violation\nexpected: string?\ngiven: {}",
                                #s,
                                x
                            )));
                        }
                    }
                    Ok(LispVal::Bool(true))
                }
                _ => unreachable!(),
            }
        }
    })
}

#[proc_macro_attribute]
pub fn num_to_bool_binop(args: TokenStream, stream: TokenStream) -> TokenStream {
    // TODO: This dont really work so good :()
    let input = parse_macro_input!(stream as ItemFn);

    let attr_args = parse_macro_input!(args as AttributeArgs);

    let s = match &attr_args[..] {
        [m1] => get_ident(m1),
        _ => panic!("TODO"),
    };

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;
    let stmts = &block.stmts;

    TokenStream::from(quote! {
        #(#attrs)* #vis #sig {
            check_arity(&args, Arity::MinMax(2, 2))?;
            let mut ss: Option<String> = None;
            if let [m, n] = &args[..] {
                match cast(m, n)? {
                    (LispVal::Integer(m), LispVal::Integer(n)) => Ok(LispVal::Bool((#(#stmts)*))),
                    (LispVal::Float(m), LispVal::Float(n)) => Ok(LispVal::Bool((#(#stmts)*))),
                    (LispVal::Rational(m), LispVal::Rational(n)) => Ok(LispVal::Bool((#(#stmts)*))),
                    (LispVal::Complex(m), LispVal::Complex(n)) => Ok(LispVal::Bool((#(#stmts)*))),
                    // TODO: Better error
                    _ => Err(LispError::GenericError(format!("Unexpected error {}", #s)))
                }
                #(#stmts)*
            } else {
                unreachable!()
            }
        }
    })
}
