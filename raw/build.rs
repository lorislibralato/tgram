use gen::generate::generate_code;
use parser::combinators::schema;
use std::{env, fs::File, io::Read, path::Path};

fn main() {
    let mut fs = File::open("../data/schema/api.tl").unwrap();

    let mut schema_text = String::new();
    fs.read_to_string(&mut schema_text).unwrap();

    let schema = schema(&schema_text).unwrap().1;

    let dest_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("generated.rs");
    let mut fs = File::options()
        .create(true)
        .write(true)
        .open(dest_path)
        .unwrap();
    generate_code(&mut fs, schema, 142).unwrap();
}
