use std::fmt;
use std::collections::BTreeMap;
use ouroboros::type_info::Type as OurType;
use ouroboros::sum::{Optional, UnionVariant};
use ouroboros::product::{Array, Tuple};
use ouroboros::field::{Fields, UnnamedFields, UnnamedField, NamedFields, NamedField};
use rex_type_system::types::{Type as RexType, ADTVariant};

// Note: We cannot use TryFrom/TryInto here as these only work if the types
// they are implemented on are defined in the same crate

#[derive(Debug)]
pub enum ConversionError {
    UnsupportedType,
}

impl std::error::Error for ConversionError {
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self, f)
    }
}

pub fn our_to_rex(our_type: &OurType) -> Result<RexType, ConversionError> {
    match our_type {
        OurType::Unit => Err(ConversionError::UnsupportedType),
        OurType::Bool => Ok(RexType::Bool),
        OurType::I8 => Err(ConversionError::UnsupportedType),
        OurType::I16 => Err(ConversionError::UnsupportedType),
        OurType::I32 => Err(ConversionError::UnsupportedType),
        OurType::I64 => Ok(RexType::Int),
        OurType::I128 => Err(ConversionError::UnsupportedType),
        OurType::U8 => Err(ConversionError::UnsupportedType),
        OurType::U16 => Err(ConversionError::UnsupportedType),
        OurType::U32 => Err(ConversionError::UnsupportedType),
        OurType::U64 => Ok(RexType::Uint),
        OurType::U128 => Err(ConversionError::UnsupportedType),
        OurType::F32 => Err(ConversionError::UnsupportedType),
        OurType::F64 => Ok(RexType::Float),
        OurType::String => Ok(RexType::String),
        OurType::Array(a) => Ok(RexType::List(Box::new(our_to_rex(&a.t)?))),
        OurType::Record(_) => Err(ConversionError::UnsupportedType),
        OurType::Tuple(tuple) => Ok(unnamed_fields_to_rex(&tuple.fields)?),
        OurType::Func(_) => Err(ConversionError::UnsupportedType),
        OurType::Enum(_) => Err(ConversionError::UnsupportedType),
        OurType::Fallible(_) => Err(ConversionError::UnsupportedType),
        OurType::Optional(o) => Ok(RexType::Option(Box::new(our_to_rex(&o.t)?))),
        OurType::Union(u) => {
            let mut adt_variants: Vec<ADTVariant> = Vec::new();
            for v in u.variants.iter() {
                let adt_t: Option<Box<RexType>> = match &v.fields {
                    Some(fields) => Some(Box::new(fields_to_rex(fields)?)),
                    None => None,
                };
                adt_variants.push(ADTVariant {
                    name: v.n.clone(),
                    t: adt_t,
                    docs: None,
                    t_docs: None,
                })
            }
            unimplemented!()
        }
        OurType::Ptr(_) => Err(ConversionError::UnsupportedType),
        OurType::Symbolic(_) => Err(ConversionError::UnsupportedType),
        OurType::Generic(_) => Err(ConversionError::UnsupportedType),
        OurType::Alias(_) => Err(ConversionError::UnsupportedType),
    }
}

pub fn fields_to_rex(fields: &Fields) -> Result<RexType, ConversionError> {
    match fields {
        Fields::Named(named) => named_fields_to_rex(named),
        Fields::Unnamed(unnamed) => unnamed_fields_to_rex(unnamed),
    }
}

fn named_fields_to_rex(fields: &NamedFields) -> Result<RexType, ConversionError> {
    let mut entries: BTreeMap<String, RexType> = BTreeMap::new();
    for field in fields.fields.iter() {
        entries.insert(field.n.clone(), our_to_rex(&field.t)?);
    }
    Ok(RexType::Dict(entries))
}

fn unnamed_fields_to_rex(fields: &UnnamedFields) -> Result<RexType, ConversionError> {
    let mut rex_types: Vec<RexType> = Vec::new();
    for field in fields.iter() {
        rex_types.push(our_to_rex(&field.t)?);
    }
    Ok(RexType::Tuple(rex_types))
}

fn rex_to_fields(rex_type: &RexType) -> Result<Fields, ConversionError> {
    match rex_type {
        RexType::Tuple(rex_types) => {
            let mut our_fields: Vec<UnnamedField> = Vec::new();
            for t in rex_types.iter() {
                our_fields.push(UnnamedField {
                    t: rex_to_our(t)?,
                });
            }
            Ok(Fields::Unnamed(UnnamedFields {
                fields: our_fields,
            }))
        }
        RexType::Dict(entries) => {
            let mut our_fields: Vec<NamedField> = Vec::new();
            for (k, t) in entries.iter() {
                our_fields.push(NamedField {
                    n: k.clone(),
                    t: rex_to_our(t)?,
                });
            }
            Ok(Fields::Named(NamedFields {
                fields: our_fields,
            }))
        }
        _ => return Err(ConversionError::UnsupportedType),
    }
}

pub fn rex_to_our(rex_type: &RexType) -> Result<OurType, ConversionError> {
    match rex_type {
        RexType::UnresolvedVar(_) => Err(ConversionError::UnsupportedType),
        RexType::Var(_) => Err(ConversionError::UnsupportedType),
        RexType::ForAll(_, _, _) => Err(ConversionError::UnsupportedType),
        RexType::ADT(adt) => {

            let mut our_variants: Vec<UnionVariant> = Vec::new();
            for rex_variant in adt.variants.iter() {

                let fields: Option<Fields> = match &rex_variant.t {
                    Some(rex_fields) => Some(rex_to_fields(rex_fields)?),
                    None => None,
                };
                our_variants.push(UnionVariant {
                    n: rex_variant.name.clone(),
                    fields,
                })
            }
            Err(ConversionError::UnsupportedType)
        }
        RexType::Arrow(_, _) => Err(ConversionError::UnsupportedType),
        RexType::Result(_, _) => Err(ConversionError::UnsupportedType),
        RexType::Option(t) => Ok(OurType::Optional(Optional {
            t: Box::new(rex_to_our(t)?)
        })),
        RexType::List(t) => Ok(OurType::Array(Array::new(rex_to_our(t)?))),
        RexType::Dict(_) => Err(ConversionError::UnsupportedType),
        RexType::Tuple(ts) => {
            let mut our_fields: Vec<UnnamedField> = Vec::new();
            for t in ts.iter() {
                our_fields.push(UnnamedField::new(rex_to_our(t)?));
            }
            Ok(OurType::Tuple(Tuple {
                fields: UnnamedFields {
                    fields: our_fields,
                },
            }))
        }
        RexType::Bool => Ok(OurType::Bool),
        RexType::Uint => Ok(OurType::U64),
        RexType::Int => Ok(OurType::I64),
        RexType::Float => Ok(OurType::F64),
        RexType::String => Ok(OurType::String),
        RexType::Uuid => Err(ConversionError::UnsupportedType),
        RexType::DateTime => Err(ConversionError::UnsupportedType),
    }
}
