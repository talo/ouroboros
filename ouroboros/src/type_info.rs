use std::fmt::{self, Display, Formatter};

use crate::{
    field::UnnamedField,
    product::{Array, Record, Tuple},
    sum::{Enum, Optional, Union},
    symbolic::Symbolic,
    Func, Generic, Ptr,
};

pub trait TypeInfo {
    fn tname() -> TypeName;
    fn t() -> Type;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeName {
    pub n: &'static str,
    pub g: Vec<TypeName>,
}

impl TypeName {
    pub fn new(n: &'static str) -> Self {
        Self { n, g: vec![] }
    }

    pub fn with_generics(n: &'static str, generics: impl Into<Vec<TypeName>>) -> Self {
        Self {
            n,
            g: generics.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    // Basic types
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    String,

    // Product types
    Array(Array),
    Record(Record),
    Tuple(Tuple),
    Func(Func),

    // Sum types
    Enum(Enum),
    Optional(Optional),
    Union(Union),

    // Special types
    Ptr(Ptr),
    Symbolic(Symbolic),
    Generic(Generic),
}

impl Type {
    pub fn n(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::String => "string",
            Self::Array(_) => "array",
            Self::Func(_) => "λ",
            Self::Record(rec) => &rec.n,
            Self::Tuple(_) => "tuple",
            Self::Enum(enm) => &enm.n,
            Self::Optional(_) => "optional",
            Self::Union(union) => &union.n,
            Self::Ptr(_) => "@",
            Self::Symbolic(sym) => &sym.n,
            Self::Generic(gen) => &gen.n,
        }
    }

    pub fn is_compat(&self, value: &serde_json::Value) -> bool {
        match self {
            Self::Bool => value.is_boolean(),
            Self::I8 => value.is_i64(),
            Self::I16 => value.is_i64(),
            Self::I32 => value.is_i64(),
            Self::I64 => value.is_i64(),
            Self::I128 => value.is_i64(),
            Self::U8 => value.is_u64(),
            Self::U16 => value.is_u64(),
            Self::U32 => value.is_u64(),
            Self::U64 => value.is_u64(),
            Self::U128 => value.is_u64(),
            Self::F32 => value.is_f64(),
            Self::F64 => value.is_f64(),
            Self::String => value.is_string(),
            Self::Array(arr) => arr.is_compat(value),
            Self::Func(func) => func.is_compat(value),
            Self::Record(rec) => rec.is_compat(value),
            Self::Tuple(tup) => tup.is_compat(value),
            Self::Enum(enm) => enm.is_compat(value),
            Self::Optional(opt) => opt.is_compat(value),
            Self::Union(union) => union.is_compat(value),
            Self::Ptr(p) => p.is_compat(value),
            Self::Symbolic(sym) => sym.is_compat(value),
            Self::Generic(gen) => gen.is_compat(value),
        }
    }
}

impl From<Array> for Type {
    fn from(t: Array) -> Self {
        Self::Array(t)
    }
}

impl From<Func> for Type {
    fn from(t: Func) -> Self {
        Self::Func(t)
    }
}

impl From<Record> for Type {
    fn from(t: Record) -> Self {
        Self::Record(t)
    }
}

impl From<Tuple> for Type {
    fn from(t: Tuple) -> Self {
        Self::Tuple(t)
    }
}

impl From<Enum> for Type {
    fn from(t: Enum) -> Self {
        Self::Enum(t)
    }
}

impl From<Optional> for Type {
    fn from(t: Optional) -> Self {
        Self::Optional(t)
    }
}

impl From<Union> for Type {
    fn from(t: Union) -> Self {
        Self::Union(t)
    }
}

impl From<Ptr> for Type {
    fn from(t: Ptr) -> Self {
        Self::Ptr(t)
    }
}

impl From<Symbolic> for Type {
    fn from(t: Symbolic) -> Self {
        Self::Symbolic(t)
    }
}

impl From<Generic> for Type {
    fn from(t: Generic) -> Self {
        Self::Generic(t)
    }
}

impl TypeInfo for bool {
    fn tname() -> TypeName {
        TypeName {
            n: "bool",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::Bool
    }
}

impl TypeInfo for i8 {
    fn tname() -> TypeName {
        TypeName {
            n: "int8",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::I8
    }
}

impl TypeInfo for i16 {
    fn tname() -> TypeName {
        TypeName {
            n: "int16",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::I16
    }
}

impl TypeInfo for i32 {
    fn tname() -> TypeName {
        TypeName {
            n: "int32",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::I32
    }
}

impl TypeInfo for i64 {
    fn tname() -> TypeName {
        TypeName {
            n: "int64",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::I64
    }
}

impl TypeInfo for i128 {
    fn tname() -> TypeName {
        TypeName {
            n: "int128",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::I128
    }
}

impl TypeInfo for u8 {
    fn tname() -> TypeName {
        TypeName {
            n: "uint8",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::U8
    }
}

impl TypeInfo for u16 {
    fn tname() -> TypeName {
        TypeName {
            n: "uint16",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::U16
    }
}

impl TypeInfo for u32 {
    fn tname() -> TypeName {
        TypeName {
            n: "uint32",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::U32
    }
}

impl TypeInfo for u64 {
    fn tname() -> TypeName {
        TypeName {
            n: "uint64",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::U64
    }
}

impl TypeInfo for u128 {
    fn tname() -> TypeName {
        TypeName {
            n: "uint128",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::U128
    }
}

impl TypeInfo for f32 {
    fn tname() -> TypeName {
        TypeName {
            n: "float32",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::F32
    }
}

impl TypeInfo for f64 {
    fn tname() -> TypeName {
        TypeName {
            n: "float64",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::F64
    }
}

impl TypeInfo for String {
    fn tname() -> TypeName {
        TypeName {
            n: "string",
            g: vec![],
        }
    }

