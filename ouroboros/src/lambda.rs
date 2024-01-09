use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{Func, Type, TypeInfo};

#[derive(Deserialize, Serialize)]
pub struct Lambda<A, B> {
    #[serde(rename = "λ")]
    pub n: String,
    pub extras: serde_json::Value,

    #[serde(skip)]
    pub _args: PhantomData<A>,
    #[serde(skip)]
    pub _ret: PhantomData<B>,
}

impl<A, B> Lambda<A, B> {
    pub fn new(n: impl Into<String>) -> Self {
        Self {
            n: n.into(),
            extras: serde_json::Value::Null,

            _args: PhantomData,
            _ret: PhantomData,
        }
    }

    pub fn with_extras(n: impl Into<String>, extras: serde_json::Value) -> Self {
        Self {
            n: n.into(),
            extras,

            _args: PhantomData,
            _ret: PhantomData,
        }
    }
}

impl<A, B> TypeInfo for Lambda<A, B>
where
    A: TypeInfo,
    B: TypeInfo,
{
    fn tname() -> String {
        format!("({} -> {})", A::tname(), B::tname())
    }

    fn t() -> Type {
        Type::from(Func::new(A::t(), B::t()))
    }
}
