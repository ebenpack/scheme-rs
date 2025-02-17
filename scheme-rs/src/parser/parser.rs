extern crate nom;

use std::rc::Rc;

use crate::lisp_val::LispVal;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0, multispace1, newline, none_of, one_of},
    combinator::fail,
    error::ParseError,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, terminated, tuple},
    Err, IResult, Parser,
};

pub use super::parse_number::number;

pub fn letter(input: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

pub fn digit(input: &str) -> IResult<&str, char> {
    one_of("0123456789")(input)
}

pub fn symbol(input: &str) -> IResult<&str, char> {
    one_of("!#$%&|*+-/:<=>?@^_~")(input)
}

pub fn match_bracket(open: char) -> impl FnMut(&str) -> IResult<&str, char> {
    move |input| {
        let p = match open {
            '(' => char(')'),
            '[' => char(']'),
            '{' => char('}'),
            _ => return fail(input),
        };
        p(input)
    }
}

fn end_by<'a, O1, O2, E, F, G>(p: F, sep: G) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O1>, E>
where
    F: Parser<&'a str, O1, E>,
    G: Parser<&'a str, O2, E>,
    E: ParseError<&'a str>,
{
    many0(terminated(p, sep))
}

pub fn bracketed<'i, O, E: ParseError<&'i str>, F>(
    mut p: F,
) -> impl FnMut(&'i str) -> IResult<&str, O, E>
where
    F: Parser<&'i str, O, E>,
    nom::Err<E>: From<nom::Err<nom::error::Error<&'i str>>>,
{
    move |input| {
        let (input, open) = alt((char('('), char('['), char('{')))(input)?;
        let (input, _) = multispace0.parse(input)?;
        let (input, val) = p.parse(input)?;
        let (input, _) = multispace0.parse(input)?;
        let (input, _) = match_bracket(open).parse(input)?;
        Ok((input, val))
    }
}

pub fn escaped_char(input: &str) -> IResult<&str, char> {
    let (input, (_, c)) = tuple((char('\\'), one_of("\\\"nrt"))).parse(input)?;
    let val = match c {
        '\\' => c,
        '"' => c,
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        _ => return fail(input),
    };
    Ok((input, val))
}

pub fn string(input: &str) -> IResult<&str, LispVal> {
    let (input, val) = delimited(
        tag("\""),
        many0(alt((escaped_char, none_of("\"\\")))),
        tag("\""),
    )
    .parse(input)?;
    Ok((input, LispVal::String(val.iter().collect::<String>())))
}

pub fn atom(input: &str) -> IResult<&str, LispVal> {
    let (input, (first, rest)) =
        tuple((alt((letter, symbol)), many0(alt((letter, digit, symbol))))).parse(input)?;
    let atom = format!("{}{}", first, rest.iter().collect::<String>());
    Ok((
        input,
        match atom.as_str() {
            "#t" => LispVal::Bool(true),
            "#f" => LispVal::Bool(false),
            _ => LispVal::Atom(atom),
        },
    ))
}

pub fn character(input: &str) -> IResult<&str, LispVal> {
    // -- TODO: Meta-, bucky-bit stuff
    let (input, (_, c)) = tuple((tag("#\\"), many1(letter))).parse(input)?;
    let s = c.iter().collect::<String>();
    if s.len() == 1 {
        let val = s
            .chars()
            .next()
            .ok_or_else(|| Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Char)))?;
        Ok((input, LispVal::Char(val)))
    } else {
        let val = match s.as_str() {
            "newline" => LispVal::Char('\n'),
            "space" => LispVal::Char(' '),
            "altmode" => LispVal::Char(27u8 as char),
            "backnext" => LispVal::Char(31u8 as char),
            "backspace" => LispVal::Char(8u8 as char),
            "call" => LispVal::Char(26u8 as char),
            "linefeed" => LispVal::Char(10u8 as char),
            "page" => LispVal::Char(12u8 as char),
            "return" => LispVal::Char(13u8 as char),
            "rubout" => LispVal::Char(127u8 as char),
            "tab" => LispVal::Char(9u8 as char),
            _ => {
                return Err(Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Char,
                )))
            }
        };
        Ok((input, val))
    }
}

pub fn line_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((char(';'), terminated(is_not("\n"), newline))).parse(input)?;
    Ok((input, ()))
}

pub fn take_until_unmatched<'a>(
    opening_bracket: &'a str,
    closing_bracket: &'a str,
) -> impl Fn(&'a str) -> IResult<&str, &str> {
    enum Match {
        OpeningBracket(usize),
        ClosingBracket(usize),
        Escape(usize),
    }
    fn find(s: &str, opening_bracket: &str, closing_bracket: &str) -> Option<Match> {
        if let Some(s) = s.find(opening_bracket) {
            return Some(Match::OpeningBracket(s));
        };
        if let Some(s) = s.find(closing_bracket) {
            return Some(Match::ClosingBracket(s));
        };
        if let Some(s) = s.find('\\') {
            return Some(Match::Escape(s));
        }
        None
    }

    move |i: &str| {
        let mut index = 0;
        let mut bracket_counter = 0;
        while let Some(m) = find(&i[index..], opening_bracket, closing_bracket) {
            match m {
                Match::OpeningBracket(n) => {
                    bracket_counter += 1;
                    index += n;
                    index += opening_bracket.len();
                }
                Match::ClosingBracket(n) => {
                    bracket_counter -= 1;
                    index += n;
                    index += closing_bracket.len();
                }
                Match::Escape(n) => {
                    index += n;
                    index += '\\'.len_utf8();
                    let mut c = i.chars();
                    c.next();
                    let c = c.next().unwrap_or_default();
                    index += c.len_utf8();
                }
            }
            if bracket_counter == -1 {
                index -= closing_bracket.len();
                return Ok((&i[index..], &i[0..index]));
            };
        }

        if bracket_counter == 0 {
            Ok(("", i))
        } else {
            Err(Err::Error(nom::error::Error::new(
                "",
                nom::error::ErrorKind::TakeUntil,
            )))
        }
    }
}

