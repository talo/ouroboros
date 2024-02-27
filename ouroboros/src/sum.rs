use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::{field::Fields, type_info::Type, Error, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enum {
    pub doc: Option<String>,
    pub n: String,
    pub variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(n: impl Into<String>, variants: impl Into<Vec<EnumVariant>>) -> Self {
        Self {
            doc: None,
            n: n.into(),
            variants: variants.into(),
        }
    }

    pub fn is_compat(&self, value: Option<&serde_json::Value>) -> Result<()> {
        match value {
            Some(value) => {
                if let Some(x) = value.as_str() {
                    self.variants
                        .iter()
                        .any(|variant| variant.n == x)
                        .then_some(())
                        .ok_or(Error::InvalidEnum {
                            expected: self.clone(),
                            e: Error::UnexpectedValue { got: value.clone() }.into(),
                        })
                } else if let Some(x) = value.as_u64() {
                    self.variants
                        .iter()
                        .any(|variant| variant.v.map(|u| u as u64 == x).unwrap_or(false))
                        .then_some(())
                        .ok_or(Error::InvalidEnum {
                            expected: self.clone(),
                            e: Error::UnexpectedValue { got: value.clone() }.into(),
                        })
                } else {
                    Err(Error::InvalidEnum {
                        expected: self.clone(),
                        e: Error::UnexpectedValue { got: value.clone() }.into(),
                    })
                }
            }
            None => Err(Error::InvalidEnum {
                expected: self.clone(),
                e: Error::UnexpectedNull.into(),
            }),
        }
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)
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

    pub fn is_compat(&self, value: Option<&serde_json::Value>) -> Result<()> {
        match value {
            None => Ok(()),
            Some(serde_json::Value::Null) => Ok(()),
            Some(value) => self
                .t
                .is_compat(Some(value))
                .map_err(|e| Error::InvalidOptional {
                    expected: self.clone(),
                    e: e.into(),
                }),
        }
    }
}

impl Display for Optional {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.t.fmt(f)?;
        "?".fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct Union {
    pub doc: Option<String>,
    pub n: String,
    pub variants: Vec<UnionVariant>,
}

impl Union {
    pub fn new(n: impl Into<String>, variants: impl Into<Vec<UnionVariant>>) -> Self {
        Self {
            doc: None,
            n: n.into(),
            variants: variants.into(),
        }
    }

    pub fn is_compat(&self, value: Option<&serde_json::Value>) -> Result<()> {
        match value {
            Some(value) => {
                if let Some(string) = value.as_str() {
                    self.variants
                        .iter()
                        .any(|variant| variant.n == string)
                        .then_some(())
                        .ok_or(Error::InvalidUnion {
                            expected: self.clone(),
                            e: Error::UnexpectedValue { got: value.clone() }.into(),
                        })
                } else if let Some(object) = value.as_object() {
                    self.variants
                        .iter()
                        .map(|variant| {
                            object.get(&variant.n).map(|object_fields| {
                                variant
                                    .fields
                                    .as_ref()
                                    .map(|variant_fields| {
                                        variant_fields.is_compat(Some(object_fields)).map_err(|e| {
                                            Error::InvalidUnion {
                                                expected: self.clone(),
                                                e: e.into(),
                                            }
                                        })
                                    })
                                    .unwrap_or(Ok(()))
                            })
                        })
                        .collect::<Option<Result<_>>>()
                        .unwrap_or(Err(Error::InvalidUnion {
                            expected: self.clone(),
                            e: Error::UnexpectedValue { got: value.clone() }.into(),
                        }))
                } else {
                    Err(Error::InvalidUnion {
                        expected: self.clone(),
                        e: Error::UnexpectedValue { got: value.clone() }.into(),
                    })
                }
            }
            None => Err(Error::InvalidUnion {
                expected: self.clone(),
                e: Error::UnexpectedNull.into(),
            }),
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

impl Display for Union {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)
    }
}

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
