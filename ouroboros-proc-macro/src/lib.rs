use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, Variant};

#[proc_macro_derive(TypeInfo)]
pub fn derive_type_info(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let name_as_str = format!("{name}");

    let description = match &ast.data {
        // Struct
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => {
                let fields = named.named.iter().map(|field| {
                    let field_name = format!("{}", field.ident.clone().unwrap());
                    let field_type = describe_type(&field.ty);
                    quote! {
                        fields.push(::ouroboros::NamedField::new(#field_name.to_string(), #field_type));
                    }
                });
                quote!(
                    let mut fields = ::std::vec::Vec::new();
                    #(#fields;)*
                    ::ouroboros::Type::Record(::ouroboros::Record::new(#name_as_str, fields))
                )
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed.iter().map(|field| {
                    let field_type = describe_type(&field.ty);
                    quote! {
                        #field_type
                    }
                });
                quote!(::ouroboros::Type::Tuple(::ouroboros::Tuple([#(#fields,)*].into_iter().collect())))
            }
            Fields::Unit => quote!(
                ::ouroboros::Type::Record(::ouroboros::Record::new_unit(#name_as_str))
            ),
        },

        // Enum
        Data::Enum(data) => {
            let is_enum = data.variants.iter().all(is_enum_variant);

            if is_enum {
                let variants = data.variants.iter().map(|variant| {
                    let variant_name = format!("{}", variant.ident.clone());
                    quote! {
                        ::ouroboros::EnumVariant::new(#variant_name)
                    }
                });
                quote!(::ouroboros::Type::Enum(::ouroboros::Enum::new(#name_as_str, [#(#variants,)*])))
            } else {
                let variants = data.variants.iter().map(|variant| {
                    let variant_name = format!("{}", variant.ident.clone());
                    match &variant.fields {
                        Fields::Unnamed(unnamed) => {
                            let fields = unnamed.unnamed.iter().map(|field| {
                                let field_type = describe_type(&field.ty);
                                quote! {
                                    #field_type
                                }
                            });
                            quote! {
                                ::ouroboros::UnionVariant::with_fields(#variant_name, [#(#fields,)*])
                            }
                        }
                        Fields::Named(named) => {
                            let fields = named.named.iter().map(|field| {
                                let field_name = format!("{}", field.ident.clone().unwrap());
                                let field_type = describe_type(&field.ty);
                                quote! {
                                    (#field_name, #field_type)
                                }
                            });
                            quote! {
                                ::ouroboros::UnionVariant::with_fields(#variant_name, [#(#fields,)*])
                            }
                        }
                        Fields::Unit => quote!{ ::ouroboros::UnionVariant::new(#variant_name) },
                    }
                });
                quote!(::ouroboros::Type::Union(::ouroboros::Union::new(#name_as_str, [#(#variants,)*])))
            }
        }
        _ => panic!("TypeInfo can only be derived for structs and enums"),
    };

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl  #impl_generics ::ouroboros::TypeInfo for #name #ty_generics #where_clause {
            fn tname() -> ::std::string::String {
                #name_as_str.to_string()
            }

            fn t() -> ::ouroboros::Type {
                #description
            }
        }
    };

    TokenStream::from(expanded)
}

fn describe_type(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let ident = &type_path.path.segments.last().unwrap().ident;
            let inner_types = &type_path.path.segments.last().unwrap().arguments;
            quote!(<#ident #inner_types as ::ouroboros::TypeInfo>::t())
        }
        Type::Tuple(tuple) => {
            let inner_types = tuple.elems.iter().map(describe_type);
            quote!(::ouroboros::Type::Tuple(::ouroboros::Tuple([#(#inner_types,)*].into_iter().collect())))
        }
        _ => panic!("Unsupported type"),
    }
}

fn is_enum_variant(variant: &Variant) -> bool {
    matches!(variant.fields, Fields::Unit)
}
