use std::collections::HashMap;

use crate::type_info::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedField {
    pub doc: Option<String>,
    pub n: String,
    pub t: Type,
}

impl NamedField {
    pub fn new(n: impl Into<String>, t: Type) -> Self {
        Self {
            doc: None,
            n: n.into(),
            t,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnnamedField {
    pub doc: Option<String>,
    pub t: Type,
}

impl UnnamedField {
    pub fn new(t: Type) -> Self {
        Self { doc: None, t }
    }
}

#[derive(Clone, Debug, Eq)]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
}

impl Fields {
    pub fn named(fields: impl Into<Vec<NamedField>>) -> Self {
        Self::Named(fields.into())
    }

    pub fn unnamed(fields: impl Into<Vec<UnnamedField>>) -> Self {
        Self::Unnamed(fields.into())
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        match self {
            Self::Unnamed(unnamed) if value.is_array() => value
                .as_array()
                .map(|array| {
                    array.len() >= unnamed.len()
                        && unnamed
                            .iter()
                            .enumerate()
                            .all(|(i, f)| array.get(i).map(|v| f.t.is_compat(v)).unwrap_or(false))
                })
                .unwrap_or(false),
            Self::Named(named) if value.is_object() => value
                .as_object()
                .map(|object| {
                    object.len() >= named.len()
                        && named
                            .iter()
                            .all(|f| object.get(&f.n).map(|v| f.t.is_compat(v)).unwrap_or(false))
                })
                .unwrap_or(false),
            _ => false,
        }
    }
}

impl PartialEq for Fields {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Named(a), Self::Named(b)) if a.len() == b.len() => {
                let a = a.iter().map(|f| (&f.n, &f.t)).collect::<HashMap<_, _>>();
                let b = b.iter().map(|f| (&f.n, &f.t)).collect::<HashMap<_, _>>();
                a == b
            }
            (Self::Unnamed(a), Self::Unnamed(b)) => a == b,
            _ => false,
        }
    }
}

impl From<Vec<NamedField>> for Fields {
    fn from(fields: Vec<NamedField>) -> Self {
        Self::Named(fields)
    }
}

impl From<Vec<UnnamedField>> for Fields {
    fn from(fields: Vec<UnnamedField>) -> Self {
        Self::Unnamed(fields)
    }
}

impl From<Vec<(&'static str, Type)>> for Fields {
    fn from(fields: Vec<(&'static str, Type)>) -> Self {
        Self::Named(
            fields
                .into_iter()
                .map(|(n, t)| NamedField::new(n, t))
                .collect(),
        )
    }
}

impl From<Vec<Type>> for Fields {
    fn from(fields: Vec<Type>) -> Self {
        Self::Unnamed(fields.into_iter().map(UnnamedField::new).collect())
    }
}

impl<const N: usize> From<[NamedField; N]> for Fields {
    fn from(fields: [NamedField; N]) -> Self {
        Self::Named(fields.into())
    }
}

impl<const N: usize> From<[UnnamedField; N]> for Fields {
    fn from(fields: [UnnamedField; N]) -> Self {
        Self::Unnamed(fields.into())
    }
}
impl<const N: usize> From<[Type; N]> for Fields {
    fn from(fields: [Type; N]) -> Self {
        Self::Unnamed(fields.into_iter().map(UnnamedField::new).collect())
    }
}

impl<const N: usize> From<[(&'static str, Type); N]> for Fields {
    fn from(fields: [(&'static str, Type); N]) -> Self {
        Self::Named(
            fields
                .into_iter()
                .map(|(n, t)| NamedField::new(n, t))
                .collect(),
        )
    }
}
