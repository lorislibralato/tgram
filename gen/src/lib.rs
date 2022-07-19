pub mod enums;
pub mod formatter;
pub mod generate;
pub mod metadata;
pub mod structs;

#[cfg(test)]
mod tests {
    use crate::generate::generate_code;
    use parser::{combinators::schema, types::TLSchema};
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

    #[test]
    fn test_generate_all_scheme() {
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

        

        let mut output = std::io::sink();
        generate_code(&mut output, schema, 142).unwrap();
    }
}