    fn t() -> Type {
        Type::String
    }
}

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn tname() -> TypeName {
        TypeName {
            n: "array",
            g: vec![T::tname()],
        }
    }

    fn t() -> Type {
        Type::Array(Array::new(T::t()))
    }
}

impl<T: TypeInfo> TypeInfo for [T] {
    fn tname() -> TypeName {
        TypeName {
            n: "array",
            g: vec![T::tname()],
        }
    }

    fn t() -> Type {
        Type::Array(Array::new(T::t()))
    }
}

impl<T0: TypeInfo> TypeInfo for (T0,) {
    fn tname() -> TypeName {
        TypeName {
            n: "tuple",
            g: vec![T0::tname()],
        }
    }

    fn t() -> Type {
        Type::Tuple(Tuple::new([UnnamedField::new(T0::t())]))
    }
}

impl<T0: TypeInfo, T1: TypeInfo> TypeInfo for (T0, T1) {
    fn tname() -> TypeName {
        TypeName {
            n: "tuple",
            g: vec![T0::tname(), T1::tname()],
        }
    }

    fn t() -> Type {
        Type::Tuple(Tuple::new([
            UnnamedField::new(T0::t()),
            UnnamedField::new(T1::t()),
        ]))
    }
}

impl<T0: TypeInfo, T1: TypeInfo, T2: TypeInfo> TypeInfo for (T0, T1, T2) {
    fn tname() -> TypeName {
        TypeName {
            n: "tuple",
            g: vec![T0::tname(), T1::tname(), T2::tname()],
        }
    }

    fn t() -> Type {
        Type::Tuple(Tuple::new([
            UnnamedField::new(T0::t()),
            UnnamedField::new(T1::t()),
            UnnamedField::new(T2::t()),
        ]))
    }
}

impl<T0: TypeInfo, T1: TypeInfo, T2: TypeInfo, T3: TypeInfo> TypeInfo for (T0, T1, T2, T3) {
    fn tname() -> TypeName {
        TypeName {
            n: "tuple",
            g: vec![T0::tname(), T1::tname(), T2::tname(), T3::tname()],
        }
    }

    fn t() -> Type {
        Type::Tuple(Tuple::new([
            UnnamedField::new(T0::t()),
            UnnamedField::new(T1::t()),
            UnnamedField::new(T2::t()),
            UnnamedField::new(T3::t()),
        ]))
    }
}

impl<T: TypeInfo> TypeInfo for Option<T> {
    fn tname() -> TypeName {
        TypeName {
            n: "optional",
            g: vec![T::tname()],
        }
    }

    fn t() -> Type {
        Type::Optional(Optional::new(T::t()))
    }
}

impl<T: TypeInfo> TypeInfo for Box<T> {
    fn tname() -> TypeName {
        T::tname()
    }

    fn t() -> Type {
        T::t()
    }
}

impl<T: TypeInfo> TypeInfo for &T {
    fn tname() -> TypeName {
        T::tname()
    }

    fn t() -> Type {
        T::t()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool => "bool".fmt(f),
            Self::I8 => "i8".fmt(f),
            Self::I16 => "i16".fmt(f),
            Self::I32 => "i32".fmt(f),
            Self::I64 => "i64".fmt(f),
            Self::I128 => "i128".fmt(f),
            Self::U8 => "u8".fmt(f),
            Self::U16 => "u16".fmt(f),
            Self::U32 => "u32".fmt(f),
            Self::U64 => "u64".fmt(f),
            Self::U128 => "u128".fmt(f),
            Self::F32 => "f32".fmt(f),
            Self::F64 => "f64".fmt(f),
            Self::String => "string".fmt(f),
            Self::Array(arr) => arr.fmt(f),
            Self::Func(func) => func.fmt(f),
            Self::Record(rec) => rec.fmt(f),
            Self::Tuple(tup) => tup.fmt(f),
            Self::Enum(enm) => enm.fmt(f),
            Self::Optional(opt) => opt.fmt(f),
            Self::Union(union) => union.fmt(f),
            Self::Ptr(p) => p.fmt(f),
            Self::Symbolic(sym) => sym.fmt(f),
            Self::Generic(gen) => gen.fmt(f),
        }
    }
}
