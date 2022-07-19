use crate::{
    formatter::{builtin_type, escape_builtin_kw, Rustifiable},
    metadata::Metadata,
};
use itertools::Itertools;
use parser::types::{Arg, CombinatorDecl, IdentNs, Term};
use std::io::{self, Write};

fn type_from_identns(ins: &IdentNs) -> String {
    if let Some(bty) = builtin_type(ins.name) {
        bty.to_string()
    } else {
        let module = if ins.is_boxed() { "enums" } else { "types" };
        format!("crate::{}::{}", module, ins.rust_path())
    }
}

fn type_from_term(term: &Term) -> String {
    match term {
        Term::IdentNs(ins) => type_from_identns(ins),
        Term::Ang(ang) => {
            let ty = type_from_identns(&ang.identns);
            let ang_ty = type_from_term(&ang.term);
            format!("{}<{}>", ty, ang_ty)
        }
        Term::Percent(p) => type_from_term(p),
        Term::Nat => "u32".to_string(),
        _ => unimplemented!(),
    }
}

pub fn get_struct_field_type(arg: &Arg) -> String {
    match arg {
        Arg::Cond(a) => {
            let ty = type_from_term(&a.term);
            if a.cond.is_some() {
                format!("Option<{}>", ty)
            } else {
                ty
            }
        }
        _ => unimplemented!(),
    }
}

fn get_struct_field_name(arg: &Arg) -> String {
    match arg {
        Arg::Cond(a) => escape_builtin_kw(a.ident),
        _ => unimplemented!(),
    }
}

fn get_conditional_args<'a>(args: &[Arg<'a>]) -> Vec<&'a str> {
    let mut list = vec![];
    for arg in args {
        match arg {
            Arg::Cond(c) if c.cond.is_some() => match &c.cond {
                Some(s) => list.push(s.ident),
                None => (),
            },
            _ => (),
        }
    }

    list
}

fn write_struct<W: Write>(f: &mut W, decl: &CombinatorDecl, indent: &str) -> io::Result<()> {
    let struct_name = decl.identns.rust_name();
    let conditionals = get_conditional_args(&decl.args);

    writeln!(f, "{indent}pub struct {struct_name} {{")?;

    for arg in &decl.args {
        let name = get_struct_field_name(arg);

        if !conditionals.iter().any(|c| *c == name) {
            let ty = get_struct_field_type(arg);
            writeln!(f, "{indent}    pub {name}: {ty},")?;
        }
    }
    writeln!(f, "{indent}}}")?;

    let id = decl.get_id();
    writeln!(
        f,
        "{indent}impl crate::Identificable for {struct_name} {{ const ID: u32 = {id}; }}"
    )?;
    Ok(())
}

fn write_struct_ser<W: Write>(f: &mut W, decl: &CombinatorDecl, indent: &str) -> io::Result<()> {
    let struct_name = decl.identns.rust_name();
    let conditionals = get_conditional_args(&decl.args);

    writeln!(f, "{indent}impl crate::Serializable for {struct_name} {{")?;
    writeln!(
        f,
        "{indent}    fn serialize(&self, {}buf: crate::serialize::Buf) {{",
        if decl.args.is_empty() { "_" } else { "" }
    )?;
    for arg in &decl.args {
        let name = get_struct_field_name(arg);

        if !conditionals.iter().any(|c| *c == name) {
            match arg {
                Arg::Cond(c) if c.cond.is_some() => {
                    writeln!(f, "{indent}        if let Some(ref x) = self.{name} {{")?;
                    writeln!(f, "{indent}            x.serialize(buf);")?;
                    writeln!(f, "{indent}        }}")?;
                }
                _ => writeln!(f, "{indent}        self.{name}.serialize(buf);")?,
            }
        } else {
            let flag_names = decl
                .args
                .iter()
                .filter(|a| match a {
                    Arg::Cond(c) => matches!(&c.cond, Some(cond) if cond.ident == name),
                    _ => false,
                })
                .map(|a| match a {
                    Arg::Cond(c) => (escape_builtin_kw(c.ident), c.cond.as_ref().unwrap().index.unwrap_or(0)),
                    _ => panic!(),
                });
            let fill_flag = flag_names
                .map(|(attr, idx)| {
                    format!("if self.{attr}.is_some() {{ {} }} else {{ 0 }}", 1 << idx)
                })
                .join(" | ");
            // is flag
            writeln!(f, "{indent}        ({fill_flag}).serialize(buf);")?;
        }
    }

    writeln!(f, "{indent}    }}")?;
    writeln!(f, "{indent}}}")?;

    Ok(())
}

fn write_struct_des<W: Write>(f: &mut W, decl: &CombinatorDecl, indent: &str) -> io::Result<()> {
    let struct_name = decl.identns.rust_name();
    let conditionals = get_conditional_args(&decl.args);

    writeln!(f, "{indent}impl crate::Deserializable for {struct_name} {{")?;
    writeln!(
        f,
        "{indent}    fn deserialize(buf: crate::deserialize::Buf) -> crate::Result<Self> {{"
    )?;

    writeln!(f, "{indent}    }}")?;
    writeln!(f, "{indent}}}")?;
    Ok(())
}

pub fn write_structs<W: Write>(f: &mut W, metadata: &Metadata) -> io::Result<()> {
    writeln!(f, "pub mod types {{")?;

    for (ns, decls) in &metadata.types_ns {
        let indent = if let Some(ns) = ns {
            writeln!(f, "    pub mod {ns} {{")?;
            "        "
        } else {
            "    "
        };

        for decl in decls {
            match decl.identns.name {
                "int128" | "int256" | "vector" => continue,
                _ => (),
            }
            writeln!(f, "{indent}#[derive(Debug, Clone, PartialEq)]")?;
            write_struct(f, decl, indent)?;
            write_struct_ser(f, decl, indent)?;
            //write_struct_des(f, &decl, indent)?;
        }

        if ns.is_some() {
            writeln!(f, "    }}")?;
        }
    }

    writeln!(f, "}}")?;

    Ok(())
}
