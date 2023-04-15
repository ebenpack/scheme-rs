use std::rc::Rc;

use super::parser::*;
use crate::lisp_val::*;
use nom::{Err, Parser};

#[test]
fn parse_atom() {
    assert_eq!(atom.parse("#t"), Ok(("", LispVal::Bool(true))));
    assert_eq!(atom.parse("#f"), Ok(("", LispVal::Bool(false))));
    assert_eq!(
        atom.parse("foobar"),
        Ok(("", LispVal::Atom("foobar".to_string())))
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        string.parse("\"foobar\""),
        Ok(("", LispVal::String("foobar".to_string())))
    );
    assert_eq!(
        string.parse("\"foo\\\\bar\""),
        Ok(("", LispVal::String("foo\\bar".to_string())))
    );
    assert_eq!(
        string.parse("\"foo\\\"bar\""),
        Ok(("", LispVal::String("foo\"bar".to_string())))
    );
    assert_eq!(
        string.parse("\"foo\\rbar\""),
        Ok(("", LispVal::String("foo\rbar".to_string())))
    );
    assert_eq!(
        string.parse("\"foo\\tbar\""),
        Ok(("", LispVal::String("foo\tbar".to_string())))
    );
    assert_eq!(
        string.parse("\"foo\\nbar\""),
        Ok(("", LispVal::String("foo\nbar".to_string())))
    );
    assert_eq!(
        string.parse("\"foo"),
        Err(Err::Error(nom::error::Error::new(
            "",
            nom::error::ErrorKind::Tag
        )))
    );
}

#[test]
fn parse_character() {
    assert_eq!(character.parse("#\\a"), Ok(("", LispVal::Char('a'))));
    assert_eq!(character.parse("#\\newline"), Ok(("", LispVal::Char('\n'))));
    assert_eq!(character.parse("#\\space"), Ok(("", LispVal::Char(' '))));
    assert_eq!(
        character.parse("#\\altmode"),
        Ok(("", LispVal::Char('\u{1b}')))
    );
    assert_eq!(
        character.parse("#\\backnext"),
        Ok(("", LispVal::Char('\u{1f}')))
    );
    assert_eq!(
        character.parse("#\\backspace"),
        Ok(("", LispVal::Char('\u{8}')))
    );
    assert_eq!(
        character.parse("#\\call"),
        Ok(("", LispVal::Char('\u{1a}')))
    );
    assert_eq!(
        character.parse("#\\linefeed"),
        Ok(("", LispVal::Char('\n')))
    );
    assert_eq!(character.parse("#\\page"), Ok(("", LispVal::Char('\u{c}'))));
    assert_eq!(character.parse("#\\return"), Ok(("", LispVal::Char('\r'))));
    assert_eq!(
        character.parse("#\\rubout"),
        Ok(("", LispVal::Char('\u{7f}')))
    );
    assert_eq!(character.parse("#\\tab"), Ok(("", LispVal::Char('\t'))));
}

#[test]
fn parse_line_comment() {
    assert_eq!(line_comment.parse("; foo bar baz qux\n"), Ok(("", ())));
}

#[test]
fn parse_block_comment() {
    assert_eq!(block_comment.parse("#| foobar baz |#"), Ok(("", ())));
    assert_eq!(
        block_comment.parse("#| foobar #| fsjdkhfkdhs |# baz |#"),
        Ok(("", ()))
    );
}

