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
            && value
                .as_str()
                .map(|s| s.starts_with("$") && (s[1..] == self.n))
                .unwrap_or(false)
    }
}
