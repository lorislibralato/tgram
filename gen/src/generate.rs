use std::io::{self, Write};

use parser::types::TLSchema;

pub fn generate_code<W: Write>(f: &mut W, _schema: TLSchema, layer: u32) -> io::Result<()> {
    write!(f, "pub const LAYER: u32 = {};", layer)?;

    Ok(())
}

pub fn write_definition<W: Write>(_f: &mut W) -> io::Result<()> {
    Ok(())
}
