pub mod enums;
pub mod formatter;
pub mod generate;
pub mod metadata;
pub mod structs;

#[cfg(test)]
mod tests {
    use crate::generate::generate_code;
    use parser::combinators::schema;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_generate_api_scheme() {
        let mut fs = File::open("../data/schema/api.tl").unwrap();
        let mut schema_text = String::new();
        fs.read_to_string(&mut schema_text).unwrap();

        let mut schema = schema(&schema_text).unwrap().1;
        schema.calculate_ids();

        let mut output = std::io::sink();

        generate_code(&mut output, schema, 142).unwrap();
    }

    #[test]
    fn test_generate_mtproto_scheme() {
        let mut fs = File::open("../data/schema/mtproto.tl").unwrap();
        let mut schema_text = String::new();
        fs.read_to_string(&mut schema_text).unwrap();

        let mut schema = schema(&schema_text).unwrap().1;
        schema.calculate_ids();

        let mut output = std::io::sink();
        generate_code(&mut output, schema, 142).unwrap();
    }
}
