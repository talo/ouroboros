use std::{
    borrow::Borrow,
    collections::HashSet,
    hash::Hash,
    sync::{Arc, Mutex},
};

pub use ouroboros_proc_macro::*;

lazy_static::lazy_static! {
    static ref CPP_TYPE_NAMES: Arc<Mutex<HashSet<String>>> = {
        Arc::new(Mutex::new(HashSet::new()))
    };

    static ref PYTHON_TYPE_NAMES: Arc<Mutex<HashSet<String>>> = {
        Arc::new(Mutex::new(HashSet::new()))
    };
}

pub fn register_cpp_type_name(name: String) {
    let mut mu = CPP_TYPE_NAMES.lock().unwrap();
    mu.insert(name);
}

pub fn register_python_type_name(name: String) {
    let mut mu = PYTHON_TYPE_NAMES.lock().unwrap();
    mu.insert(name);
}

pub fn is_cpp_type_name_registered<Q>(name: &Q) -> bool
where
    Q: ?Sized + Hash + Eq,
    String: Borrow<Q>,
{
    CPP_TYPE_NAMES.lock().unwrap().contains(name)
}

pub fn is_python_type_name_registered<Q>(name: &Q) -> bool
where
    Q: ?Sized + Hash + Eq,
    String: Borrow<Q>,
{
    PYTHON_TYPE_NAMES.lock().unwrap().contains(name)
}

pub mod field;
pub mod product;
pub mod sum;
pub mod symbolic;
pub mod transpile;
pub mod type_info;

pub trait Ouroboros {
    fn python() -> String;
    fn cpp() -> String;
}

impl Ouroboros for bool {
    fn python() -> String {
        "bool".to_string()
    }

    fn cpp() -> String {
        "bool".to_string()
    }
}

impl Ouroboros for u8 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "uint8_t".to_string()
    }
}

impl Ouroboros for u16 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "uint16_t".to_string()
    }
}

impl Ouroboros for u32 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "uint32_t".to_string()
    }
}

impl Ouroboros for u64 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "uint64_t".to_string()
    }
}

impl Ouroboros for u128 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "uint128_t".to_string()
    }
}

impl Ouroboros for i8 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "int8_t".to_string()
    }
}

impl Ouroboros for i16 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "int16_t".to_string()
    }
}

impl Ouroboros for i32 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "int32_t".to_string()
    }
}

impl Ouroboros for i64 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "int64_t".to_string()
    }
}

impl Ouroboros for i128 {
    fn python() -> String {
        "int".to_string()
    }

    fn cpp() -> String {
        "int128_t".to_string()
    }
}

impl Ouroboros for f32 {
    fn python() -> String {
        "float".to_string()
    }

    fn cpp() -> String {
        "float".to_string()
    }
}

impl Ouroboros for f64 {
    fn python() -> String {
        "float".to_string()
    }

    fn cpp() -> String {
        "double".to_string()
    }
}

impl Ouroboros for String {
    fn python() -> String {
        "str".to_string()
    }

    fn cpp() -> String {
        "std::string".to_string()
    }
}

impl<T> Ouroboros for Vec<T>
where
    T: Ouroboros,
{
    fn python() -> String {
        format!("list[{}]", T::python())
    }

    fn cpp() -> String {
        format!("std::vector<{}>", T::cpp())
    }
}

impl<T> Ouroboros for Option<T>
where
    T: Ouroboros,
{
    fn python() -> String {
        format!("Optional[{}]", T::python())
    }

    fn cpp() -> String {
        format!("std::optional<{}>", T::cpp())
    }
}

impl<T0> Ouroboros for (T0,)
where
    T0: Ouroboros,
{
    fn python() -> String {
        format!("tuple[{}]", T0::python())
    }

    fn cpp() -> String {
        format!("std::tuple<{}>", T0::cpp())
    }
}

impl<T0, T1> Ouroboros for (T0, T1)
where
    T0: Ouroboros,
    T1: Ouroboros,
{
    fn python() -> String {
        format!("tuple[{}, {}]", T0::python(), T1::python())
    }

    fn cpp() -> String {
        format!("std::tuple<{}, {}>", T0::cpp(), T1::cpp())
    }
}

