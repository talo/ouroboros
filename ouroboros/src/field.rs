use std::vec::IntoIter;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    slice::Iter,
};

use crate::type_info::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedFields {
    pub fields: Vec<NamedField>,
}

impl NamedFields {
    pub fn empty() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn iter(&self) -> Iter<'_, NamedField> {
        self.fields.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&NamedField> {
        self.fields.iter().find(|f| f.n == name)
    }
}

impl From<Vec<NamedField>> for NamedFields {
    fn from(fields: Vec<NamedField>) -> Self {
        Self { fields }
    }
}

impl<'a> From<Vec<(&'a str, Type)>> for NamedFields {
    fn from(fields: Vec<(&'a str, Type)>) -> Self {
        Self {
            fields: fields
                .into_iter()
                .map(|(n, t)| NamedField::new(n, t))
                .collect(),
        }
    }
}

impl<const N: usize> From<[NamedField; N]> for NamedFields {
    fn from(fields: [NamedField; N]) -> Self {
        Self {
            fields: fields.into(),
        }
    }
}

impl<'a, const N: usize> From<[(&'a str, Type); N]> for NamedFields {
    fn from(fields: [(&'a str, Type); N]) -> Self {
        Self {
            fields: fields.map(|(n, t)| NamedField::new(n, t)).into(),
        }
    }
}

impl Display for NamedFields {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "{".fmt(f)?;
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                ", ".fmt(f)?;
            }
            field.fmt(f)?;
        }
        "}".fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedField {
    pub n: String,
    pub t: Type,
}

impl NamedField {
    pub fn new(n: impl Into<String>, t: Type) -> Self {
        Self { n: n.into(), t }
    }
}

impl Display for NamedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)?;
        ": ".fmt(f)?;
        self.t.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnnamedFields {
    pub fields: Vec<UnnamedField>,
}

impl UnnamedFields {
    pub fn empty() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn iter(&self) -> Iter<'_, UnnamedField> {
        self.fields.iter()
    }
    pub fn into_iter(self) -> IntoIter<UnnamedField> {
        self.fields.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl From<Vec<UnnamedField>> for UnnamedFields {
    fn from(fields: Vec<UnnamedField>) -> Self {
        Self { fields }
    }
}

impl<'a> From<Vec<Type>> for UnnamedFields {
    fn from(fields: Vec<Type>) -> Self {
        Self {
            fields: fields.into_iter().map(|t| UnnamedField::new(t)).collect(),
        }
    }
}

impl<const N: usize> From<[UnnamedField; N]> for UnnamedFields {
    fn from(fields: [UnnamedField; N]) -> Self {
        Self {
            fields: fields.into(),
        }
    }
}

impl<'a, const N: usize> From<[Type; N]> for UnnamedFields {
    fn from(fields: [Type; N]) -> Self {
        Self {
            fields: fields.map(|t| UnnamedField::new(t)).into(),
        }
    }
}

impl Display for UnnamedFields {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "{".fmt(f)?;
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                ", ".fmt(f)?;
            }
            field.fmt(f)?;
        }
        "}".fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnnamedField {
    pub t: Type,
}

impl UnnamedField {
    pub fn new(t: Type) -> Self {
        Self { t }
    }

    pub fn name(self, n: impl Into<String>) -> NamedField {
        NamedField {
            n: n.into(),
            t: self.t,
        }
    }
}

impl Display for UnnamedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.t.fmt(f)
    }
}

#[derive(Clone, Debug, Eq)]
pub enum Fields {
    Named(NamedFields),
    Unnamed(UnnamedFields),
}

impl Fields {
    pub fn named(fields: impl Into<NamedFields>) -> Self {
        Self::Named(fields.into())
    }

    pub fn unnamed(fields: impl Into<UnnamedFields>) -> Self {
        Self::Unnamed(fields.into())
    }

    pub fn is_compat(&self, value: Option<&serde_json::Value>) -> bool {
        match value {
            Some(value) => match self {
                Self::Unnamed(unnamed) if value.is_array() => value
                    .as_array()
                    .map(|array| {
                        array.len() >= unnamed.len()
                            && unnamed.iter().enumerate().all(|(i, f)| match array.get(i) {
                                Some(v) if f.t.is_compat(Some(v)) => true,
                                Some(_) => false,
                                None => false,
                            })
                    })
                    .unwrap_or(false),
                Self::Named(named) if value.is_object() => value
                    .as_object()
                    .map(|object| {
                        named.iter().all(|f| match object.get(&f.n) {
                            Some(v) if f.t.is_compat(Some(v)) => true,
                            Some(_) => false,
                            None => f.t.is_compat(None),
                        })
                    })
                    .unwrap_or(false),
                _ => false,
            },
            None => false,
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
        Self::Named(fields.into())
    }
}

impl<'a> From<Vec<(&'a str, Type)>> for Fields {
    fn from(fields: Vec<(&'a str, Type)>) -> Self {
        Self::Named(fields.into())
    }
}

impl<const N: usize> From<[NamedField; N]> for Fields {
    fn from(fields: [NamedField; N]) -> Self {
        Self::Named(fields.into())
    }
}

impl<'a, const N: usize> From<[(&'a str, Type); N]> for Fields {
    fn from(fields: [(&'a str, Type); N]) -> Self {
        Self::Named(fields.into())
    }
}

impl From<Vec<UnnamedField>> for Fields {
    fn from(fields: Vec<UnnamedField>) -> Self {
        Self::Unnamed(fields.into())
    }
}

impl<'a> From<Vec<Type>> for Fields {
    fn from(fields: Vec<Type>) -> Self {
        Self::Unnamed(fields.into())
    }
}

impl<const N: usize> From<[UnnamedField; N]> for Fields {
    fn from(fields: [UnnamedField; N]) -> Self {
        Self::Unnamed(fields.into())
    }
}

impl<'a, const N: usize> From<[Type; N]> for Fields {
    fn from(fields: [Type; N]) -> Self {
        Self::Unnamed(fields.into())
    }
}

impl Display for Fields {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(fields) => fields.fmt(f),
            Self::Unnamed(fields) => fields.fmt(f),
        }
    }
}
