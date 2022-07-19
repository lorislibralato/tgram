use crate::{
    formatter::{Rustifiable},
    metadata::Metadata,
    structs::{get_generic_arg, get_generic_impl, get_generic_name, get_struct_field_type},
};
use itertools::Itertools;
use parser::types::{Arg, CombinatorDecl, IdentNs, OptArg, ResType};
use std::io::{self, Write};

fn get_enum_variant_path(identns: &IdentNs) -> String {
    format!(
        "crate::{}::{}",
        if identns.is_boxed() { "enums" } else { "types" },
        identns.rust_path()
    )
}

fn get_enum_variant_generics(args: &[OptArg]) -> String {
    let generics = get_generic_arg(args);

    if generics.len() > 0 {
        format!("<{}>", generics.join(","))
    } else {
        "".to_string()
    }
}

fn is_recursive_enum_variant(enum_name: &str, args: &[Arg]) -> bool {
    args.iter()
        .map(|arg| get_struct_field_type(arg, &vec![]))
        .any(|a| a == format!("crate::enums::{}", enum_name))
}

fn write_enum<W: Write>(
    f: &mut W,
    res: &ResType,
    decls: &[&CombinatorDecl],
    indent: &str,
) -> io::Result<()> {
    let enum_name = res.rust_name();
    let generics = get_generic_names(decls);
    let generic_name = get_generic_name(&generics);

    writeln!(f, "{indent}pub enum {enum_name}{generic_name} {{")?;

    for decl in decls {
        let mut ty = get_enum_variant_path(&decl.identns);

        ty.push_str(&get_enum_variant_generics(&decl.opt_args));

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
    decls: &[&CombinatorDecl],
    indent: &str,
) -> io::Result<()> {
    let enum_name = res.rust_name();
    let generics = get_generic_names(decls);
    let generic_name = get_generic_name(&generics);
    let generics_impl = get_generic_impl(&generics, "crate::Serializable");

    writeln!(
        f,
        "{indent}impl{generics_impl} crate::Serializable for {enum_name}{generic_name} {{"
    )?;
    writeln!(
        f,
        "{indent}    fn serialize(&self, buf: crate::serialize::Buf) {{"
    )?;

    writeln!(f, "{indent}        use crate::Identificable;")?;
    writeln!(f, "{indent}        match self {{")?;
    for decl in decls {
        let variant_name = decl.identns.rust_name();
    
        let mut variant_path = get_enum_variant_path(&decl.identns);
        let variant_generic = get_enum_variant_generics(&decl.opt_args);
        if  !variant_generic.is_empty() {
            variant_path = format!("{variant_path}::{variant_generic}")
        }
        
        

        writeln!(f, "{indent}            Self::{variant_name}(x) => {{")?;
        writeln!(f, "{indent}                {variant_path}::ID.serialize(buf);")?;
        writeln!(f, "{indent}                x.serialize(buf);")?;
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
    decls: &[&CombinatorDecl],
    indent: &str,
) -> io::Result<()> {
    let enum_name = res.rust_name();
    let generics = get_generic_names(decls);
    let generic_name = get_generic_name(&generics);
    let generics_des = get_generic_impl(&generics, "crate::Deserializable");

    writeln!(
        f,
        "{indent}impl{generics_des} crate::Deserializable for {enum_name}{generic_name} {{"
    )?;
    writeln!(
        f,
        "{indent}    fn deserialize(buf: crate::deserialize::Buf) -> crate::Result<Self> {{"
    )?;
    writeln!(f, "{indent}    }}")?;
    writeln!(f, "{indent}}}")?;
    Ok(())
}

fn get_generic_names(decls: &[&CombinatorDecl]) -> Vec<String> {
    decls
        .iter()
        .map(|decl| get_generic_arg(&decl.opt_args))
        .fold(Vec::<String>::new(), |mut vec, v| {
            vec.extend(v);
            vec
        })
        .into_iter()
        .unique()
        .collect::<Vec<String>>()
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
            write_enum(f, res_ty, decls, indent)?;
            // write_enum_des(f, decls, indent)?;
            write_enum_ser(f, res_ty, decls, indent)?;
        }

        if ns.is_some() {
            writeln!(f, "    }}")?;
        }
    }
    writeln!(f, "}}")?;

    Ok(())
}
