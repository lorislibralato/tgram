use gen::generate::generate_code;
use parser::{combinators::schema, types::TLSchema};
use std::{env, fs::File, io::Read, path::Path};

fn main() {
    let mut api_fs = File::open("../data/schema/api.tl").unwrap();
    let mut mtproto_fs = File::open("../data/schema/mtproto.tl").unwrap();

    let mut api_text = String::new();
    let mut mtproto_text = String::new();

    mtproto_fs.read_to_string(&mut mtproto_text).unwrap();
    api_fs.read_to_string(&mut api_text).unwrap();

    let api_schema = schema(&api_text).unwrap().1;
    let mtproto_schema = schema(&mtproto_text).unwrap().1;

    let mut schema = TLSchema::default();
    schema.extend(api_schema);
    schema.extend(mtproto_schema);
    schema.calculate_ids();

    let dest_path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("generated.rs");
    let mut fs = File::options()
        .create(true)
        .write(true)
        .open(dest_path)
        .unwrap();
    generate_code(&mut fs, schema, 142).unwrap();
}
