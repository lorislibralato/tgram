use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TLSchema<'a> {
    pub funcs: Vec<CombinatorDecl<'a>>,
    pub constrs: Vec<CombinatorDecl<'a>>,
    pub builtin: Vec<BuiltinDecl<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Declaration<'a> {
    Fun(CombinatorDecl<'a>),
    Constr(CombinatorDecl<'a>),
    Builtin(BuiltinDecl<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CombinatorDecl<'a> {
    pub identns: IdentNs<'a>,
    pub id: Option<u32>,
    pub opt_args: Vec<OptArg<'a>>,
    pub args: Vec<Arg<'a>>,
    pub res: ResType<'a>,
}

impl<'a> Hash for CombinatorDecl<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BuiltinDecl<'a> {
    pub identns: IdentNs<'a>,
    pub id: Option<u32>,
    pub res: IdentNs<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct OptArg<'a> {
    pub ident: &'a str,
    pub idents: Vec<&'a str>,
    pub excl: bool,
    pub terms: Vec<Term<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Arg<'a> {
    Par(ArgPar<'a>),
    Brack(ArgBrack<'a>),
    Cond(ArgCond<'a>),
    Single(ArgSingle<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ArgSingle<'a> {
    pub excl: bool,
    pub term: Term<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ArgPar<'a> {
    pub ident: &'a str,
    pub idents: Vec<&'a str>,
    pub excl: bool,
    pub term: Term<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ArgBrack<'a> {
    pub ident: Option<&'a str>,
    pub mult: Option<i32>,
    pub args: Vec<Arg<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ArgCond<'a> {
    pub ident: &'a str,
    pub cond: Option<ConditionalDef<'a>>,
    pub excl: bool,
    pub term: Term<'a>,
}

impl<'a> From<ArgBrack<'a>> for Arg<'a> {
    fn from(a: ArgBrack<'a>) -> Self {
        Self::Brack(a)
    }
}

impl<'a> From<ArgCond<'a>> for Arg<'a> {
    fn from(a: ArgCond<'a>) -> Self {
        Self::Cond(a)
    }
}

impl<'a> From<ArgPar<'a>> for Arg<'a> {
    fn from(a: ArgPar<'a>) -> Self {
        Self::Par(a)
    }
}

impl<'a> From<ArgSingle<'a>> for Arg<'a> {
    fn from(a: ArgSingle<'a>) -> Self {
        Self::Single(a)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ConditionalDef<'a> {
    pub ident: &'a str,
    pub index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResType<'a> {
    Normal(ResTypeNormal<'a>),
    Ang(ResTypeAng<'a>),
}

impl<'a> From<ResTypeAng<'a>> for ResType<'a> {
    fn from(r: ResTypeAng<'a>) -> Self {
        Self::Ang(r)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ResTypeNormal<'a> {
    pub identns: IdentNs<'a>,
    pub terms: Vec<Term<'a>>,
}

impl<'a> From<ResTypeNormal<'a>> for ResType<'a> {
    fn from(r: ResTypeNormal<'a>) -> Self {
        Self::Normal(r)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ResTypeAng<'a> {
    pub identns: IdentNs<'a>,
    pub term: Term<'a>,
    pub terms: Vec<Term<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct IdentNs<'a> {
    pub namespace: Option<&'a str>,
    pub name: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Term<'a> {
    Par(Vec<Term<'a>>),
    IdentNs(IdentNs<'a>),
    #[default]
    Nat,
    NatConst(u32),
    Percent(Box<Term<'a>>),
    Ang(TermAng<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TermAng<'a> {
    pub identns: IdentNs<'a>,
    pub term: Box<Term<'a>>,
    pub terms: Vec<Term<'a>>,
}

impl<'a> From<TermAng<'a>> for Term<'a> {
    fn from(t: TermAng<'a>) -> Self {
        Self::Ang(t)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Section {
    #[default]
    Types,
    Function,
}

impl<'a> From<(&'a str, Option<u32>)> for ConditionalDef<'a> {
    fn from(r: (&'a str, Option<u32>)) -> Self {
        Self {
            ident: r.0,
            index: r.1,
        }
    }
}

impl<'a> From<(Option<&'a str>, &'a str)> for IdentNs<'a> {
    fn from(r: (Option<&'a str>, &'a str)) -> Self {
        Self {
            namespace: r.0,
            name: r.1,
        }
    }
}

impl<'a> CombinatorDecl<'a> {
    pub fn get_id(&self) -> u32 {
        match self.id {
            Some(id) => id,
            None => self.gen_id(),
        }
    }

    pub fn gen_id(&self) -> u32 {
        0
    }
}

impl<'a> IdentNs<'a> {
    pub fn is_boxed(&self) -> bool {
        self.name.chars().next().unwrap().is_ascii_uppercase()
    }
}

impl<'a> TLSchema<'a> {
    pub fn extend(&mut self, other: Self) {
        self.builtin.extend(other.builtin);
        self.funcs.extend(other.funcs);
        self.constrs.extend(other.constrs);
    }

    pub fn calculate_ids(&mut self) {
        for decl in &mut self.constrs {
            match decl.id {
                Some(_) => (),
                None => decl.id = Some(decl.gen_id()),
            }
        }

        for decl in &mut self.funcs {
            match decl.id {
                Some(_) => (),
                None => decl.id = Some(decl.gen_id()),
            }
        }
    }
}
