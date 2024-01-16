use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{Func, Type, TypeInfo};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lambda<A, B> {
    #[serde(rename = "λ")]
    pub n: String,
    pub extras: serde_json::Value,
    pub captured_args: Vec<serde_json::Value>,

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
            captured_args: vec![],

            _args: PhantomData,
            _ret: PhantomData,
        }
    }

    pub fn with_extras(n: impl Into<String>, extras: serde_json::Value) -> Self {
        Self {
            n: n.into(),
            extras,
            captured_args: vec![],

            _args: PhantomData,
            _ret: PhantomData,
        }
    }

    pub fn with_captured_args(n: impl Into<String>, captured_args: Vec<serde_json::Value>) -> Self {
        Self {
            n: n.into(),
            extras: serde_json::Value::Null,
            captured_args,

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

pub trait Curry {
    type Arg: Serialize;
    type Closure;
    type Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error>;
}

impl<A0, A1, B> Curry for Lambda<(A0, A1), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<A1, B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}

impl<A0, A1, A2, B> Curry for Lambda<(A0, A1, A2), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<(A1, A2), B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}

impl<A0, A1, A2, A3, B> Curry for Lambda<(A0, A1, A2, A3), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<(A1, A2, A3), B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}

impl<A0, A1, A2, A3, A4, B> Curry for Lambda<(A0, A1, A2, A3, A4), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<(A1, A2, A3, A4), B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}

impl<A0, A1, A2, A3, A4, A5, B> Curry for Lambda<(A0, A1, A2, A3, A4, A5), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<(A1, A2, A3, A4, A5), B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, B> Curry for Lambda<(A0, A1, A2, A3, A4, A5, A6), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<(A1, A2, A3, A4, A5, A6), B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}

impl<A0, A1, A2, A3, A4, A5, A6, A7, B> Curry for Lambda<(A0, A1, A2, A3, A4, A5, A6, A7), B>
where
    A0: Serialize,
{
    type Arg = A0;
    type Closure = Lambda<(A1, A2, A3, A4, A5, A6, A7), B>;
    type Error = serde_json::Error;

    fn curry(self, arg: Self::Arg) -> Result<Self::Closure, Self::Error> {
        let mut captured_args = self.captured_args;
        captured_args.push(serde_json::to_value(&arg)?);
        Ok(Lambda {
            n: self.n,
            extras: self.extras,
            captured_args,

            _args: PhantomData,
            _ret: PhantomData,
        })
    }
}
