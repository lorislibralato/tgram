use crate::types::{
    Arg, ArgBrack, ArgCond, ArgPar, ArgSingle, BuiltinDecl, CombinatorDecl, ConditionalDef,
    Declaration, OptArg, ResType, ResTypeAng, ResTypeNormal, Section, TLSchema, Term, TermAng,
};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, anychar, char, digit1, multispace0},
    combinator::{eof, map_res, opt, recognize, value, verify},
    error::ParseError,
    multi::{count, many0, many1, many1_count},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

fn ws<F, I, O, E>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    E: ParseError<I>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, inner, multispace0)
}

fn comments_inner(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("//"), is_not("\n\r")),
        delimited(tag("/*"), is_not("*/"), tag("*/")),
    ))(input)
}

fn comments(input: &str) -> IResult<&str, ()> {
    value((), comments_inner)(input)
}

fn var_ident(input: &str) -> IResult<&str, &str> {
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

fn uc_ident_ns(input: &str) -> IResult<&str, (Option<&str>, &str)> {
    pair(opt(terminated(lc_ident, char('.'))), uc_ident)(input)
}

fn ident_ns(input: &str) -> IResult<&str, (Option<&str>, &str)> {
    pair(opt(terminated(ws(lc_ident), ws(char('.')))), ws(var_ident))(input)
}

fn lc_ident_full(input: &str) -> IResult<&str, ((Option<&str>, &str), Option<u32>)> {
    let from_str_radix_16 = |h| u32::from_str_radix(h, 16);

    pair(
        ws(lc_ident_ns),
        opt(preceded(
            ws(char('#')),
            map_res(
                ws(recognize(count(
                    verify(anychar, char::is_ascii_hexdigit),
                    8,
                ))),
                from_str_radix_16,
            ),
        )),
    )(input)
}

fn constr_sep(input: &str) -> IResult<&str, Section> {
    value(
        Section::Types,
        delimited(ws(tag("---")), ws(tag("types")), ws(tag("---"))),
    )(input)
}

fn func_sep(input: &str) -> IResult<&str, Section> {
    value(
        Section::Function,
        delimited(ws(tag("---")), ws(tag("functions")), ws(tag("---"))),
    )(input)
}

fn type_term(input: &str) -> IResult<&str, Term> {
    alt((
        // ident_ns <term>
        pair(
            ws(ident_ns),
            delimited(
                ws(char('<')),
                pair(ws(term), many0(preceded(ws(char(',')), ws(term)))),
                ws(char('>')),
            ),
        )
        .map(|(ident, (term, terms))| {
            TermAng {
                identns: ident.into(),
                term: Box::new(term),
                terms,
            }
            .into()
        }),
        // ident_ns
        ws(ident_ns).map(|ident| Term::IdentNs(ident.into())),
        // ( expr )
        delimited(ws(char('(')), many1(ws(term)), ws(char('('))).map(|terms| Term::Par(terms)),
        // nat
        ws(char('#')).map(|_| Term::Nat),
        // %
        pair(ws(char('%')), ws(term)).map(|(_, term)| Term::Percent(Box::new(term))),
    ))(input)
}

fn nat_term(input: &str) -> IResult<&str, Term> {
    // digit
    map_res(ws(digit1), |d: &str| d.parse::<u32>())
        .map(|d| Term::NatConst(d))
        .parse(input)
}

fn term(input: &str) -> IResult<&str, Term> {
    alt((ws(type_term), ws(nat_term)))(input)
}

fn conditional_def(input: &str) -> IResult<&str, (&str, Option<u32>)> {
    terminated(
        pair(
            ws(var_ident),
            opt(preceded(
                ws(char('.')),
                map_res(ws(digit1), |r: &str| r.parse::<u32>()),
            )),
        ),
        ws(char('?')),
    )(input)
}

fn arg(input: &str) -> IResult<&str, Arg> {
    alt((
        // par
        delimited(
            ws(char('(')),
            separated_pair(
                pair(ws(var_ident), many0(ws(var_ident))),
                ws(char(':')),
                pair(opt(ws(char('!'))), term),
            ),
            ws(char('(')),
        )
        .map(|((ident, idents), (excl, term))| {
            ArgPar {
                ident,
                idents,
                term,
                excl: excl.is_some(),
            }
            .into()
        }),
        // brackets
        tuple((
            opt(terminated(ws(var_ident), ws(char(':')))),
            opt(terminated(
                map_res(ws(digit1), |d: &str| d.parse::<i32>()),
                ws(char('*')),
            )),
            delimited(ws(char('[')), many1(ws(arg)), ws(char(']'))),
        ))
        .map(|(ident, mult, args)| ArgBrack { ident, mult, args }.into()),
        // conditional
        separated_pair(
            ws(var_ident),
            ws(char(':')),
            tuple((opt(ws(conditional_def)), opt(ws(char('!'))), ws(type_term))),
        )
        .map(|(ident, (cond, excl, term))| {
            ArgCond {
                ident,
                term,
                cond: cond.map(ConditionalDef::from),
                excl: excl.is_some(),
            }
            .into()
        }),
        // single
        pair(opt(ws(char('!'))), ws(term)).map(|(excl, term)| {
            ArgSingle {
                term,
                excl: excl.is_some(),
            }
            .into()
        }),
    ))(input)
}

fn opt_arg(input: &str) -> IResult<&str, OptArg> {
    delimited(
        ws(char('{')),
        separated_pair(
            pair(ws(var_ident), many0(ws(var_ident))),
            ws(char(':')),
            pair(opt(ws(char('!'))), many1(ws(term))),
        ),
        ws(char('}')),
    )
    .map(|((ident, idents), (excl, terms))| OptArg {
        ident,
        idents,
        excl: excl.is_some(),
        terms,
    })
    .parse(input)
}

fn result_type(input: &str) -> IResult<&str, ResType> {
    alt((
        pair(
            ws(uc_ident_ns),
            delimited(
                ws(char('<')),
                pair(ws(term), many0(preceded(ws(char(',')), ws(term)))),
                ws(char('>')),
            ),
        )
        .map(|(ident, (term, terms))| {
            ResTypeAng {
                identns: ident.into(),
                term,
                terms,
            }
            .into()
        }),
        pair(ws(uc_ident_ns), many0(ws(term))).map(|(ident, terms)| {
            ResTypeNormal {
                identns: ident.into(),
                terms,
            }
            .into()
        }),
    ))(input)
}

fn combinator_declaration(input: &str) -> IResult<&str, CombinatorDecl> {
    separated_pair(
        tuple((ws(lc_ident_full), many0(ws(opt_arg)), many0(ws(arg)))),
        ws(char('=')),
        terminated(ws(result_type), ws(char(';'))),
    )
    .map(|(((ident, id), opt_args, args), res)| CombinatorDecl {
        identns: ident.into(),
        id,
        opt_args,
        args,
        res,
    })
    .parse(input)
}

fn builtin_declaration(input: &str) -> IResult<&str, BuiltinDecl> {
    separated_pair(
        ws(lc_ident_full),
        pair(ws(char('?')), ws(char('='))),
        terminated(ws(uc_ident_ns), ws(char(';'))),
    )
    .map(|((name_ident, id), res_ident)| BuiltinDecl {
        identns: name_ident.into(),
        id,
        res: res_ident.into(),
        ..Default::default()
    })
    .parse(input)
}

fn tl_program(input: &str) -> IResult<&str, Vec<Declaration>> {
    terminated(
        pair(
            many1(delimited(
                many0(ws(comments)),
                alt((
                    ws(combinator_declaration).map(|d| Declaration::Constr(d)),
                    ws(builtin_declaration).map(|d| Declaration::Builtin(d)),
                )),
                many0(ws(comments)),
            )),
            many1(alt((
                preceded(
                    ws(func_sep),
                    many1(delimited(
                        many0(ws(comments)),
                        alt((
                            ws(combinator_declaration).map(|d| Declaration::Fun(d)),
                            ws(builtin_declaration).map(|d| Declaration::Builtin(d)),
                        )),
                        many0(ws(comments)),
                    )),
                ),
                preceded(
                    ws(constr_sep),
                    many1(delimited(
                        many0(ws(comments)),
                        alt((
                            ws(combinator_declaration).map(|d| Declaration::Constr(d)),
                            ws(builtin_declaration).map(|d| Declaration::Builtin(d)),
                        )),
                        many0(ws(comments)),
                    )),
                ),
            ))),
        ),
        ws(eof),
    )
    .map(|(mut constr, mixs): (Vec<_>, Vec<Vec<_>>)| {
        let len = mixs.iter().fold(0, |len, d| len + d.len());
        constr.reserve(len);

        mixs.into_iter()
            .fold(constr, |mut decls: Vec<Declaration>, dec| {
                decls.extend(dec);
                decls
            })
    })
    .parse(input)
}

pub fn schema(input: &str) -> IResult<&str, TLSchema> {
    let (input, definitions) = tl_program(input)?;

    let mut schema = TLSchema {
        funcs: Vec::with_capacity(definitions.len() / 2),
        constrs: Vec::with_capacity(definitions.len() / 2),
        ..Default::default()
    };

    let mut iter = definitions.into_iter();

    while let Some(dec) = iter.next() {
        match dec {
            Declaration::Constr(d) => schema.constrs.push(d),
            Declaration::Fun(d) => schema.funcs.push(d),
            Declaration::Builtin(d) => schema.builtin.push(d),
        }
    }

    Ok((input, schema))
}

#[cfg(test)]
mod tests {
    use super::{
        builtin_declaration, combinator_declaration, comments_inner, many0, var_ident, ws,
    };
    use crate::types::{
        Arg, ArgBrack, ArgCond, ArgPar, ArgSingle, BuiltinDecl, CombinatorDecl, IdentNs, OptArg,
        ResType, ResTypeAng, ResTypeNormal, Term,
    };

    #[test]
    fn test_comments() {
        assert_eq!(
            comments_inner("//this is a comment\n"),
            Ok(("\n", "this is a comment"))
        );
        assert_eq!(
            comments_inner("//this is a comment"),
            Ok(("", "this is a comment"))
        );
        assert_eq!(
            comments_inner("/*this is a comment*/"),
            Ok(("", "this is a comment"))
        );
        assert_eq!(
            comments_inner("//input\n//test\n//test"),
            Ok(("\n//test\n//test", "input"))
        );
        assert_eq!(
            many0(ws(comments_inner))("//input\n//test\n//test"),
            Ok(("", vec!["input", "test", "test"]))
        );
    }

    #[test]
    fn test_builtin() {
        assert_eq!(
            builtin_declaration("int ? = Int;"),
            Ok((
                "",
                BuiltinDecl {
                    identns: IdentNs {
                        namespace: None,
                        name: "int"
                    },
                    id: None,
                    res: IdentNs {
                        namespace: None,
                        name: "Int"
                    }
                }
            ))
        )
    }

    #[test]
    fn test_bytes() {
        assert_eq!(
            combinator_declaration("bytes = Bytes;"),
            Ok((
                "",
                CombinatorDecl {
                    identns: IdentNs {
                        name: "bytes",
                        ..Default::default()
                    },
                    res: ResTypeNormal {
                        identns: IdentNs {
                            name: "Bytes",
                            ..Default::default()
                        },
                        terms: vec![],
                    }
                    .into(),
                    id: None,
                    opt_args: vec![],
                    args: vec![]
                }
            ))
        )
    }

    #[test]
    fn test_vector() {
        assert_eq!(
            combinator_declaration("vector#1cb5c415 {t:Type} # [ t ] = Vector t;"),
            Ok((
                "",
                CombinatorDecl {
                    identns: IdentNs {
                        name: "vector",
                        ..Default::default()
                    },
                    id: Some(481674261),
                    opt_args: vec![OptArg {
                        ident: "t",
                        terms: vec![Term::IdentNs(IdentNs {
                            name: "Type",
                            ..Default::default()
                        })],
                        ..Default::default()
                    }],
                    args: vec![
                        ArgSingle {
                            excl: false,
                            term: Term::Nat
                        }
                        .into(),
                        ArgBrack {
                            ident: None,
                            mult: None,
                            args: vec![ArgSingle {
                                excl: false,
                                term: Term::IdentNs(IdentNs {
                                    name: "t",
                                    ..Default::default()
                                })
                            }
                            .into()]
                        }
                        .into()
                    ],
                    res: ResTypeNormal {
                        identns: IdentNs {
                            name: "Vector",
                            ..Default::default()
                        },
                        terms: vec![Term::IdentNs(IdentNs {
                            name: "t",
                            ..Default::default()
                        })],
                    }
                    .into()
                }
            ))
        )
    }

    #[test]
    fn test_var_ident() {
        assert_eq!(var_ident("int ? = "), Ok((" ? = ", "int")))
    }
}
