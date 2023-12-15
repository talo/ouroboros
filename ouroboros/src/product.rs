use crate::{
    field::{Fields, UnnamedField},
    type_info::Type,
};

#[derive(Clone)]
pub struct Array {
    pub t: Box<Type>,
}

impl Array {
    pub fn new(t: impl Into<Type>) -> Self {
        Self {
            t: Box::new(t.into()),
        }
    }
}

#[derive(Clone)]
pub struct Record {
    pub n: String,
    pub fields: Fields,
}

impl Record {
    pub fn new(n: impl Into<String>, fields: impl Into<Fields>) -> Self {
        Self {
            n: n.into(),
            fields: fields.into(),
        }
    }
}

#[derive(Clone)]
pub struct Tuple {
    pub fields: Vec<UnnamedField>,
}

impl Tuple {
    pub fn new(fields: impl Into<Vec<UnnamedField>>) -> Self {
        Self {
            fields: fields.into(),
        }
    }
}
