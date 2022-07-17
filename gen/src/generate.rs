use crate::{metadata::Metadata, utils::{Rustifiable, builtin_type, escape_builtin_kw}};
use parser::types::{Arg, ArgBrack, ArgCond, ArgSingle, TLSchema, Term, TermAng};
use std::io::{self, Write};

pub fn generate_code<W: Write>(f: &mut W, schema: TLSchema, layer: u32) -> io::Result<()> {
    writeln!(f, "pub const LAYER: u32 = {};", layer)?;

    let meta = Metadata::new(&schema);

    write_schema(f, &meta)?;

    writeln!(f)?;
    f.flush()
}

pub fn write_schema<W: Write>(f: &mut W, metadata: &Metadata) -> io::Result<()> {
    writeln!(f, "pub mod types {{")?;

    for (ns, decls) in &metadata.types_ns {
        if let Some(ns) = ns {
            writeln!(f, "pub mod {ns} {{")?;
        }

        for decl in decls {
            let name = decl.identns.struct_name();
            let id = decl.get_id();

            writeln!(f, "pub struct {name} {{")?;

            for arg in &decl.args {
                match arg {
                    Arg::Single(a) => {
                        match &a.term {
                            Term::Nat => writeln!(f, "pub nat: u32,")?,
                            _ => unimplemented!(),
                        }
                    },
                    Arg::Cond(a) => {
                        match &a.term {
                            Term::IdentNs(ins) if !ins.is_boxed() => {
                                let ty = if let Some(bty) = builtin_type(ins.name) {
                                    bty.to_string()
                                } else {
                                    format!("crate::types::{}", ins.path())
                                };

                                let ty = if let Some(_) = &a.cond {
                                    format!("Option<{}>", ty)
                                } else {
                                    ty
                                };

                                writeln!(f, "pub {}: {},", escape_builtin_kw(a.ident), ty)?
                            },
                            _ => continue
                        }
                    }
                    _ => continue,
                }
            }

            writeln!(f, "}}")?;

            writeln!(
                f,
                "impl crate::Identificable for {name} {{ fn id(&self) -> u32 {{ {id} }} }}",
            )?;

            writeln!(
                f,
                "impl crate::Serializable for {name} {{
                    fn serialize(&self, _buf: crate::serialize::Buf) 
                    {{
                        
                    }}
                }}",
            )?;

            writeln!(
                f,
                "impl crate::Deserializable for {name} {{
                    fn deserialize(_buf: crate::deserialize::Buf) -> crate::Result<Self>{{
                        todo!()
                    }}
                }}"
            )?;
        }

        if ns.is_some() {
            writeln!(f, "}}")?;
        }
    }

    writeln!(f, "}}")?;

    writeln!(f, "pub mod functions {{")?;

    writeln!(f, "}}")?;

    writeln!(f, "pub mod enums {{")?;

    writeln!(f, "}}")?;

    Ok(())
}
