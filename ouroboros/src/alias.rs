use std::fmt::{self, Display, Formatter};

use crate::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alias {
    pub n: String,
    pub t: Box<Type>,
}

impl Alias {
    pub fn new(n: impl Into<String>, t: impl Into<Type>) -> Self {
        Self {
            n: n.into(),
            t: Box::new(t.into()),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        self.t.is_compat(value)
    }
}

impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)
    }
}
