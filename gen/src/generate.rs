use crate::{enums::write_enums, metadata::Metadata, structs::write_structs};
use parser::types::TLSchema;
use std::io::{self, Write};

pub fn generate_code<W: Write>(f: &mut W, schema: TLSchema, layer: u32) -> io::Result<()> {
    let meta = Metadata::new(&schema);
    writeln!(f, "pub const LAYER: u32 = {};", layer)?;
    
    writeln!(f, "#[allow(clippy::large_enum_variant, non_snake_case)]")?;
    write_structs(f, &meta)?;
    writeln!(f, "#[allow(clippy::large_enum_variant, non_snake_case)]")?;
    write_enums(f, &meta)?;

    writeln!(f, "#[allow(clippy::large_enum_variant, non_snake_case)]")?;
    writeln!(f, "pub mod functions {{")?;
    writeln!(f, "}}")?;

    writeln!(f)?;
    f.flush()
}
