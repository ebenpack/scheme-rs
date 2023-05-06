extern crate nom;

use crate::lisp_val::LispVal;

use nom::{
    branch::alt,
    character::complete::{
        char, digit0, digit1, hex_digit0, hex_digit1, none_of, oct_digit0, oct_digit1, one_of,
    },
    combinator::{fail, opt, peek},
    multi::{many0, many1},
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};
use num::{complex::Complex64, Rational64};

enum Base {
    Binary,
    Decimal,
    Hex,
    Octal,
}

fn parse_base(input: &str) -> IResult<&str, Base> {
    let (input, base) = one_of("bdox").parse(input)?;

    let base = match base {
        'b' => Base::Binary,
        'd' => Base::Decimal,
        'o' => Base::Octal,
        'x' => Base::Hex,
        _ => unreachable!(),
    };

    Ok((input, base))
}

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
    let (input, (_, base)) = tuple((char('#'), parse_base)).parse(input)?;
    match base {
        Base::Binary => integer_binary(input),
        Base::Decimal => integer_decimal(input),
        Base::Octal => integer_octal(input),
        Base::Hex => integer_hex(input),
    }
}

fn positive_integer<F>(f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, i64> + Clone,
{
    move |input| {
        let (input, n) = preceded(opt(char('+')), f.clone()).parse(input)?;
        Ok((input, LispVal::Integer(n)))
    }
}

fn negative_integer<F>(f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, i64> + Clone,
{
    move |input| {
        let (input, n) = preceded(char('-'), f.clone()).parse(input)?;
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

fn float_converter(m: f64, base: f64, size: i32) -> f64 {
    let mut size = size;
    let mut m = m;
    while size > 0 {
        m /= base;
        size -= 1;
    }
    m
}

// TODO: DRY these... function or macro if absolutely necessary
fn float_decimal(input: &str) -> IResult<&str, LispVal> {
    float_helper(input, |input| {
        separated_pair(digit1, char('.'), digit0)
            .map(|(n, m): (&str, &str)| {
                let n = n.parse::<i64>().unwrap();
                let (size, m) = if m.is_empty() {
                    (1, 0)
                } else {
                    let size = m.chars().count();
                    (size, m.parse::<i64>().unwrap())
                };
                let base: f64 = 10.0;
                n as f64 + float_converter(m as f64, base, size as i32)
            })
            .parse(input)
    })
}

fn float_octal(input: &str) -> IResult<&str, LispVal> {
    float_helper(input, |input| {
        separated_pair(oct_digit1, char('.'), oct_digit0)
            .map(|(n, m)| {
                let n = i64::from_str_radix(n, 8).unwrap();
                let (size, m) = if m.is_empty() {
                    (1, 0)
                } else {
                    let size = m.chars().count();
                    (size, i64::from_str_radix(m, 8).unwrap())
                };
                let base: f64 = 8.0;
                n as f64 + float_converter(m as f64, base, size as i32)
            })
            .parse(input)
    })
}

fn float_hex(input: &str) -> IResult<&str, LispVal> {
    float_helper(input, |input| {
        separated_pair(hex_digit1, char('.'), hex_digit0)
            .map(|(n, m)| {
                let n = i64::from_str_radix(n, 16).unwrap();
                let (size, m) = if m.is_empty() {
                    (1, 0)
                } else {
                    let size = m.chars().count();
                    (size, i64::from_str_radix(m, 16).unwrap())
                };
                let base: f64 = 16.0;
                n as f64 + float_converter(m as f64, base, size as i32)
            })
            .parse(input)
    })
}

fn float_binary(input: &str) -> IResult<&str, LispVal> {
    float_helper(input, |input| {
        separated_pair(many1(one_of("01")), char('.'), many0(one_of("01")))
            .map(|(n, m)| {
                let n = n.iter().collect::<String>();
                let n = i64::from_str_radix(&n, 2).unwrap();
                let m = m.iter().collect::<String>();
                let (size, m) = if m.is_empty() {
                    (1, 0)
                } else {
                    let size = m.chars().count();
                    (size, i64::from_str_radix(&m, 2).unwrap())
                };
                let base: i32 = 2;
                n as f64 + ((m as f64) / base.pow(size as u32) as f64)
            })
            .parse(input)
    })
}

fn float_base(input: &str) -> IResult<&str, LispVal> {
    let (input, base) = preceded(char('#'), parse_base).parse(input)?;
    match base {
        Base::Binary => float_binary(input),
        Base::Decimal => float_decimal(input),
        Base::Octal => float_octal(input),
        Base::Hex => float_hex(input),
    }
}

fn positive_float<F>(f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, f64> + Clone,
{
    move |input| {
        let (input, n) = preceded(opt(char('+')), f.clone()).parse(input)?;
        Ok((input, LispVal::Float(n)))
    }
}

fn negative_float<F>(f: F) -> impl FnMut(&str) -> IResult<&str, LispVal>
where
    F: Fn(&str) -> IResult<&str, f64> + Clone,
{
    move |input| {
        let (input, n) = preceded(char('-'), f.clone()).parse(input)?;
        Ok((input, LispVal::Float(-n)))
    }
}

fn float(input: &str) -> IResult<&str, LispVal> {
    alt((float_decimal, float_base)).parse(input)
}

/*--------------
-- Complex
--------------*/

fn complex(input: &str) -> IResult<&str, LispVal> {
    // Store the base for later if it's there, but don't consume it
    // b/c float/integer will need it
    let (input, base) = peek(opt(preceded(char('#'), parse_base))).parse(input)?;
    let (input, m) = alt((float, integer)).parse(input)?;

    // Just enforce that there is a +/-, because they're optional for
    // the various float/integer parsers
    let (input, _) = peek(alt((char('-'), char('+')))).parse(input)?;
    let (input, n) = match base {
        Some(Base::Binary) => alt((float_binary, integer_binary)).parse(input),
        Some(Base::Decimal) => alt((float_decimal, integer_decimal)).parse(input),
        Some(Base::Octal) => alt((float_octal, integer_octal)).parse(input),
        Some(Base::Hex) => alt((float_hex, integer_hex)).parse(input),
        None => alt((float_decimal, integer_decimal)).parse(input),
    }?;
    let (input, _) = char('i').parse(input)?;

    match (m, n) {
        (LispVal::Float(m), LispVal::Float(n)) => {
            Ok((input, LispVal::Complex(Complex64::new(m, n))))
        }
        (LispVal::Float(m), LispVal::Integer(n)) => {
            Ok((input, LispVal::Complex(Complex64::new(m, n as f64))))
        }
        (LispVal::Integer(m), LispVal::Float(n)) => {
            Ok((input, LispVal::Complex(Complex64::new(m as f64, n))))
        }
        (LispVal::Integer(m), LispVal::Integer(n)) => {
            Ok((input, LispVal::Complex(Complex64::new(m as f64, n as f64))))
        }
        _ => unreachable!(),
    }
}

/*--------------
-- Rational
--------------*/

fn rational(input: &str) -> IResult<&str, LispVal> {
    // Store the base for later if it's there, but don't consume it
    // b/c float/integer will need it
    let (input, base) = peek(opt(preceded(char('#'), parse_base))).parse(input)?;
    let (input, m) = integer.parse(input)?;

    let (input, _) = char('/').parse(input)?;
    let (input, _) = peek(none_of("+-")).parse(input)?;

    let (input, n) = match base {
        Some(Base::Binary) => integer_binary.parse(input),
        Some(Base::Decimal) => integer_decimal.parse(input),
        Some(Base::Octal) => integer_octal.parse(input),
        Some(Base::Hex) => integer_hex.parse(input),
        None => integer_decimal.parse(input),
    }?;

    match (m, n) {
        (LispVal::Integer(m), LispVal::Integer(n)) => {
            Ok((input, LispVal::Rational(Rational64::new(m, n))))
        }
        _ => unreachable!(),
    }
}

pub fn number(input: &str) -> IResult<&str, LispVal> {
    alt((complex, rational, float, integer)).parse(input)
}
