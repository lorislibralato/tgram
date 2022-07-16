use parser::combinators::schema;
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::Path,
};

fn main() {
    let mut fs = File::open("../data/schema/api.tl").unwrap();
    let mut schema_text = String::new();
    fs.read_to_string(&mut schema_text).unwrap();

    let schema = schema(&schema_text).unwrap().1;

    /*
    let scope = gen(schema);
    let code = scope.to_string();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(dest_path, code).unwrap();
    */
}
