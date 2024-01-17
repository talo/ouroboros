// Self::Array(arr) => arr.is_compat(value),
//             Self::Func(func) => func.is_compat(value),
//             Self::Record(rec) => rec.is_compat(value),
//             Self::Tuple(tup) => tup.is_compat(value),
//             Self::Enum(enm) => enm.is_compat(value),
//             Self::Optional(opt) => opt.is_compat(value),
//             Self::Union(union) => union.is_compat(value),
//             Self::Symbolic(sym) => sym.is_compat(value),
//             Self::Generic(gen) => gen.is_compat(value),

use serde_json::{Map, Value};

use crate::{
    Array, Enum, Fields, Func, Generic, Lambda, Optional, Record, Symbolic, Tuple, Type, Union,
};

pub trait Visitor {
    fn visit_bool(&mut self, _val: bool) {}
    fn visit_u8(&mut self, _val: u8) {}
    fn visit_u16(&mut self, _val: u16) {}
    fn visit_u32(&mut self, _val: u32) {}
    fn visit_u64(&mut self, _val: u64) {}
    fn visit_u128(&mut self, _val: u128) {}
    fn visit_i8(&mut self) {}
    fn visit_i16(&mut self) {}
    fn visit_i32(&mut self) {}
    fn visit_i64(&mut self) {}
    fn visit_i128(&mut self) {}
    fn visit_f32(&mut self) {}
    fn visit_f64(&mut self) {}
    fn visit_string(&mut self) {}
    fn visit_array(&mut self, _arr: &Array, _val: &Vec<Value>) {}
    fn visit_func(&mut self, _func: &Func, _val: &Lambda<Value, Value>) {}
    fn visit_record_with_named_fields(&mut self, _rec: &Record, _val: &Map<String, Value>) {}
    fn visit_record_with_unnamed_fields(&mut self, _rec: &Record, _val: &Vec<Value>) {}
    fn visit_tuple(&mut self, _tup: &Tuple, _val: &Vec<Value>) {}
    fn visit_enum(&mut self, _enm: &Enum) {}
    fn visit_optional(&mut self, _opt: &Optional, _val: Option<&Value>) {}
    fn visit_union(&mut self, _union: &Union) {}
    fn visit_symbolic(&mut self, _sym: &Symbolic, _val: &str) {}
    fn visit_generic(&mut self, _gen: &Generic, _val: &str) {}
}

pub fn walk<V: Visitor>(v: &mut V, t: &Type, val: &Value) {
    if !t.is_compat(val) {
        todo!()
    }

    match t {
        Type::Bool => v.visit_bool(val.as_bool().expect("value should be bool")),
        Type::U8 => v.visit_u8(val.as_u64().expect("value should be u8") as u8),
        Type::U16 => v.visit_u16(val.as_u64().expect("value should be u8") as u16),
        Type::U32 => v.visit_u32(val.as_u64().expect("value should be u8") as u32),
        Type::U64 => v.visit_u64(val.as_u64().expect("value should be u8") as u64),
        Type::U128 => v.visit_u128(val.as_u64().expect("value should be u8") as u128),
        Type::I8 => v.visit_i8(),
        Type::I16 => v.visit_i16(),
        Type::I32 => v.visit_i32(),
        Type::I64 => v.visit_i64(),
        Type::I128 => v.visit_i128(),
        Type::F32 => v.visit_f32(),
        Type::F64 => v.visit_f64(),
        Type::String => v.visit_string(),
        Type::Array(arr) => {
            let val = val.as_array().expect("value should be array");
            v.visit_array(arr, val);
            for val in val {
                walk(v, &arr.t, val)
            }
        }
        Type::Func(func) => {
            let val = serde_json::from_value::<Lambda<Value, Value>>(val.clone())
                .expect("value should be func");
            v.visit_func(func, &val);
            // // You cannot walk further into a function because it does not actually contain its arguments
            // walk(v, &func.a, todo!());
            // walk(v, &func.b, todo!());
        }
        Type::Record(rec) => match &rec.fields {
            Fields::Named(fields) => {
                let val = val.as_object().expect("value should be record");
                v.visit_record_with_named_fields(rec, val);
                for field in fields {
                    walk(
                        v,
                        &field.t,
                        val.get(&field.n).expect("value should have record field"),
                    )
                }
            }
            Fields::Unnamed(fields) => {
                let val = val.as_array().expect("value should be record");
                v.visit_record_with_unnamed_fields(rec, val);
                for (i, field) in fields.iter().enumerate() {
                    walk(
                        v,
                        &field.t,
                        val.get(i).expect("value should have record field"),
                    )
                }
            }
        },
        Type::Tuple(tup) => {
            let val = val.as_array().expect("value should be tuple");
            v.visit_tuple(tup, val);
            for (i, field) in tup.fields.iter().enumerate() {
                walk(
                    v,
                    &field.t,
                    val.get(i).expect("value should have tuple field"),
                )
            }
        }
        Type::Enum(_) => todo!(),
        Type::Optional(opt) => {
            if val.is_null() {
                v.visit_optional(opt, None);
            } else {
                v.visit_optional(opt, Some(val));
                walk(v, &opt.t, val)
            }
        }
        Type::Union(_) => todo!(),
        Type::Symbolic(sym) => v.visit_symbolic(sym, val.as_str().expect("value should be symbol")),
        Type::Generic(gen) => v.visit_generic(gen, val.as_str().expect("value should be generic")),
    }
}
