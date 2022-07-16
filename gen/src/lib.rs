pub mod utils;
pub mod generate;

#[cfg(test)]
mod tests {
    use parser::combinators::schema;
    use std::fs::File;
    use std::io::Read;

    /*
    #[test]
    fn test_generate_api_scheme() {
        let mut fs = File::open("../data/schema/api.tl").unwrap();
        let mut schema_text = String::new();
        fs.read_to_string(&mut schema_text).unwrap();

        let schema = schema(&schema_text).unwrap().1;

        let _ = gen(schema);
    }

    #[test]
    fn test_generate_mtproto_scheme() {
        let mut fs = File::open("../data/schema/mtproto.tl").unwrap();
        let mut schema_text = String::new();
        fs.read_to_string(&mut schema_text).unwrap();

        let schema = schema(&schema_text).unwrap().1;

        let _ = gen(schema);
    }
    */
}
