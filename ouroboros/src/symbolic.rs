use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbolic {
    pub doc: Option<String>,
    pub n: String,
}

impl Symbolic {
    pub fn new(n: impl Into<String>) -> Self {
        Self {
            doc: None,
            n: n.into(),
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        value.is_string()
    }
}

impl Display for Symbolic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "$".fmt(f)?;
        self.n.fmt(f)
    }
}