#[test]
fn parse_number() {
    // Decimal
    assert_eq!(number.parse("1"), Ok(("", LispVal::Number(1))));
    assert_eq!(number.parse("1729"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("+1729"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("-1729"), Ok(("", LispVal::Number(-1729))));
    assert_eq!(number.parse("#d1729"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("#d+1729"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("#d-1729"), Ok(("", LispVal::Number(-1729))));
    // Binary
    assert_eq!(number.parse("#b11011000001"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("#b+11011000001"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("#b-11011000001"), Ok(("", LispVal::Number(-1729))));
    // Octal
    assert_eq!(number.parse("#o3301"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("#o+3301"), Ok(("", LispVal::Number(1729))));
    assert_eq!(number.parse("#o-3301"), Ok(("", LispVal::Number(-1729))));
    // Hex
    assert_eq!(number.parse("#xDEADBEEF"), Ok(("", LispVal::Number(3735928559))));
    assert_eq!(number.parse("#x+DEADBEEF"), Ok(("", LispVal::Number(3735928559))));
    assert_eq!(number.parse("#x-DEADBEEF"), Ok(("", LispVal::Number(-3735928559))));
}

#[test]
fn parse_bracketed() {
    assert_eq!(
        bracketed(string).parse("(\"foo\")"),
        Ok(("", LispVal::String("foo".to_string())))
    );
    assert_eq!(
        bracketed(string).parse("[\"foo\"]"),
        Ok(("", LispVal::String("foo".to_string())))
    );
    assert_eq!(
        bracketed(string).parse("{\"foo\"}"),
        Ok(("", LispVal::String("foo".to_string())))
    );
    assert_eq!(
        bracketed(string).parse("(\"foo\"]"),
        Err(Err::Error(nom::error::Error::new(
            "]",
            nom::error::ErrorKind::Char
        )))
    );
}

#[test]
fn parse_raw_list() {
    assert_eq!(
        raw_list("1 2 3"),
        Ok((
            "",
            vec![LispVal::Number(1), LispVal::Number(2), LispVal::Number(3)]
        ))
    );
    assert_eq!(raw_list(""), Ok(("", vec![])));
}

#[test]
fn parse_vector() {
    assert_eq!(
        vector("#(1 2 3)"),
        Ok((
            "",
            LispVal::Vector(Rc::new(vec![
                LispVal::Number(1),
                LispVal::Number(2),
                LispVal::Number(3)
            ]))
        ))
    );
    assert_eq!(
        vector("#(1 2 3 #[4 5 6])"),
        Ok((
            "",
            LispVal::Vector(Rc::new(vec![
                LispVal::Number(1),
                LispVal::Number(2),
                LispVal::Number(3),
                LispVal::Vector(Rc::new(vec![
                    LispVal::Number(4),
                    LispVal::Number(5),
                    LispVal::Number(6)
                ]))
            ]))
        ))
    );
}

#[test]
fn parse_lists() {
    assert_eq!(
        lists("(1 2 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Number(1),
                LispVal::Number(2),
                LispVal::Number(3)
            ]))
        ))
    );
    assert_eq!(
        lists("(1 2 3 (4 5 6))"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Number(1),
                LispVal::Number(2),
                LispVal::Number(3),
                LispVal::List(Rc::new(vec![
                    LispVal::Number(4),
                    LispVal::Number(5),
                    LispVal::Number(6)
                ]))
            ]))
        ))
    );
    assert_eq!(
        lists("(1 . 2)"),
        Ok((
            "",
            LispVal::DottedList(
                Rc::new(vec![LispVal::Number(1)]),
                Rc::new(LispVal::Number(2))
            )
        ))
    );
    assert_eq!(
        lists("(1 2 3 . 2)"),
        Ok((
            "",
            LispVal::DottedList(
                Rc::new(vec![
                    LispVal::Number(1),
                    LispVal::Number(2),
                    LispVal::Number(3)
                ]),
                Rc::new(LispVal::Number(2))
            )
        ))
    );
    assert_eq!(
        lists("(1 . 2 . 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Number(2),
                LispVal::Number(1),
                LispVal::Number(3),
            ]))
        ))
    );
    assert_eq!(
        lists("[1 . 2 . 3 4]"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Number(2),
                LispVal::Number(1),
                LispVal::Number(3),
                LispVal::Number(4),
            ]))
        ))
    );
    assert_eq!(
        lists("(1 . 2 3 4)"),
        Err(Err::Error(nom::error::Error::new(
            "3 4)",
            nom::error::ErrorKind::Char
        )))
    );
}

#[test]
fn parse_quoted() {
    assert_eq!(
        quoted("'(1 2 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Atom("quote".to_string()),
                LispVal::List(Rc::new(vec![
                    LispVal::Number(1),
                    LispVal::Number(2),
                    LispVal::Number(3),
                ]))
            ]))
        ))
    )
}

#[test]
fn parse_unquoted() {
    assert_eq!(
        unquoted(",(1 2 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Atom("unquote".to_string()),
                LispVal::List(Rc::new(vec![
                    LispVal::Number(1),
                    LispVal::Number(2),
                    LispVal::Number(3),
                ]))
            ]))
        ))
    )
}
#[test]
fn parse_quasi_quote() {
    assert_eq!(
        quasi_quote("`(1 2 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Atom("quasiquote".to_string()),
                LispVal::List(Rc::new(vec![
                    LispVal::Number(1),
                    LispVal::Number(2),
                    LispVal::Number(3),
                ]))
            ]))
        ))
    )
}
#[test]
fn parse_unquote_splicing() {
    assert_eq!(
        unquote_splicing(",@(1 2 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Atom("unquote-splicing".to_string()),
                LispVal::List(Rc::new(vec![
                    LispVal::Number(1),
                    LispVal::Number(2),
                    LispVal::Number(3),
                ]))
            ]))
        ))
    )
}
