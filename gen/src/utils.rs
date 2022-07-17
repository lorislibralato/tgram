use parser::types::IdentNs;

pub trait Rustifiable {
    fn struct_name(&self) -> String;
    fn path(&self) -> String;
}

pub fn escape_builtin_kw(ty: &str) -> String {
    match ty {
        "final" | "async" | "impl" | "fn" | "pub" | "mut" | "let" | "type" | "loop" | "while" | "for" | "static" => {
            let mut s = String::from("r#");
            s.push_str(ty);
            s
        },
        "self" => "_self".to_string(),
        e => e.to_string()
    }
}

pub fn builtin_type(ty: &str) -> Option<&str> {
    Some(match ty {
        "int" => "i32",
        "long" => "i64",
        "double" => "f64",
        "string" => "String",
        "bytes" => "Vec<u8>",
        _ => return None
    })
}

impl<'a> Rustifiable for IdentNs<'a> {
    fn struct_name(&self) -> String {
        enum Case {
            Upper,
            Lower,
            Preserve,
        }

        let mut res = String::with_capacity(self.name.len());

        self.name.chars().fold(Case::Upper, |case, c| {
            if c == '_' {
                return Case::Upper;
            }

            match case {
                Case::Upper => {
                    res.push(c.to_ascii_uppercase());
                    Case::Lower
                }
                Case::Lower => {
                    res.push(c.to_ascii_lowercase());
                    if c.is_ascii_uppercase() {
                        Case::Lower
                    } else {
                        Case::Preserve
                    }
                }
                Case::Preserve => {
                    res.push(c);
                    if c.is_ascii_uppercase() {
                        Case::Lower
                    } else {
                        Case::Preserve
                    }
                }
            }
        });

        res
    }

    fn path(&self) -> String {
        match self.namespace {
            Some(n) => format!("{}::{}", n, self.struct_name()),
            None => self.struct_name(),
        }
    }
}
