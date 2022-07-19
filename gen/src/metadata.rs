use parser::types::{CombinatorDecl, ResType, TLSchema};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Metadata<'a> {
    pub types_ns: HashMap<Option<&'a str>, HashSet<&'a CombinatorDecl<'a>>>,
    pub funcs_ns: HashMap<Option<&'a str>, HashSet<&'a CombinatorDecl<'a>>>,

    pub types_group_ns:
        HashMap<Option<&'a str>, HashMap<ResType<'a>, HashSet<&'a CombinatorDecl<'a>>>>,
}

impl<'a> Metadata<'a> {
    pub fn new(schema: &'a TLSchema) -> Self {
        let mut meta = Self::default();

        for ty_decl in &schema.constrs {
            meta.types_ns
                .entry(ty_decl.identns.namespace)
                .or_default()
                .insert(ty_decl);
        }

        for (ns, decls) in &meta.types_ns {
            let entry = meta.types_group_ns.entry(*ns).or_default();
            for decl in decls {
                entry.entry(decl.res.clone()).or_default().insert(decl);
            }
        }

        for fn_decl in &schema.funcs {
            meta.funcs_ns
                .entry(fn_decl.identns.namespace)
                .or_default()
                .insert(fn_decl);
        }

        meta
    }
}
