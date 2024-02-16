use serde_json::{Map, Value};

use crate::{
    Array, Enum, EnumVariant, Fields, Func, Generic, Lambda, Optional, Ptr, Record, Symbolic,
    Tuple, Type, Union, UnionVariant,
};

pub trait ValueVisitor {
    type Error;

    fn visit_unit(&mut self, _val: &Value) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_bool(&mut self, _val: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u8(&mut self, _val: u8) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u16(&mut self, _val: u16) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u32(&mut self, _val: u32) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u64(&mut self, _val: u64) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u128(&mut self, _val: u128) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i8(&mut self, _val: i8) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i16(&mut self, _val: i16) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i32(&mut self, _val: i32) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i64(&mut self, _val: i64) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i128(&mut self, _val: i128) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_f32(&mut self, _val: f32) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_f64(&mut self, _val: f64) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_string(&mut self, _val: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_array(&mut self, _arr: &Array, _val: &Vec<Value>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_func(&mut self, _func: &Func, _val: &Lambda<Value, Value>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_record_with_named_fields(
        &mut self,
        _rec: &Record,
        _val: &Map<String, Value>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_record_with_unnamed_fields(
        &mut self,
        _rec: &Record,
        _val: &Vec<Value>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_tuple(&mut self, _tup: &Tuple, _val: &Vec<Value>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_variant_string(
        &mut self,
        _enm: &Enum,
        _var: &EnumVariant,
        _val: &str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_variant_const_value(
        &mut self,
        _enm: &Enum,
        _var: &EnumVariant,
        _val: u8,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_optional(&mut self, _opt: &Optional, _val: Option<&Value>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_union_variant_string(
        &mut self,
        _union: &Union,
        _var: &UnionVariant,
        _val: &str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_union_variant_fields(
        &mut self,
        _union: &Union,
        _var: &UnionVariant,
        _val: &Map<String, Value>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_ptr(&mut self, _ptr: &Ptr, _val: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_symbolic(&mut self, _sym: &Symbolic, _val: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_generic(&mut self, _gen: &Generic, _val: &str) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn walk_value<V>(v: &mut V, t: &Type, val: &Value) -> Result<(), V::Error>
where
    V: ValueVisitor,
{
    if !t.is_compat(val) {
        println!("t: {:?}", t);
        println!("val: {:?}", val);
        todo!()
    }

    match t {
        Type::Unit => v.visit_unit(val),
        Type::Bool => v.visit_bool(val.as_bool().expect("value should be bool")),
        Type::U8 => v.visit_u8(val.as_u64().expect("value should be u8") as u8),
        Type::U16 => v.visit_u16(val.as_u64().expect("value should be u16") as u16),
        Type::U32 => v.visit_u32(val.as_u64().expect("value should be u32") as u32),
        Type::U64 => v.visit_u64(val.as_u64().expect("value should be u64") as u64),
        Type::U128 => v.visit_u128(val.as_u64().expect("value should be u128") as u128),
        Type::I8 => v.visit_i8(val.as_u64().expect("value should be i8") as i8),
        Type::I16 => v.visit_i16(val.as_u64().expect("value should be i16") as i16),
        Type::I32 => v.visit_i32(val.as_u64().expect("value should be i32") as i32),
        Type::I64 => v.visit_i64(val.as_u64().expect("value should be i64") as i64),
        Type::I128 => v.visit_i128(val.as_u64().expect("value should be i128") as i128),
        Type::F32 => v.visit_f32(val.as_f64().expect("value should be f32") as f32),
        Type::F64 => v.visit_f64(val.as_f64().expect("value should be f64") as f64),
        Type::String => v.visit_string(val.as_str().expect("value should be string")),
        Type::Array(arr) => {
            let val = val.as_array().expect("value should be array");
            v.visit_array(arr, val)?;
            for val in val {
                walk_value(v, &arr.t, val)?;
            }
            Ok(())
        }
        Type::Func(func) => {
            let val = serde_json::from_value::<Lambda<Value, Value>>(val.clone())
                .expect("value should be function");
            v.visit_func(func, &val)
        }
        Type::Record(rec) => match &rec.fields {
            Fields::Named(fields) => {
                let val = val.as_object().expect("value should be record");
                v.visit_record_with_named_fields(rec, val)?;
                for field in fields.iter() {
                    walk_value(
                        v,
                        &field.t,
                        val.get(&field.n).expect("value should have record field"),
                    )?;
                }
                Ok(())
            }
            Fields::Unnamed(fields) => {
                let val = val.as_array().expect("value should be record");
                v.visit_record_with_unnamed_fields(rec, val)?;
                for (i, field) in fields.iter().enumerate() {
                    walk_value(
                        v,
                        &field.t,
                        val.get(i).expect("value should have record field"),
                    )?;
                }
                Ok(())
            }
        },
        Type::Tuple(tup) => {
            let val = val.as_array().expect("value should be tuple");
            v.visit_tuple(tup, val)?;
            for (i, field) in tup.fields.iter().enumerate() {
                walk_value(
                    v,
                    &field.t,
                    val.get(i).expect("value should have tuple field"),
                )?;
            }
            Ok(())
        }
        Type::Enum(enm) => match val {
            Value::String(string) => {
                for variant in &enm.variants {
                    if string == &variant.n {
                        return v.visit_enum_variant_string(enm, variant, string);
                    }
                }
                panic!("value should be enum variant (string)")
            }
            Value::Number(num) => {
                for variant in &enm.variants {
                    if num.as_u64() == variant.v.map(|y| y as u64) {
                        return v.visit_enum_variant_const_value(
                            enm,
                            variant,
                            num.as_u64().unwrap() as u8,
                        );
                    }
                }
                panic!("value should be enum variant (const value)")
            }
            _ => panic!("value should be enum variant"),
        },
        Type::Optional(opt) => {
            if val.is_null() {
                v.visit_optional(opt, None)
            } else {
                v.visit_optional(opt, Some(val))?;
                walk_value(v, &opt.t, val)
            }
        }
        Type::Union(union) => match val {
            Value::String(string) => {
                for variant in &union.variants {
                    if let None = variant.fields {
                        if string == &variant.n {
                            return v.visit_union_variant_string(union, variant, string);
                        }
                    }
                }
                panic!("value should be union variant (string)")
            }
            Value::Object(object) => {
                for variant in &union.variants {
                    if object
                        .get(&variant.n)
                        .and_then(|object_fields| {
                            variant
                                .fields
                                .as_ref()
                                .map(|variant_fields| variant_fields.is_compat(object_fields))
                        })
                        .unwrap_or(false)
                    {
                        return v.visit_union_variant_fields(union, variant, object);
                    }
                }
                panic!("value should be union variant (fields)")
            }
            _ => panic!("value should be union variant"),
        },
        Type::Ptr(p) => v.visit_ptr(p, val.as_str().expect("value should be pointer")),
        Type::Symbolic(sym) => v.visit_symbolic(sym, val.as_str().expect("value should be symbol")),
        Type::Generic(gen) => v.visit_generic(gen, val.as_str().expect("value should be generic")),
    }
}

pub trait TypeVisitor {
    type Error;

    fn visit_unit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_bool(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u8(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u16(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u32(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u64(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_u128(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i8(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i16(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i32(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i64(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_i128(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_f32(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_f64(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_string(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_array(&mut self, _arr: &Array) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_func(&mut self, _func: &Func) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_record_with_named_fields(&mut self, _rec: &Record) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_record_with_unnamed_fields(&mut self, _rec: &Record) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_tuple(&mut self, _tup: &Tuple) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_variant_string(
        &mut self,
        _enm: &Enum,
        _var: &EnumVariant,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_variant_const_value(
        &mut self,
        _enm: &Enum,
        _var: &EnumVariant,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_optional(&mut self, _opt: &Optional) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_union_variant_string(&mut self, _var: &UnionVariant) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_union_variant_fields(&mut self, _var: &UnionVariant) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_ptr(&mut self, _p: &Ptr) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_symbolic(&mut self, _sym: &Symbolic) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_generic(&mut self, _gen: &Generic) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn walk_type<V>(v: &mut V, t: &Type) -> Result<(), V::Error>
where
    V: TypeVisitor,
{
    match t {
        Type::Unit => v.visit_unit(),
        Type::Bool => v.visit_bool(),
        Type::U8 => v.visit_u8(),
        Type::U16 => v.visit_u16(),
        Type::U32 => v.visit_u32(),
        Type::U64 => v.visit_u64(),
        Type::U128 => v.visit_u128(),
        Type::I8 => v.visit_i8(),
        Type::I16 => v.visit_i16(),
        Type::I32 => v.visit_i32(),
        Type::I64 => v.visit_i64(),
        Type::I128 => v.visit_i128(),
        Type::F32 => v.visit_f32(),
        Type::F64 => v.visit_f64(),
        Type::String => v.visit_string(),
        Type::Array(arr) => {
            v.visit_array(arr)?;
            walk_type(v, &arr.t)
        }
        Type::Func(func) => {
            v.visit_func(func)?;
            walk_type(v, &func.a)?;
            walk_type(v, &func.b)
        }
        Type::Record(rec) => match &rec.fields {
            Fields::Named(fields) => {
                v.visit_record_with_named_fields(rec)?;
                for field in fields.iter() {
                    walk_type(v, &field.t)?;
                }
                Ok(())
            }
            Fields::Unnamed(fields) => {
                v.visit_record_with_unnamed_fields(rec)?;
                for field in fields.iter() {
                    walk_type(v, &field.t)?;
                }
                Ok(())
            }
        },
        Type::Tuple(tup) => {
            v.visit_tuple(tup)?;
            for field in &tup.fields {
                walk_type(v, &field.t)?;
            }
            Ok(())
        }
        Type::Enum(enm) => {
            for variant in &enm.variants {
                match variant.v {
                    Some(_) => v.visit_enum_variant_const_value(enm, variant)?,
                    None => v.visit_enum_variant_string(enm, variant)?,
                };
            }
            Ok(())
        }
        Type::Optional(opt) => {
            v.visit_optional(opt)?;
            walk_type(v, &opt.t)
        }
        Type::Union(union) => {
            for variant in &union.variants {
                match &variant.fields {
                    Some(_) => v.visit_union_variant_fields(variant)?,
                    None => v.visit_union_variant_string(variant)?,
                };
            }
            Ok(())
        }
        Type::Ptr(p) => {
            v.visit_ptr(p)?;
            walk_type(v, &p.t)
        }
        Type::Symbolic(sym) => v.visit_symbolic(sym),
        Type::Generic(gen) => v.visit_generic(gen),
    }
}
