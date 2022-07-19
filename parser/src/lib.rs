pub mod basics;
pub mod combinators;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
    use super::combinators::schema;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_parse_api_schema() {
        let mut fs = File::open("../data/schema/api.tl").unwrap();
        let mut schema_text = String::new();
        fs.read_to_string(&mut schema_text).unwrap();

        let mut schema = schema(&schema_text).unwrap().1;
        schema.calculate_ids();
    }

    #[test]
    fn test_parse_mtproto_schema() {
        let mut fs = File::open("../data/schema/mtproto.tl").unwrap();
        let mut schema_text = String::new();
        fs.read_to_string(&mut schema_text).unwrap();

        let mut schema = schema(&schema_text).unwrap().1;
        schema.calculate_ids();
    }
}