pub fn block_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) =
        delimited(tag("#|"), take_until_unmatched("#|", "|#"), tag("|#")).parse(input)?;

    Ok((input, ()))
}

pub fn comment(input: &str) -> IResult<&str, LispVal> {
    let (input, _) = alt((line_comment, block_comment)).parse(input)?;
    Ok((input, LispVal::Void))
}

pub fn vector(input: &str) -> IResult<&str, LispVal> {
    let (input, (_, list)) = tuple((char('#'), bracketed(raw_list))).parse(input)?;
    Ok((input, LispVal::Vector(Rc::new(list))))
}

pub fn quoted(input: &str) -> IResult<&str, LispVal> {
    let (input, (_, q)) = tuple((char('\''), expression)).parse(input)?;
    Ok((input, LispVal::Quote(Rc::new(q))))
}

pub fn quasi_quote(input: &str) -> IResult<&str, LispVal> {
    let (input, (_, q)) = tuple((char('`'), expression)).parse(input)?;
    Ok((input, LispVal::QuasiQuote(Rc::new(q))))
}

pub fn unquoted(input: &str) -> IResult<&str, LispVal> {
    // TODO try?
    let (input, (_, q)) = tuple((char(','), expression)).parse(input)?;
    Ok((input, LispVal::Unquote(Rc::new(q))))
}

pub fn unquote_splicing(input: &str) -> IResult<&str, LispVal> {
    // TODO try?
    let (input, (_, q)) = tuple((tag(",@"), expression)).parse(input)?;
    Ok((input, LispVal::UnquoteSplicing(Rc::new(q))))
}

pub fn raw_list(input: &str) -> IResult<&str, Vec<LispVal>> {
    let (input, (_, list, _)) = tuple((
        multispace0,
        separated_list0(multispace1, expression),
        multispace0,
    ))
    .parse(input)?;
    Ok((input, list))
}

pub fn list(input: &str) -> IResult<&str, LispVal> {
    raw_list.map(|x| LispVal::List(Rc::new(x))).parse(input)
}

pub fn dotted_list(input: &str) -> IResult<&str, LispVal> {
    let (input, (_, head, _, _, tail, _)) = tuple((
        multispace0,
        end_by(expression, multispace1),
        char('.'),
        multispace1,
        expression,
        multispace0,
    ))
    .parse(input)?;
    let result = match tail {
        LispVal::DottedList(xs, x) => {
            let mut ys = Vec::with_capacity(head.len() + xs.len());
            for x in head {
                ys.push(x);
            }
            for x in xs.iter() {
                ys.push(x.to_owned());
            }
            LispVal::DottedList(Rc::new(ys), x)
        }
        LispVal::List(xs) => {
            let mut ys = Vec::with_capacity(head.len() + xs.len());
            for x in head {
                ys.push(x);
            }
            for x in xs.iter() {
                ys.push(x.to_owned());
            }
            LispVal::List(Rc::new(ys))
        }
        _ => LispVal::DottedList(Rc::new(head), Rc::new(tail)),
    };
    Ok((input, result))
}

pub fn two_dotted_list(input: &str) -> IResult<&str, LispVal> {
    // TODO: Is this really needed?
    let (input, (_, mut head, _, _, mid, _, _, _, mut tail, _)) = tuple((
        multispace0,
        end_by(expression, multispace1),
        char('.'),
        multispace1,
        expression,
        multispace1,
        char('.'),
        multispace1,
        separated_list0(tag(" "), expression),
        multispace0,
    ))
    .parse(input)?;
    if head.is_empty() || tail.is_empty() {
        // TODO: Better error
        Err(Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        )))
    } else {
        head.append(&mut tail);
        head.insert(0, mid);
        Ok((input, LispVal::List(Rc::new(head))))
    }
}

pub fn lists(input: &str) -> IResult<&str, LispVal> {
    bracketed(alt((two_dotted_list, dotted_list, list))).parse(input)
}

pub fn expression(input: &str) -> IResult<&str, LispVal> {
    alt((
        lists,
        vector,
        comment,
        number,
        character,
        atom,
        string,
        quoted,
        quasi_quote,
        unquoted,
        unquote_splicing,
    ))
    .parse(input)
}

pub fn expression_list(input: &str) -> IResult<&str, Vec<LispVal>> {
    let (input, _) = multispace0.parse(input)?;
    end_by(expression, multispace0).parse(input)
}
