use crate::type_info::Type;

#[derive(Clone)]
pub struct NamedField {
    pub n: String,
    pub t: Type,
}

impl NamedField {
    pub fn new(n: impl Into<String>, t: Type) -> Self {
        Self { n: n.into(), t }
    }
}

#[derive(Clone)]
pub struct UnnamedField {
    pub t: Type,
}

impl UnnamedField {
    pub fn new(t: Type) -> Self {
        Self { t }
    }
}

#[derive(Clone)]
pub enum Fields {
    Unnamed(Vec<UnnamedField>),
    Named(Vec<NamedField>),
}

impl Fields {
    pub fn unnamed(fields: impl Into<Vec<UnnamedField>>) -> Self {
        Self::Unnamed(fields.into())
    }

    pub fn named(fields: impl Into<Vec<NamedField>>) -> Self {
        Self::Named(fields.into())
    }
}

impl From<Vec<UnnamedField>> for Fields {
    fn from(fields: Vec<UnnamedField>) -> Self {
        Self::Unnamed(fields)
    }
}

impl From<Vec<NamedField>> for Fields {
    fn from(fields: Vec<NamedField>) -> Self {
        Self::Named(fields)
    }
}

impl<const N: usize> From<[UnnamedField; N]> for Fields {
    fn from(fields: [UnnamedField; N]) -> Self {
        Self::Unnamed(fields.into())
    }
}

impl<const N: usize> From<[NamedField; N]> for Fields {
    fn from(fields: [NamedField; N]) -> Self {
        Self::Named(fields.into())
    }
}