impl<T0, T1, T2> Ouroboros for (T0, T1, T2)
where
    T0: Ouroboros,
    T1: Ouroboros,
    T2: Ouroboros,
{
    fn python() -> String {
        format!(
            "tuple[{}, {}, {}]",
            T0::python(),
            T1::python(),
            T2::python()
        )
    }

    fn cpp() -> String {
        format!("std::tuple<{}, {}, {}>", T0::cpp(), T1::cpp(), T2::cpp())
    }
}

impl<T0, T1, T2, T3> Ouroboros for (T0, T1, T2, T3)
where
    T0: Ouroboros,
    T1: Ouroboros,
    T2: Ouroboros,
    T3: Ouroboros,
{
    fn python() -> String {
        format!(
            "tuple[{}, {}, {}, {}]",
            T0::python(),
            T1::python(),
            T2::python(),
            T3::python()
        )
    }

    fn cpp() -> String {
        format!(
            "std::tuple<{}, {}, {}, {}>",
            T0::cpp(),
            T1::cpp(),
            T2::cpp(),
            T3::cpp()
        )
    }
}

impl<T0, T1, T2, T3, T4> Ouroboros for (T0, T1, T2, T3, T4)
where
    T0: Ouroboros,
    T1: Ouroboros,
    T2: Ouroboros,
    T3: Ouroboros,
    T4: Ouroboros,
{
    fn python() -> String {
        format!(
            "tuple[{}, {}, {}, {}, {}]",
            T0::python(),
            T1::python(),
            T2::python(),
            T3::python(),
            T4::python()
        )
    }

    fn cpp() -> String {
        format!(
            "std::tuple<{}, {}, {}, {}, {}>",
            T0::cpp(),
            T1::cpp(),
            T2::cpp(),
            T3::cpp(),
            T4::cpp()
        )
    }
}

impl<T0, T1, T2, T3, T4, T5> Ouroboros for (T0, T1, T2, T3, T4, T5)
where
    T0: Ouroboros,
    T1: Ouroboros,
    T2: Ouroboros,
    T3: Ouroboros,
    T4: Ouroboros,
    T5: Ouroboros,
{
    fn python() -> String {
        format!(
            "tuple[{}, {}, {}, {}, {}, {}]",
            T0::python(),
            T1::python(),
            T2::python(),
            T3::python(),
            T4::python(),
            T5::python()
        )
    }

    fn cpp() -> String {
        format!(
            "std::tuple<{}, {}, {}, {}, {}, {}>",
            T0::cpp(),
            T1::cpp(),
            T2::cpp(),
            T3::cpp(),
            T4::cpp(),
            T5::cpp()
        )
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Ouroboros for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Ouroboros,
    T1: Ouroboros,
    T2: Ouroboros,
    T3: Ouroboros,
    T4: Ouroboros,
    T5: Ouroboros,
    T6: Ouroboros,
{
    fn python() -> String {
        format!(
            "tuple[{}, {}, {}, {}, {}, {}, {}]",
            T0::python(),
            T1::python(),
            T2::python(),
            T3::python(),
            T4::python(),
            T5::python(),
            T6::python()
        )
    }

    fn cpp() -> String {
        format!(
            "std::tuple<{}, {}, {}, {}, {}, {}, {}>",
            T0::cpp(),
            T1::cpp(),
            T2::cpp(),
            T3::cpp(),
            T4::cpp(),
            T5::cpp(),
            T6::cpp()
        )
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Ouroboros for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Ouroboros,
    T1: Ouroboros,
    T2: Ouroboros,
    T3: Ouroboros,
    T4: Ouroboros,
    T5: Ouroboros,
    T6: Ouroboros,
    T7: Ouroboros,
{
    fn python() -> String {
        format!(
            "tuple[{}, {}, {}, {}, {}, {}, {}, {}]",
            T0::python(),
            T1::python(),
            T2::python(),
            T3::python(),
            T4::python(),
            T5::python(),
            T6::python(),
            T7::python()
        )
    }

    fn cpp() -> String {
        format!(
            "std::tuple<{}, {}, {}, {}, {}, {}, {}, {}>",
            T0::cpp(),
            T1::cpp(),
            T2::cpp(),
            T3::cpp(),
            T4::cpp(),
            T5::cpp(),
            T6::cpp(),
            T7::cpp()
        )
    }
}
