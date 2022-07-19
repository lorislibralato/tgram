use parser::combinators::schema;
use parser::types::TLSchema;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn count_arg(schema: &TLSchema, counter: &mut HashMap<&str, i32>) {
    for decl in &schema.constrs {
        for arg in &decl.args {
            let entry_name = match arg {
                parser::types::Arg::Par(_) => "arg par",
                parser::types::Arg::Brack(_) => "arg brack",
                parser::types::Arg::Cond(_) => "arg cond",
                parser::types::Arg::Single(_) => "arg single",
            };

            *(counter.entry(entry_name).or_insert(0)) += 1
        }
    }
}

fn main() {
    let mut api_schema_text = String::new();
    File::open("data/schema/api.tl")
        .unwrap()
        .read_to_string(&mut api_schema_text)
        .unwrap();
    let api_schema = schema(&api_schema_text).unwrap().1;

    let mut mtproto_schema_text = String::new();
    File::open("data/schema/mtproto.tl")
        .unwrap()
        .read_to_string(&mut mtproto_schema_text)
        .unwrap();
    let mtproto_schema = schema(&mtproto_schema_text).unwrap().1;

    let mut counter = HashMap::new();

    count_arg(&api_schema, &mut counter);
    count_arg(&mtproto_schema, &mut counter);

    println!("{:?}", counter);
}
