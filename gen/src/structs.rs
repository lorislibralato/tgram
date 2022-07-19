use crate::{
    formatter::{builtin_type, escape_builtin_kw, Rustifiable},
    metadata::Metadata,
};
use parser::types::{Arg, CombinatorDecl, IdentNs, OptArg, Term};
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
        Term::Nat => "u32".to_string(),
        Term::Ang(ang) => {
            let ty = type_from_identns(&ang.identns);
            let ang_ty = type_from_term(&ang.term);
            format!("{}<{}>", ty, ang_ty)
        },
        Term::Percent(p) => {
            type_from_term(p)
        }
        _ => unimplemented!(),
    }
}

pub fn get_struct_field_type(arg: &Arg, generics: &[String]) -> String {
    match arg {
        Arg::Single(a) => {
            if let Term::IdentNs(ins) = &a.term {
                if generics.iter().any(|g| *g == ins.name) {
                    return ins.name.to_string();
                }
            }

            type_from_term(&a.term)
        }
        Arg::Cond(a) => {
            let ty = type_from_term(&a.term);
            if let Some(_) = &a.cond {
                format!("Option<{}>", ty)
            } else {
                ty
            }
        }
        Arg::Brack(b) => {
            match &b.mult {
                Some(n) => {
                    let args_type = b
                        .args
                        .iter()
                        .map(|arg| get_struct_field_type(arg, generics))
                        .collect::<Vec<String>>()
                        .join(",");
                    
                    if args_type != "i32" {
                        unimplemented!()
                    }

                    format!("[u8; {}]", n*4)
                },
                None => {
                    let args_type = b
                        .args
                        .iter()
                        .map(|arg| get_struct_field_type(arg, generics))
                        .collect::<Vec<String>>();

                    if args_type.len() > 1 {
                        format!("Vec<({})>", args_type.join(","))
                    } else {
                        format!("Vec<{}>", args_type[0])
                    }
                    
                },
            }            
        },
        _ => unimplemented!(),
    }
}

fn get_struct_field_name(arg: &Arg) -> String {
    match arg {
        Arg::Single(a) => match &a.term {
            Term::Nat => "nat".to_string(),
            Term::Ang(_) => "inner_list".to_string(),
            _ => unimplemented!(),
        },
        Arg::Cond(a) => escape_builtin_kw(a.ident),
        Arg::Brack(b) => match b.ident {
            Some(ident) => escape_builtin_kw(ident),
            None => "inner_list".to_string(),
        },
        _ => unimplemented!(),
    }
}

pub fn get_generic_arg(opt_args: &[OptArg]) -> Vec<String> {
    let mut args = vec![];

    for arg in opt_args {
        if arg.terms.len() != 1 {
            unimplemented!()
        }

        if let Term::IdentNs(ins) = &arg.terms[0] {
            if ins.name != "Type" {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }

        args.push(arg.ident.to_ascii_uppercase());
    }

    args
}

fn get_conditional_args(args: &[Arg]) -> Vec<String> {
    let mut list = vec![];
    for arg in args {
        match arg {
            Arg::Cond(c) if c.cond.is_some() => match &c.cond {
                Some(s) => list.push(s.ident.to_ascii_uppercase()),
                None => (),
            },
            _ => (),
        }
    }

    list
}

pub fn get_generic_name(generics: &[String]) -> String {
    if generics.len() > 0 {
        format!("<{}>", generics.join(","))
    } else {
        "".to_string()
    }
}

fn write_struct<W: Write>(f: &mut W, decl: &CombinatorDecl, indent: &str) -> io::Result<()> {
    let struct_name = decl.identns.rust_name();
    let generics = get_generic_arg(&decl.opt_args);
    let generic_name = get_generic_name(&generics);
    let conditionals = get_conditional_args(&decl.args);

    writeln!(f, "{indent}pub struct {struct_name}{generic_name} {{")?;

    for arg in &decl.args {
        let name = get_struct_field_name(arg);

        if !conditionals.iter().any(|c| *c == name) {
            let ty = get_struct_field_type(arg, &generics);
            writeln!(f, "{indent}    pub {name}: {ty},")?;
        }
    }
    writeln!(f, "{indent}}}")?;

    let id = decl.get_id();
    writeln!(f, "{indent}impl{generic_name} crate::Identificable for {struct_name}{generic_name} {{ const ID: u32 = {id}; }}")?;
    Ok(())
}

pub fn get_generic_impl(generics: &[String], trait_name: &str) -> String {
    if generics.len() > 0 {
        format!(
            "<{}>",
            generics
                .iter()
                .map(|arg: &String| format!("{arg}: {trait_name}"))
                .collect::<Vec<String>>()
                .join(",")
        )
    } else {
        "".to_string()
    }
}

fn write_struct_ser<W: Write>(f: &mut W, decl: &CombinatorDecl, indent: &str) -> io::Result<()> {
    let struct_name = decl.identns.rust_name();
    let generics = get_generic_arg(&decl.opt_args);
    let generic_name = get_generic_name(&generics);
    let generic_impl = get_generic_impl(&generics, "crate::Serializable");
    let conditionals = get_conditional_args(&decl.args);

    writeln!(
        f,
        "{indent}impl{generic_impl} crate::Serializable for {struct_name}{generic_name} {{"
    )?;
    writeln!(
        f,
        "{indent}    fn serialize(&self, buf: crate::serialize::Buf) {{"
    )?;
    for arg in &decl.args {
        let name = get_struct_field_name(arg);

        if !conditionals.iter().any(|c| *c == name) {
            match arg {
                Arg::Cond(c) if c.cond.is_some() => {
                    writeln!(f, "{indent}        if let Some(ref x) = self.{name} {{")?;
                    writeln!(f, "{indent}            x.serialize(buf);")?;
                    writeln!(f, "{indent}        }}")?;
                },
                _ => writeln!(f, "{indent}        self.{name}.serialize(buf);")?,
            }
            
        } else {
            // is flag
            writeln!(f, "{indent}        0u32.serialize(buf);")?;
        }
    }

    writeln!(f, "{indent}    }}")?;
    writeln!(f, "{indent}}}")?;

    Ok(())
}

fn write_struct_des<W: Write>(f: &mut W, decl: &CombinatorDecl, indent: &str) -> io::Result<()> {
    let struct_name = decl.identns.rust_name();
    let generics = get_generic_arg(&decl.opt_args);
    let generic_name = get_generic_name(&generics);
    let generic_impl = get_generic_impl(&generics, "crate::Deserializable");
    let conditionals = get_conditional_args(&decl.args);

    writeln!(
        f,
        "{indent}impl{generic_impl} crate::Deserializable for {struct_name}{generic_name} {{"
    )?;
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
            write_struct(f, &decl, indent)?;
            write_struct_ser(f, &decl, indent)?;
            //write_struct_des(f, &decl, indent)?;
        }

        if ns.is_some() {
            writeln!(f, "    }}")?;
        }
    }

    writeln!(f, "}}")?;

    Ok(())
}
