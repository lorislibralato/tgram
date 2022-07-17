use parser::types::{CombinatorDecl, TLSchema};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Metadata<'a> {
    pub types_ns: HashMap<Option<&'a str>, Vec<&'a CombinatorDecl<'a>>>,
    pub funcs_ns: HashMap<Option<&'a str>, Vec<&'a CombinatorDecl<'a>>>,
}

impl<'a> Metadata<'a> {
    pub fn new(schema: &'a TLSchema) -> Self {
        let mut meta = Self::default();

        for ty_decl in &schema.constrs {
            meta.types_ns
                .entry(ty_decl.identns.namespace)
                .or_insert(Vec::new())
                .push(ty_decl);
        }

        for fn_decl in &schema.funcs {
            meta.funcs_ns
                .entry(fn_decl.identns.namespace)
                .or_insert(Vec::new())
                .push(fn_decl);
        }

        meta
    }
}
