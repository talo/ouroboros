use crate::{field::Fields, type_info::Type};

#[derive(Clone)]
pub struct Enum {
    pub n: String,
    pub variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(n: impl Into<String>, variants: impl Into<Vec<EnumVariant>>) -> Self {
        Self {
            n: n.into(),
            variants: variants.into(),
        }
    }
}

#[derive(Clone)]
pub struct EnumVariant {
    pub n: String,
    pub v: u8,
}

impl EnumVariant {
    pub fn new(n: impl Into<String>, v: u8) -> Self {
        Self { n: n.into(), v }
    }
}

#[derive(Clone)]
pub struct Optional {
    pub t: Box<Type>,
}

impl Optional {
    pub fn new(t: impl Into<Type>) -> Self {
        Self {
            t: Box::new(t.into()),
        }
    }
}

#[derive(Clone)]
pub struct Union {
    pub n: String,
    pub variants: Vec<UnionVariant>,
}

impl Union {
    pub fn new(n: impl Into<String>, variants: impl Into<Vec<UnionVariant>>) -> Self {
        Self {
            n: n.into(),
            variants: variants.into(),
        }
    }
}

#[derive(Clone)]
pub struct UnionVariant {
    pub n: String,
    pub fields: Option<Fields>,
}

impl UnionVariant {
    pub fn with_fields(n: impl Into<String>, fields: impl Into<Fields>) -> Self {
        Self {
            n: n.into(),
            fields: Some(fields.into()),
        }
    }

    pub fn new(n: impl Into<String>) -> Self {
        Self {
            n: n.into(),
            fields: None,
        }
    }
}
