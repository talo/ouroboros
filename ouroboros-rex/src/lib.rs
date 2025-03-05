use std::collections::BTreeMap;
use ouroboros::type_info::Type as OurType;
use ouroboros::sum::{Optional, UnionVariant};
use ouroboros::product::{Array, Tuple};
use ouroboros::field::{Fields, UnnamedFields, UnnamedField, NamedFields, NamedField};
use rex_type_system::types::{Type as RexType, ADTVariant};

// Note: We cannot use From/Into here as these only work if the types
// they are implemented on are defined in the same crate

pub fn our_to_rex(our_type: &OurType) -> RexType {
    match our_type {
        OurType::Unit => panic!("Unsupported ouroboros type: Unit"),
        OurType::Bool => RexType::Bool,
        OurType::I8 => panic!("Unsupported ouroboros type: I8"),
        OurType::I16 => panic!("Unsupported ouroboros type: I16"),
        OurType::I32 => panic!("Unsupported ouroboros type: I32"),
        OurType::I64 => RexType::Int,
        OurType::I128 => panic!("Unsupported ouroboros type: I128"),
        OurType::U8 => panic!("Unsupported ouroboros type: U8"),
        OurType::U16 => panic!("Unsupported ouroboros type: U16"),
        OurType::U32 => panic!("Unsupported ouroboros type: U32"),
        OurType::U64 => RexType::Uint,
        OurType::U128 => panic!("Unsupported ouroboros type: U128"),
        OurType::F32 => panic!("Unsupported ouroboros type: F32"),
        OurType::F64 => RexType::Float,
        OurType::String => RexType::String,
        OurType::Array(a) => RexType::List(Box::new(our_to_rex(&a.t))),
        OurType::Record(_) => panic!("Unsupported ouroboros type: Record"),
        OurType::Tuple(tuple) => unnamed_fields_to_rex(&tuple.fields),
        OurType::Func(_) => panic!("Unsupported ouroboros type: Func"),
        OurType::Enum(_) => panic!("Unsupported ouroboros type: Enum"),
        OurType::Fallible(_) => panic!("Unsupported ouroboros type: Fallible"),
        OurType::Optional(o) => RexType::Option(Box::new(our_to_rex(&o.t))),
        OurType::Union(u) => {
            let mut adt_variants: Vec<ADTVariant> = Vec::new();
            for v in u.variants.iter() {
                let adt_t: Option<Box<RexType>> = match &v.fields {
                    Some(fields) => Some(Box::new(fields_to_rex(fields))),
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
        OurType::Ptr(_) => panic!("Unsupported ouroboros type: Ptr"),
        OurType::Symbolic(_) => panic!("Unsupported ouroboros type: Symbolic"),
        OurType::Generic(_) => panic!("Unsupported ouroboros type: Generic"),
        OurType::Alias(_) => panic!("Unsupported ouroboros type: Alias"),
    }
}

pub fn fields_to_rex(fields: &Fields) -> RexType {
    match fields {
        Fields::Named(named) => named_fields_to_rex(named),
        Fields::Unnamed(unnamed) => unnamed_fields_to_rex(unnamed),
    }
}

fn named_fields_to_rex(fields: &NamedFields) -> RexType {
    let mut entries: BTreeMap<String, RexType> = BTreeMap::new();
    for field in fields.fields.iter() {
        entries.insert(field.n.clone(), our_to_rex(&field.t));
    }
    RexType::Dict(entries)
}

fn unnamed_fields_to_rex(fields: &UnnamedFields) -> RexType {
    let mut rex_types: Vec<RexType> = Vec::new();
    for field in fields.iter() {
        rex_types.push(our_to_rex(&field.t));
    }
    RexType::Tuple(rex_types)
}

fn rex_to_fields(rex_type: &RexType) -> Fields {
    match rex_type {
        RexType::Tuple(rex_types) => {
            let mut our_fields: Vec<UnnamedField> = Vec::new();
            for t in rex_types.iter() {
                our_fields.push(UnnamedField {
                    t: rex_to_our(t),
                });
            }
            Fields::Unnamed(UnnamedFields {
                fields: our_fields,
            })
        }
        RexType::Dict(entries) => {
            let mut our_fields: Vec<NamedField> = Vec::new();
            for (k, t) in entries.iter() {
                our_fields.push(NamedField {
                    n: k.clone(),
                    t: rex_to_our(t),
                });
            }
            Fields::Named(NamedFields {
                fields: our_fields,
            })
        }
        _ => panic!("Unsupported type"), // TODO: give detail
    }
}

pub fn rex_to_our(rex_type: &RexType) -> OurType {
    match rex_type {
        RexType::UnresolvedVar(_) => panic!("Unsupported Rex type: UnresolvedVar"),
        RexType::Var(_) => panic!("Unsupported Rex type: Var"),
        RexType::ForAll(_, _, _) => panic!("Unsupported Rex type: ForAll"),
        RexType::ADT(adt) => {

            let mut our_variants: Vec<UnionVariant> = Vec::new();
            for rex_variant in adt.variants.iter() {

                let fields: Option<Fields> = match &rex_variant.t {
                    Some(rex_fields) => Some(rex_to_fields(rex_fields)),
                    None => None,
                };
                our_variants.push(UnionVariant {
                    n: rex_variant.name.clone(),
                    fields,
                })
            }
            panic!("Unsupported Rex type: ADT")
        }
        RexType::Arrow(_, _) => panic!("Unsupported Rex type: Arrow"),
        RexType::Result(_, _) => panic!("Unsupported Rex type: Result"),
        RexType::Option(t) => OurType::Optional(Optional {
            t: Box::new(rex_to_our(t))
        }),
        RexType::List(t) => OurType::Array(Array::new(rex_to_our(t))),
        RexType::Dict(_) => panic!("Unsupported Rex type: Dict"),
        RexType::Tuple(ts) => {
            let mut our_fields: Vec<UnnamedField> = Vec::new();
            for t in ts.iter() {
                our_fields.push(UnnamedField::new(rex_to_our(t)));
            }
            OurType::Tuple(Tuple {
                fields: UnnamedFields {
                    fields: our_fields,
                },
            })
        }
        RexType::Bool => OurType::Bool,
        RexType::Uint => OurType::U64,
        RexType::Int => OurType::I64,
        RexType::Float => OurType::F64,
        RexType::String => OurType::String,
        RexType::Uuid => panic!("Unsupported Rex type: Uuid"),
        RexType::DateTime => panic!("Unsupported Rex type: DateTime"),
    }
}
