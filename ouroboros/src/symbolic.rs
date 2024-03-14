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

    pub fn is_compat(&self, value: Option<&serde_json::Value>) -> bool {
        match value {
            Some(value) => value.is_string(),
            None => false,
        }
    }
}

impl Display for Symbolic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "$".fmt(f)?;
        self.n.fmt(f)
    }
}
