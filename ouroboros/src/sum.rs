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

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        // Stringy compat
        (value.is_string()
            && value.as_str().map(|v| self
                .variants
                .iter()
                .any(|variant| v == variant.n)).unwrap_or(false))
            || // Const-value compat
            (value.is_u64()
                && value.as_u64().map(|v| self.variants.iter().any(|variant| 
                    variant.v.map(|u| u == v as u8).unwrap_or(false)
                )).unwrap_or(false))
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

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        value.is_null() || self.t.is_compat(value)
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

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        // Stringy compat
        (value.is_string()
            && value.as_str().map(|s| self
                .variants
                .iter()
                .any(|variant|  s == variant.n)).unwrap_or(false))
                || // Variant compat
                (
                    value.is_object() && value.as_object().map(|object| {
                        self.variants.iter().any(|variant| {
                            object.get(&variant.n)
                                .and_then(|object_fields| variant.fields.as_ref().map(|variant_fields| variant_fields.is_compat(object_fields)))
                                .unwrap_or(false)
                        })
                    }).unwrap_or(false)    
                )
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
