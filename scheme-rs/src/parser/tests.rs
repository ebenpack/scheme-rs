use std::rc::Rc;

use super::parser::*;
use crate::lisp_val::*;
use nom::{Err, Parser};
use num::{complex::Complex64, Rational64};

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
fn parse_integer_number() {
    // Decimal
    assert_eq!(number.parse("1"), Ok(("", LispVal::Integer(1))));
    assert_eq!(number.parse("1729"), Ok(("", LispVal::Integer(1729))));
    assert_eq!(number.parse("+1729"), Ok(("", LispVal::Integer(1729))));
    assert_eq!(number.parse("-1729"), Ok(("", LispVal::Integer(-1729))));
    assert_eq!(number.parse("#d1729"), Ok(("", LispVal::Integer(1729))));
    assert_eq!(number.parse("#d+1729"), Ok(("", LispVal::Integer(1729))));
    assert_eq!(number.parse("#d-1729"), Ok(("", LispVal::Integer(-1729))));
    // Binary
    assert_eq!(
        number.parse("#b11011000001"),
        Ok(("", LispVal::Integer(1729)))
    );
    assert_eq!(
        number.parse("#b+11011000001"),
        Ok(("", LispVal::Integer(1729)))
    );
    assert_eq!(
        number.parse("#b-11011000001"),
        Ok(("", LispVal::Integer(-1729)))
    );
    // Octal
    assert_eq!(number.parse("#o3301"), Ok(("", LispVal::Integer(1729))));
    assert_eq!(number.parse("#o+3301"), Ok(("", LispVal::Integer(1729))));
    assert_eq!(number.parse("#o-3301"), Ok(("", LispVal::Integer(-1729))));
    // Hex
    assert_eq!(
        number.parse("#xDEADBEEF"),
        Ok(("", LispVal::Integer(3735928559)))
    );
    assert_eq!(
        number.parse("#x+DEADBEEF"),
        Ok(("", LispVal::Integer(3735928559)))
    );
    assert_eq!(
        number.parse("#x-DEADBEEF"),
        Ok(("", LispVal::Integer(-3735928559)))
    );
}

