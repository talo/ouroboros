use crate::{
    field::UnnamedField,
    product::{Array, Record, Tuple},
    sum::{Enum, Optional, Union},
    symbolic::Symbolic,
};

pub trait TypeInfo {
    fn tname() -> String;
    fn t() -> Type;
}

#[derive(Clone)]
pub enum Type {
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
    Array(Array),
    Record(Record),
    Tuple(Tuple),
    Enum(Enum),
    Optional(Optional),
    Union(Union),
    Symbolic(Symbolic),
}

impl From<Array> for Type {
    fn from(t: Array) -> Self {
        Self::Array(t)
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

impl From<Symbolic> for Type {
    fn from(t: Symbolic) -> Self {
        Self::Symbolic(t)
    }
}

impl TypeInfo for bool {
    fn tname() -> String {
        "bool".to_string()
    }

    fn t() -> Type {
        Type::Bool
    }
}

impl TypeInfo for i8 {
    fn tname() -> String {
        "int8".to_string()
    }

    fn t() -> Type {
        Type::I8
    }
}

impl TypeInfo for i16 {
    fn tname() -> String {
        "int16".to_string()
    }

    fn t() -> Type {
        Type::I16
    }
}

impl TypeInfo for i32 {
    fn tname() -> String {
        "int32".to_string()
    }

    fn t() -> Type {
        Type::I32
    }
}

impl TypeInfo for i64 {
    fn tname() -> String {
        "int64".to_string()
    }

    fn t() -> Type {
        Type::I64
    }
}

impl TypeInfo for i128 {
    fn tname() -> String {
        "int128".to_string()
    }

    fn t() -> Type {
        Type::I128
    }
}

impl TypeInfo for u8 {
    fn tname() -> String {
        "uint8".to_string()
    }

    fn t() -> Type {
        Type::U8
    }
}

impl TypeInfo for u16 {
    fn tname() -> String {
        "uint16".to_string()
    }

    fn t() -> Type {
        Type::U16
    }
}

impl TypeInfo for u32 {
    fn tname() -> String {
        "uint32".to_string()
    }

    fn t() -> Type {
        Type::U32
    }
}

impl TypeInfo for u64 {
    fn tname() -> String {
        "uint64".to_string()
    }

    fn t() -> Type {
        Type::U64
    }
}

impl TypeInfo for u128 {
    fn tname() -> String {
        "uint128".to_string()
    }

    fn t() -> Type {
        Type::U128
    }
}

impl TypeInfo for f32 {
    fn tname() -> String {
        "float32".to_string()
    }

    fn t() -> Type {
        Type::F32
    }
}

impl TypeInfo for f64 {
    fn tname() -> String {
        "float64".to_string()
    }

    fn t() -> Type {
        Type::F64
    }
}

impl TypeInfo for String {
    fn tname() -> String {
        "string".to_string()
    }

    fn t() -> Type {
        Type::String
    }
}

impl<T: TypeInfo> TypeInfo for Vec<T> {
    fn tname() -> String {
        format!("[{}]", T::tname())
    }

    fn t() -> Type {
        Type::Array(Array::new(T::t()))
    }
}

impl<T: TypeInfo> TypeInfo for [T] {
    fn tname() -> String {
        format!("[{}]", T::tname())
    }

    fn t() -> Type {
        Type::Array(Array::new(T::t()))
    }
}

impl<T0: TypeInfo> TypeInfo for (T0,) {
    fn tname() -> String {
        format!("({},)", T0::tname())
    }

    fn t() -> Type {
        Type::Tuple(Tuple::new([UnnamedField::new(T0::t())]))
    }
}

impl<T0: TypeInfo, T1: TypeInfo> TypeInfo for (T0, T1) {
    fn tname() -> String {
        format!("({}, {})", T0::tname(), T1::tname())
    }

    fn t() -> Type {
        Type::Tuple(Tuple::new([
            UnnamedField::new(T0::t()),
            UnnamedField::new(T1::t()),
        ]))
    }
}

impl<T0: TypeInfo, T1: TypeInfo, T2: TypeInfo> TypeInfo for (T0, T1, T2) {
    fn tname() -> String {
        format!("({}, {}, {})", T0::tname(), T1::tname(), T2::tname())
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
    fn tname() -> String {
        format!(
            "({}, {}, {}, {})",
            T0::tname(),
            T1::tname(),
            T2::tname(),
            T3::tname()
        )
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
    fn tname() -> String {
        format!("{}?", T::tname())
    }

    fn t() -> Type {
        Type::Optional(Optional::new(T::t()))
    }
}

impl<T: TypeInfo> TypeInfo for Box<T> {
    fn tname() -> String {
        T::tname()
    }

    fn t() -> Type {
        T::t()
    }
}

impl<T: TypeInfo> TypeInfo for &T {
    fn tname() -> String {
        T::tname()
    }

    fn t() -> Type {
        T::t()
    }
}
