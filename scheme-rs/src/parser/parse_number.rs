extern crate nom;

use crate::{lisp_val::LispVal};

use nom::{
    branch::alt,
    character::complete::{char, digit1, hex_digit1, oct_digit1, one_of},
    combinator::{fail, opt},
    multi::many1,
    sequence::tuple,
    IResult, Parser,
};

/*--------------
-- Integer
--------------*/

fn integer_decimal(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        digit1.map(|n: &str| n.parse::<i64>().unwrap()).parse(input)
    })
}
fn integer_octal(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        oct_digit1
            .map(|n: &str| i64::from_str_radix(n, 8).unwrap())
            .parse(input)
    })
}
fn integer_hex(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        hex_digit1
            .map(|n: &str| i64::from_str_radix(n, 16).unwrap())
            .parse(input)
    })
}
fn integer_binary(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        many1(one_of("01"))
            .map(|n| i64::from_str_radix(&n.iter().collect::<String>(), 2).unwrap())
            .parse(input)
    })
}

fn integer_base(input: &str) -> IResult<&str, LispVal> {
    let (input, (_, base)) = tuple((char('#'), one_of("bdox"))).parse(input)?;
    match base {
        'b' => integer_binary(input),
        'd' => integer_decimal(input),
        'o' => integer_octal(input),
        'x' => integer_hex(input),
        _ => fail(input),
    }
}

fn positive_integer<F>(mut f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, i64>,
{
    move |input| {
        let (input, _) = opt(char('+')).parse(input)?;
        let (input, n) = f.parse(input)?;
        Ok((input, LispVal::Integer(n)))
    }
}

fn negative_integer<F>(mut f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, i64>,
{
    move |input| {
        let (input, _) = char('-').parse(input)?;
        let (input, n) = f.parse(input)?;
        Ok((input, LispVal::Integer(-n)))
    }
}

fn integer_helper<F>(input: &str, f: F) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, i64> + Clone,
{
    alt((negative_integer(f.clone()), positive_integer(f))).parse(input)
}

fn integer(input: &str) -> IResult<&str, LispVal> {
    alt((integer_decimal, integer_base)).parse(input)
}

/*--------------
-- Float
--------------*/

fn float_helper<F>(input: &str, f: F) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, f64> + Clone,
{
    alt((negative_float(f.clone()), positive_float(f))).parse(input)
}

fn float_decimal(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        digit1.map(|n: &str| n.parse::<i64>().unwrap()).parse(input)
    })
}
fn float_octal(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        oct_digit1
            .map(|n: &str| i64::from_str_radix(n, 8).unwrap())
            .parse(input)
    })
}
fn float_hex(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        hex_digit1
            .map(|n: &str| i64::from_str_radix(n, 16).unwrap())
            .parse(input)
    })
}
fn float_binary(input: &str) -> IResult<&str, LispVal> {
    integer_helper(input, |input| {
        many1(one_of("01"))
            .map(|n| i64::from_str_radix(&n.iter().collect::<String>(), 2).unwrap())
            .parse(input)
    })
}

fn float_base(input: &str) -> IResult<&str, LispVal> {
    let (input, (_, base)) = tuple((char('#'), one_of("bdox"))).parse(input)?;
    match base {
        'b' => float_binary(input),
        'd' => float_decimal(input),
        'o' => float_octal(input),
        'x' => float_hex(input),
        _ => fail(input),
    }
}

fn positive_float<F>(mut f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, f64>,
{
    move |input| {
        let (input, _) = opt(char('+')).parse(input)?;
        let (input, n) = f.parse(input)?;
        Ok((input, LispVal::Float(n)))
    }
}

fn negative_float<F>(mut f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, f64>,
{
    move |input| {
        let (input, _) = char('-').parse(input)?;
        let (input, n) = f.parse(input)?;
        Ok((input, LispVal::Float(-n)))
    }
}

fn float(input: &str) -> IResult<&str, LispVal> {
    alt((float_decimal, float_base)).parse(input)
}

/*--------------
-- Complex
--------------*/

/*--------------
-- Rational
--------------*/

pub fn number(input: &str) -> IResult<&str, LispVal> {
    alt((integer,)).parse(input)
    // alt((complex, rational, float, integer)).parse(input)
}

// parseIntegerBase :: Char -> Parser LispVal
// parseIntegerBase base =
//   case base of
//     'd' -> parseIntegerDecimal
//     'o' -> parseIntegerOctal
//     'x' -> parseIntegerHex
//     'b' -> parseIntegerBinary
//     _ -> failure "Bad integer format"
