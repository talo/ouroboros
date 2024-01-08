use std::collections::HashMap;

use crate::{field::Fields, type_info::Type};

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnumVariant {
    pub n: String,
    pub v: Option<u8>,
}

impl EnumVariant {
    pub fn new(n: impl Into<String>) -> Self {
        Self {
            n: n.into(),
            v: None,
        }
    }

    pub fn with_const_value(n: impl Into<String>, v: u8) -> Self {
        Self {
            n: n.into(),
            v: Some(v),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
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

impl PartialEq for Union {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
            && match (&self.variants, &other.variants) {
                (a, b) if a.len() == b.len() => {
                    let a = a
                        .iter()
                        .map(|v| (&v.n, &v.fields))
                        .collect::<HashMap<_, _>>();
                    let b = b
                        .iter()
                        .map(|v| (&v.n, &v.fields))
                        .collect::<HashMap<_, _>>();
                    a == b
                }
                _ => false,
            }
    }
}

impl Eq for Union {}

#[derive(Clone, Debug, PartialEq, Eq)]
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
