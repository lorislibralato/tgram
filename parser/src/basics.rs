use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, anychar, char, multispace0},
    combinator::{map_res, opt, recognize, value, verify},
    error::ParseError,
    multi::many1_count,
    sequence::{delimited, pair, preceded, terminated},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

pub fn ws<F, I, O, E>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, inner, multispace0)
}

pub fn comments_inner(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("//"), is_not("\n\r")),
        delimited(tag("/*"), is_not("*/"), tag("*/")),
    ))(input)
}

pub fn comments(input: &str) -> IResult<&str, ()> {
    value((), comments_inner)(input)
}

pub fn var_ident(input: &str) -> IResult<&str, &str> {
    recognize(many1_count(alt((alphanumeric1, tag("_")))))(input)
}

fn lc_ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        verify(anychar, char::is_ascii_lowercase),
        opt(var_ident),
    ))(input)
}

fn uc_ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        verify(anychar, char::is_ascii_uppercase),
        opt(var_ident),
    ))(input)
}

fn lc_ident_ns(input: &str) -> IResult<&str, (Option<&str>, &str)> {
    pair(opt(terminated(lc_ident, char('.'))), lc_ident)(input)
}

pub fn uc_ident_ns(input: &str) -> IResult<&str, (Option<&str>, &str)> {
    pair(opt(terminated(lc_ident, char('.'))), uc_ident)(input)
}

pub fn ident_ns(input: &str) -> IResult<&str, (Option<&str>, &str)> {
    pair(opt(terminated(ws(lc_ident), ws(char('.')))), ws(var_ident))(input)
}

type IdentFull<'a> = ((Option<&'a str>, &'a str), Option<u32>);

pub fn lc_ident_full(input: &str) -> IResult<&str, IdentFull> {
    let from_str_radix_16 = |h| u32::from_str_radix(h, 16);

    pair(
        ws(lc_ident_ns),
        opt(preceded(
            ws(char('#')),
            map_res(
                ws(recognize(many1_count(verify(
                    anychar,
                    char::is_ascii_hexdigit,
                )))),
                from_str_radix_16,
            ),
        )),
    )(input)
}
