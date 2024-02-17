use std::fmt::{self, Display, Formatter};

use crate::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ptr {
    pub t: Box<Type>,
}

impl Ptr {
    pub fn new(t: impl Into<Type>) -> Self {
        Self {
            t: Box::new(t.into()),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        value.is_string()
    }
}

impl Display for Ptr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "@".fmt(f)?;
        self.t.fmt(f)
    }
}
