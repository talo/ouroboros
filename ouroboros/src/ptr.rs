use std::fmt::{self, Display, Formatter};

use crate::{Error, Result, Type};

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

    pub fn is_compat(&self, value: Option<&serde_json::Value>) -> Result<()> {
        match value {
            Some(value) => {
                if value.is_string() {
                    Ok(())
                } else {
                    Err(Error::InvalidPtr {
                        expected: self.clone(),
                        e: Error::UnexpectedValue { got: value.clone() }.into(),
                    })
                }
            }
            None => Err(Error::InvalidPtr {
                expected: self.clone(),
                e: Error::UnexpectedNull.into(),
            }),
        }
    }
}

impl Display for Ptr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "@".fmt(f)?;
        self.t.fmt(f)
    }
}
