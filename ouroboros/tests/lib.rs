use ouroboros::{transpile::cpp::TypedefVisitor, Tuple, TypeName, UnnamedField};
#[cfg(test)]
use ouroboros::{Enum, EnumVariant, Record, Type, TypeInfo, Union, UnionVariant};

#[derive(Clone, Debug, TypeInfo)]
struct Unit;

#[allow(dead_code)]
#[derive(Clone, Debug, TypeInfo)]
struct Unnamed(u32, Vec<u32>, Option<u32>);

#[allow(dead_code)]
#[derive(Clone, Debug, TypeInfo)]
struct Foo {
    x: u32,
    y: Vec<u32>,
    z: Option<u32>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, TypeInfo)]
struct Gen<T: TypeInfo>(T);

#[allow(dead_code)]
#[derive(Clone, Debug, TypeInfo)]
enum Bar {
    X,
    Y,
}

#[allow(dead_code)]
#[derive(Clone, Debug, TypeInfo)]
enum Baz {
    X,
    Y(u32, Vec<u32>),
    Z { foo: Foo, bar: Bar },
}

#[test]
fn test_unit_record() {
    assert_eq!(
        Unit::tname(),
        TypeName {
            n: "Unit",
            g: vec![]
        }
    );
    assert_eq!(Unit::t(), Type::Record(Record::new_unit("Unit")));
}

#[test]
fn test_unnamed_record() {
    assert_eq!(
        Unnamed::tname(),
        TypeName {
            n: "Unnamed",
            g: vec![]
        }
    );
    assert_eq!(
        Unnamed::t(),
        Type::Record(Record::new(
            "Unnamed",
            [Type::U32, Vec::<u32>::t(), Option::<u32>::t(),].map(UnnamedField::new)
        ))
    );
}

#[test]
fn test_named_record() {
    assert_eq!(
        Foo::tname(),
        TypeName {
            n: "Foo",
            g: vec![]
        }
    );
    assert_eq!(
        Foo::t(),
        Type::Record(Record::new(
            "Foo",
            [
                ("x", Type::U32,),
                ("y", Vec::<u32>::t(),),
                ("z", Option::<u32>::t(),)
            ]
        ))
    );
}

#[test]
fn test_generic_record() {
    assert_eq!(
        Gen::<Foo>::tname(),
        TypeName {
            n: "Gen",
            g: vec![Foo::tname()]
        }
    );
    assert_eq!(
        Gen::<Foo>::t(),
        Type::Tuple(Tuple::new([Foo::t()].map(UnnamedField::new)))
    );
}

#[test]
fn test_enum() {
    assert_eq!(
        Bar::tname(),
        TypeName {
            n: "Bar",
            g: vec![]
        }
    );
    assert_eq!(
        Bar::t(),
        Type::Enum(Enum::new(
            "Bar",
            [EnumVariant::new("X"), EnumVariant::new("Y"),]
        ))
    );
}

#[test]
fn test_union() {
    assert_eq!(
        Baz::tname(),
        TypeName {
            n: "Baz",
            g: vec![]
        }
    );
    assert_eq!(
        Baz::t(),
        Type::Union(Union::new(
            "Baz",
            [
                UnionVariant::new("X"),
                UnionVariant::with_fields("Y", [u32::t(), Vec::<u32>::t()]),
                UnionVariant::with_fields("Z", [("foo", Foo::t()), ("bar", Bar::t())])
            ]
        ))
    );

    println!("{}", TypedefVisitor::visit_type(&Baz::t()));
}
