use crate::{formatter::Rustifiable, metadata::Metadata, structs::get_struct_field_type};
use parser::types::{Arg, CombinatorDecl, IdentNs, ResType};
use std::{
    collections::HashSet,
    io::{self, Write},
};

fn get_enum_variant_path(identns: &IdentNs) -> String {
    format!(
        "crate::{}::{}",
        if identns.is_boxed() { "enums" } else { "types" },
        identns.rust_path()
    )
}

fn is_recursive_enum_variant(enum_name: &str, args: &[Arg]) -> bool {
    args.iter()
        .map(get_struct_field_type)
        .any(|a| a == format!("crate::enums::{}", enum_name))
}

fn write_enum<W: Write>(
    f: &mut W,
    res: &ResType,
    decls: &HashSet<&CombinatorDecl>,
    indent: &str,
) -> io::Result<()> {
    let enum_name = res.rust_name();

    writeln!(f, "{indent}pub enum {enum_name} {{")?;

    for decl in decls {
        let mut ty = get_enum_variant_path(&decl.identns);

        if is_recursive_enum_variant(&enum_name, &decl.args) {
            ty = format!("Box<{}>", ty);
        }

        writeln!(f, "{indent}    {}({}),", decl.identns.rust_name(), ty)?;
    }
    writeln!(f, "{indent}}}")?;

    Ok(())
}

fn write_enum_ser<W: Write>(
    f: &mut W,
    res: &ResType,
    decls: &HashSet<&CombinatorDecl>,
    indent: &str,
) -> io::Result<()> {
    let enum_name = res.rust_name();

    writeln!(f, "{indent}impl crate::Serializable for {enum_name} {{")?;
    writeln!(
        f,
        "{indent}    fn serialize(&self, buf: crate::serialize::Buf) {{"
    )?;

    writeln!(f, "{indent}        use crate::Identificable;")?;
    writeln!(f, "{indent}        match self {{")?;
    for decl in decls {
        let variant_name = decl.identns.rust_name();
        let variant_path = get_enum_variant_path(&decl.identns);

        let anon_name = if !decl.args.is_empty() { "x" } else { "_" };

        writeln!(
            f,
            "{indent}            Self::{variant_name}({anon_name}) => {{"
        )?;
        writeln!(
            f,
            "{indent}                {variant_path}::ID.serialize(buf);"
        )?;
        if !decl.args.is_empty() {
            writeln!(f, "{indent}                {anon_name}.serialize(buf);")?;
        }
        writeln!(f, "{indent}            }}")?;
    }
    writeln!(f, "{indent}        }}")?;

    writeln!(f, "{indent}    }}")?;
    writeln!(f, "{indent}}}")?;
    Ok(())
}

fn write_enum_des<W: Write>(
    f: &mut W,
    res: &ResType,
    decls: &HashSet<&CombinatorDecl>,
    indent: &str,
) -> io::Result<()> {
    let enum_name = res.rust_name();

    writeln!(f, "{indent}impl crate::Deserializable for {enum_name} {{")?;
    writeln!(
        f,
        "{indent}    fn deserialize(buf: crate::deserialize::Buf) -> crate::Result<Self> {{"
    )?;
    writeln!(f, "{indent}    }}")?;
    writeln!(f, "{indent}}}")?;
    Ok(())
}

pub fn write_enums<W: Write>(f: &mut W, metadata: &Metadata) -> io::Result<()> {
    writeln!(f, "pub mod enums {{")?;
    for (ns, groups) in &metadata.types_group_ns {
        let indent = if let Some(ns) = ns {
            writeln!(f, "    pub mod {ns} {{")?;
            "        "
        } else {
            "    "
        };

        for (res_ty, decls) in groups {
            if let ResType::Normal(n) = res_ty {
                match n.identns.name {
                    "Int128" | "Int256" | "Vector" => continue,
                    _ => (),
                }
            }
            writeln!(f, "{indent}#[derive(Debug, Clone, PartialEq)]")?;
            write_enum(f, res_ty, decls, indent)?;
            //write_enum_des(f, decls, indent)?;
            write_enum_ser(f, res_ty, decls, indent)?;
        }

        if ns.is_some() {
            writeln!(f, "    }}")?;
        }
    }
    writeln!(f, "}}")?;

    Ok(())
}
