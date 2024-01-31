use std::fmt::{self, Display, Formatter};

use crate::{
    field::{Fields, UnnamedField},
    type_info::Type,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Array {
    pub doc: Option<String>,
    pub t: Box<Type>,
}

impl Array {
    pub fn new(t: impl Into<Type>) -> Self {
        Self {
            doc: None,
            t: Box::new(t.into()),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        value.is_array()
            && value
                .as_array()
                .map(|a| a.iter().all(|v| self.t.is_compat(v)))
                .unwrap_or(false)
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "[".fmt(f)?;
        self.t.fmt(f)?;
        "]".fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Func {
    pub doc: Option<String>,
    pub a: Box<Type>,
    pub b: Box<Type>,
}

impl Func {
    pub fn new(a: impl Into<Type>, b: impl Into<Type>) -> Self {
        Self {
            doc: None,
            a: Box::new(a.into()),
            b: Box::new(b.into()),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        value
            .as_object()
            .and_then(|object| object.get("λ"))
            .map(|n| n.is_string())
            .unwrap_or(false)
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "(".fmt(f)?;
        self.a.fmt(f)?;
        " -> ".fmt(f)?;
        self.b.fmt(f)?;
        ")".fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub doc: Option<String>,
    pub n: String,
    pub fields: Fields,
}

impl Record {
    pub fn new_unit(n: impl Into<String>) -> Self {
        Self {
            doc: None,
            n: n.into(),
            fields: Vec::<UnnamedField>::new().into(),
        }
    }

    pub fn new(n: impl Into<String>, fields: impl Into<Fields>) -> Self {
        Self {
            doc: None,
            n: n.into(),
            fields: fields.into(),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        self.fields.is_compat(value)
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)?;
        self.fields.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple {
    pub doc: Option<String>,
    pub fields: Vec<UnnamedField>,
}

impl Tuple {
    pub fn new(fields: impl Into<Vec<UnnamedField>>) -> Self {
        Self {
            doc: None,
            fields: fields.into(),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        value.is_array()
            && value
                .as_array()
                .map(|arr| {
                    arr.len() >= self.fields.len()
                        && self
                            .fields
                            .iter()
                            .enumerate()
                            .all(|(i, f)| arr.get(i).map(|v| f.t.is_compat(v)).unwrap_or(false))
                })
                .unwrap_or(false)
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "(".fmt(f)?;
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                ", ".fmt(f)?;
            }
            field.t.fmt(f)?;
        }
        ")".fmt(f)
    }
}
