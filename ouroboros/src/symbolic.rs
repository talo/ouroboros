#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbolic {
    pub n: String,
}

impl Symbolic {
    pub fn new(n: impl Into<String>) -> Self {
        Self { n: n.into() }
    }
}