#[test]
fn parse_float_number() {
    // Decimal
    assert_eq!(number.parse("1."), Ok(("", LispVal::Float(1.0))));
    assert_eq!(number.parse("1.0"), Ok(("", LispVal::Float(1.0))));
    assert_eq!(number.parse("1.2"), Ok(("", LispVal::Float(1.2))));
    assert_eq!(number.parse("1729."), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("1729.0"), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("1729.9"), Ok(("", LispVal::Float(1729.9))));
    assert_eq!(number.parse("+1729."), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("+1729.0"), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("+1729.6"), Ok(("", LispVal::Float(1729.6))));
    assert_eq!(
        number.parse("1729.00012345"),
        Ok(("", LispVal::Float(1729.00012345)))
    );
    assert_eq!(number.parse("-1729."), Ok(("", LispVal::Float(-1729.0))));
    assert_eq!(number.parse("-1729.0"), Ok(("", LispVal::Float(-1729.0))));
    assert_eq!(number.parse("-1729.1"), Ok(("", LispVal::Float(-1729.1))));
    assert_eq!(
        number.parse("-1729.0000123"),
        Ok(("", LispVal::Float(-1729.0000123)))
    );
    assert_eq!(number.parse("#d1729."), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("#d1729.0"), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("#d1729.1"), Ok(("", LispVal::Float(1729.1))));
    assert_eq!(
        number.parse("#d1729.0000123"),
        Ok(("", LispVal::Float(1729.0000123)))
    );
    assert_eq!(number.parse("#d+1729."), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("#d+1729.0"), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("#d+1729.4"), Ok(("", LispVal::Float(1729.4))));
    assert_eq!(number.parse("#d-1729."), Ok(("", LispVal::Float(-1729.0))));
    assert_eq!(number.parse("#d-1729.0"), Ok(("", LispVal::Float(-1729.0))));
    assert_eq!(number.parse("#d-1729.8"), Ok(("", LispVal::Float(-1729.8))));
    // Binary
    assert_eq!(
        number.parse("#b11011000001."),
        Ok(("", LispVal::Float(1729.0)))
    );
    assert_eq!(
        number.parse("#b11011000001.0"),
        Ok(("", LispVal::Float(1729.0)))
    );
    assert_eq!(
        number.parse("#b11011000001.001"),
        Ok(("", LispVal::Float(1729.125)))
    );
    assert_eq!(
        number.parse("#b+11011000001."),
        Ok(("", LispVal::Float(1729.0)))
    );
    assert_eq!(
        number.parse("#b+11011000001.0"),
        Ok(("", LispVal::Float(1729.0)))
    );
    assert_eq!(
        number.parse("#b+11011000001.001"),
        Ok(("", LispVal::Float(1729.125)))
    );
    assert_eq!(
        number.parse("#b-11011000001."),
        Ok(("", LispVal::Float(-1729.0)))
    );
    assert_eq!(
        number.parse("#b-11011000001.0"),
        Ok(("", LispVal::Float(-1729.0)))
    );
    assert_eq!(
        number.parse("#b-11011000001.001"),
        Ok(("", LispVal::Float(-1729.125)))
    );
    // Octal
    assert_eq!(number.parse("#o3301."), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("#o3301.0"), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(
        number.parse("#o3301.732"),
        Ok(("", LispVal::Float(1729.92578125)))
    );
    assert_eq!(number.parse("#o+3301."), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(number.parse("#o+3301.0"), Ok(("", LispVal::Float(1729.0))));
    assert_eq!(
        number.parse("#o+3301.732"),
        Ok(("", LispVal::Float(1729.92578125)))
    );
    assert_eq!(number.parse("#o-3301."), Ok(("", LispVal::Float(-1729.0))));
    assert_eq!(number.parse("#o-3301.0"), Ok(("", LispVal::Float(-1729.0))));
    assert_eq!(
        number.parse("#o-3301.227"),
        Ok(("", LispVal::Float(-1729.294921875)))
    );
    // Hex
    assert_eq!(
        number.parse("#xDEADBEEF."),
        Ok(("", LispVal::Float(3735928559.0)))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.0"),
        Ok(("", LispVal::Float(3735928559.0)))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.CAFE"),
        Ok(("", LispVal::Float(3735928559.792938)))
    );
    assert_eq!(
        number.parse("#x+DEADBEEF."),
        Ok(("", LispVal::Float(3735928559.0)))
    );
    assert_eq!(
        number.parse("#x+DEADBEEF.0"),
        Ok(("", LispVal::Float(3735928559.0)))
    );
    assert_eq!(
        number.parse("#x+DEADBEEF.CAFE"),
        Ok(("", LispVal::Float(3735928559.792938)))
    );
    assert_eq!(
        number.parse("#x-DEADBEEF."),
        Ok(("", LispVal::Float(-3735928559.0)))
    );
    assert_eq!(
        number.parse("#x-DEADBEEF.0"),
        Ok(("", LispVal::Float(-3735928559.0)))
    );
    assert_eq!(
        number.parse("#x-DEADBEEF.CAFEBABE"),
        Ok(("", LispVal::Float(-3735928559.792949)))
    );
}

#[test]
fn parse_complex_number() {
    // Decimal
    assert_eq!(
        number.parse("1+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("+1+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1.+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1.0+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1+0.0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1.0+0.0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1.+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("+1.0+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("+1.+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("+1.0+0.0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("+1.+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("-1+0i"),
        Ok(("", LispVal::Complex(Complex64::new(-1.0, 0.0))))
    );
    assert_eq!(
        number.parse("-1.0+0i"),
        Ok(("", LispVal::Complex(Complex64::new(-1.0, 0.0))))
    );
    assert_eq!(
        number.parse("-1.+0i"),
        Ok(("", LispVal::Complex(Complex64::new(-1.0, 0.0))))
    );
    assert_eq!(
        number.parse("-1+0.0i"),
        Ok(("", LispVal::Complex(Complex64::new(-1.0, 0.0))))
    );
    assert_eq!(
        number.parse("-1+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(-1.0, 0.0))))
    );
    assert_eq!(
        number.parse("1-0i"),
        Ok(("", LispVal::Complex(Complex64::new(1.0, 0.0))))
    );
    assert_eq!(
        number.parse("0-1.0i"),
        Ok(("", LispVal::Complex(Complex64::new(0.0, -1.0))))
    );
    assert_eq!(
        number.parse("0-1.i"),
        Ok(("", LispVal::Complex(Complex64::new(0.0, -1.0))))
    );
    assert_eq!(
        number.parse("-1-1i"),
        Ok(("", LispVal::Complex(Complex64::new(-1.0, -1.0))))
    );
    // Binary
    assert_eq!(
        number.parse("#b11011000001+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#b11011000001.+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#b11011000001+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#b11011000001+11011000001i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 1729.0))))
    );
    assert_eq!(
        number.parse("#b11011000001.+11011000001i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 1729.0))))
    );
    assert_eq!(
        number.parse("#b11011000001+11011000001.i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 1729.0))))
    );
    assert_eq!(
        number.parse("#b+11011000001+11011000001i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 1729.0))))
    );
    assert_eq!(
        number.parse("#b-11011000001+11011000001i"),
        Ok(("", LispVal::Complex(Complex64::new(-1729.0, 1729.0))))
    );
    assert_eq!(
        number.parse("#b11011000001-11011000001i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, -1729.0))))
    );
    assert_eq!(
        number.parse("#b-11011000001-11011000001i"),
        Ok(("", LispVal::Complex(Complex64::new(-1729.0, -1729.0))))
    );
    assert_eq!(
        number.parse("#b-11011000001.10000011011-11011000001.10000011011i"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(-1729.51318359375, -1729.51318359375))
        ))
    );
    assert_eq!(
        number.parse("#b11011000001.10000011011-11011000001.10000011011i"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(1729.51318359375, -1729.51318359375))
        ))
    );
    assert_eq!(
        number.parse("#b+11011000001.10000011011-11011000001.10000011011i"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(1729.51318359375, -1729.51318359375))
        ))
    );
    // Octal
    assert_eq!(
        number.parse("#o3301+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301.0+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301.+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301+0.0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301.732+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.92578125, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301.7320+0i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.92578125, 0.0))))
    );
    assert_eq!(
        number.parse("#o3301+0.732i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.92578125))))
    );
    assert_eq!(
        number.parse("#o3301.+0.732i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.92578125))))
    );
    assert_eq!(
        number.parse("#o3301.0+0.732i"),
        Ok(("", LispVal::Complex(Complex64::new(1729.0, 0.92578125))))
    );
    // Hex
    assert_eq!(
        number.parse("#xDEADBEEF+0i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.0, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.0+0i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.0, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.+0i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.0, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF+0.0i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.0, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.0, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.CAFE+0.i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.792938, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.CAFE+0i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.792938, 0.0))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF+0.CAFEi"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(3735928559.0, 0.792938232421875))
        ))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.+0.CAFEi"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(3735928559.0, 0.792938232421875))
        ))
    );
    assert_eq!(
        number.parse("#xDEADBEEF.0-0.CAFEi"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(3735928559.0, -0.792938232421875))
        ))
    );
    assert_eq!(
        number.parse("#x-DEADBEEF.0-0.CAFEi"),
        Ok((
            "",
            LispVal::Complex(Complex64::new(-3735928559.0, -0.792938232421875))
        ))
    );
    assert_eq!(
        number.parse("#x+DEADBEEF.0-0.i"),
        Ok(("", LispVal::Complex(Complex64::new(3735928559.0, 0.0))))
    );
}

#[test]
fn parse_rational_number() {
    // Decimal
    assert_eq!(
        number.parse("1/1"),
        Ok(("", LispVal::Rational(Rational64::new(1, 1))))
    );
    assert_eq!(
        number.parse("1729/3"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 3))))
    );
    assert_eq!(
        number.parse("+1729/3"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 3))))
    );
    assert_eq!(
        number.parse("-1729/3"),
        Ok(("", LispVal::Rational(Rational64::new(-1729, 3))))
    );
    // Binary
    assert_eq!(
        number.parse("#b11011000001/1"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 1))))
    );
    assert_eq!(
        number.parse("#b11011000001/11"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 3))))
    );
    assert_eq!(
        number.parse("#b+11011000001/11"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 3))))
    );
    assert_eq!(
        number.parse("#b-11011000001/11"),
        Ok(("", LispVal::Rational(Rational64::new(-1729, 3))))
    );
    // Octal
    assert_eq!(
        number.parse("#o3301/1"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 1))))
    );
    assert_eq!(
        number.parse("#o3301/3"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 3))))
    );
    assert_eq!(
        number.parse("#o+3301/11"),
        Ok(("", LispVal::Rational(Rational64::new(1729, 9))))
    );
    assert_eq!(
        number.parse("#o-3301/11"),
        Ok(("", LispVal::Rational(Rational64::new(-1729, 9))))
    );
    // Hex
    assert_eq!(
        number.parse("#xDEADBEEF/1"),
        Ok(("", LispVal::Rational(Rational64::new(3735928559, 1))))
    );
    assert_eq!(
        number.parse("#xDEADBEEF/CAFE"),
        Ok(("", LispVal::Rational(Rational64::new(3735928559, 51966))))
    );
    assert_eq!(
        number.parse("#x+DEADBEEF/CAFE"),
        Ok(("", LispVal::Rational(Rational64::new(3735928559, 51966))))
    );
    assert_eq!(
        number.parse("#x-DEADBEEF/CAFE"),
        Ok(("", LispVal::Rational(Rational64::new(-3735928559, 51966))))
    );
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
            vec![
                LispVal::Integer(1),
                LispVal::Integer(2),
                LispVal::Integer(3)
            ]
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
                LispVal::Integer(1),
                LispVal::Integer(2),
                LispVal::Integer(3)
            ]))
        ))
    );
    assert_eq!(
        vector("#(1 2 3 #[4 5 6])"),
        Ok((
            "",
            LispVal::Vector(Rc::new(vec![
                LispVal::Integer(1),
                LispVal::Integer(2),
                LispVal::Integer(3),
                LispVal::Vector(Rc::new(vec![
                    LispVal::Integer(4),
                    LispVal::Integer(5),
                    LispVal::Integer(6)
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
                LispVal::Integer(1),
                LispVal::Integer(2),
                LispVal::Integer(3)
            ]))
        ))
    );
    assert_eq!(
        lists("(1 2 3 (4 5 6))"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Integer(1),
                LispVal::Integer(2),
                LispVal::Integer(3),
                LispVal::List(Rc::new(vec![
                    LispVal::Integer(4),
                    LispVal::Integer(5),
                    LispVal::Integer(6)
                ]))
            ]))
        ))
    );
    assert_eq!(
        lists("(1 . 2)"),
        Ok((
            "",
            LispVal::DottedList(
                Rc::new(vec![LispVal::Integer(1)]),
                Rc::new(LispVal::Integer(2))
            )
        ))
    );
    assert_eq!(
        lists("(1 2 3 . 2)"),
        Ok((
            "",
            LispVal::DottedList(
                Rc::new(vec![
                    LispVal::Integer(1),
                    LispVal::Integer(2),
                    LispVal::Integer(3)
                ]),
                Rc::new(LispVal::Integer(2))
            )
        ))
    );
    assert_eq!(
        lists("(1 . 2 . 3)"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Integer(2),
                LispVal::Integer(1),
                LispVal::Integer(3),
            ]))
        ))
    );
    assert_eq!(
        lists("[1 . 2 . 3 4]"),
        Ok((
            "",
            LispVal::List(Rc::new(vec![
                LispVal::Integer(2),
                LispVal::Integer(1),
                LispVal::Integer(3),
                LispVal::Integer(4),
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
                    LispVal::Integer(1),
                    LispVal::Integer(2),
                    LispVal::Integer(3),
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
                    LispVal::Integer(1),
                    LispVal::Integer(2),
                    LispVal::Integer(3),
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
                    LispVal::Integer(1),
                    LispVal::Integer(2),
                    LispVal::Integer(3),
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
                    LispVal::Integer(1),
                    LispVal::Integer(2),
                    LispVal::Integer(3),
                ]))
            ]))
        ))
    )
}
