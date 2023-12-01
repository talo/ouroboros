use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

#[proc_macro_derive(Ouroboros)]
pub fn ouroboros_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let name_as_str = format!("{}", name);

    let python_code = match &ast.data {
        // Derive for a struct
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => {
                let push_record_fields = named.named.iter().map(|field| {
                    let field_name = format!("{}", field.ident.clone().unwrap());
                    let field_type = python(&field.ty);
                    quote! {
                        record_fields.push(format!("{}: {}", #field_name, #field_type));
                    }
                });
                quote!(
                    let mut record_fields = ::std::vec::Vec::new();
                    #(#push_record_fields;)*
                    format!("class {}:\n\n{}\n\n   def __init__(self, {}):\n{}",
                        #name_as_str,
                        record_fields.iter()
                            .map(|f| format!("    {}", f))
                            .collect::<Vec<_>>().join("\n"),
                        record_fields.clone().join(", "),
                        record_fields.iter()
                            .map(|f| {
                                let field_name = f.split(":").next().unwrap();
                                format!("        self.{} = {}", &field_name, &field_name)
                            })
                            .collect::<Vec<_>>().join("\n")
                    )
                )
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed.iter().map(|field| {
                    let field_type = &field.ty;
                    quote! {
                        #field_type
                    }
                });
                quote! {
                    format!("{} = {}", #name_as_str, <(#(#fields,)*) as ::ouroboros::Ouroboros>::python())
                }
            }
            Fields::Unit => panic!("Typedef cannot be derived for unit structs"),
        },
        // Derive for an enum
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|variant| {
                let variant_name = format!("{}", variant.ident.clone());

                match &variant.fields {
                    Fields::Unnamed(_) => {
                        panic!("Ouroboros cannot be derived for unnamed enum variants")
                    }
                    Fields::Named(_) => {
                        panic!("Ouroboros cannot be derived for named enum variants")
                    }
                    Fields::Unit => {}
                };

                quote! {
                    #variant_name.to_string()
                }
            });

            quote! {
                format!("class {}(enum.Enum):\n    {}", #name_as_str, [#(#variants,)*].into_iter().collect::<Vec<_>>().join("\n    "))
            }
        }

        _ => panic!("Typedef can only be derived for structs and enums"),
    };

    let cpp_code = match &ast.data {
        // Derive for a struct
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => {
                let push_record_fields = named.named.iter().map(|field| {
                    let field_name = format!("{}", field.ident.clone().unwrap());
                    let field_type = cpp(&field.ty);
                    quote! {
                        record_fields.push(format!("    {} {};", #field_type, #field_name));
                    }
                });
                quote!(
                    let mut record_fields = ::std::vec::Vec::new();
                    #(#push_record_fields;)*
                    format!("struct {} {{\n{}\n}};",
                        #name_as_str,
                        record_fields.join("\n")
                    )
                )
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed.unnamed.iter().map(|field| {
                    let field_type = &field.ty;
                    quote! { #field_type }
                });
                quote! {
                    format!("using {} = {};",  #name_as_str, <(#(#fields,)*) as ::ouroboros::Ouroboros>::cpp())
                }
            }
            Fields::Unit => panic!("Typedef cannot be derived for unit structs"),
        },
        // Derive for an enum
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|variant| {
                let variant_name = format!("{}", variant.ident.clone());

                match &variant.fields {
                    Fields::Unnamed(_) => {
                        panic!("Ouroboros cannot be derived for unnamed enum variants")
                    }
                    Fields::Named(_) => {
                        panic!("Ouroboros cannot be derived for named enum variants")
                    }
                    Fields::Unit => {}
                };

                quote! {
                    #variant_name.to_string()
                }
            });

            quote! {
                format!("enum {} {{\n    {}\n}};", #name_as_str, [#(#variants,)*].into_iter().collect::<Vec<_>>().join(",\n    "))
            }
        }
        _ => panic!("Ouroboros can only be derived for structs and enums"),
    };

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    proc_macro::TokenStream::from(quote! {
        impl #impl_generics ::ouroboros::Ouroboros for #name #ty_generics #where_clause {
            fn python() -> ::std::string::String {
                if ::ouroboros::is_python_type_name_registered(#name_as_str) {
                    #name_as_str.to_string()
                } else {
                    ::ouroboros::register_python_type_name(#name_as_str.to_string());
                    #python_code
                }
            }

            fn cpp() -> ::std::string::String {
                if ::ouroboros::is_cpp_type_name_registered(#name_as_str) {
                    #name_as_str.to_string()
                } else {
                    ::ouroboros::register_cpp_type_name(#name_as_str.to_string());
                    #cpp_code
                }
            }
        }
    })
}

fn python(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let ident = &type_path.path.segments.last().unwrap().ident;
            let inner_types = &type_path.path.segments.last().unwrap().arguments;
            quote!(<#ident #inner_types as ::ouroboros::Ouroboros>::python())
        }
        Type::Tuple(tuple) => {
            let inner_types = tuple.elems.iter().collect::<Vec<_>>();
            quote!(<(#(#inner_types,)*) as ::ouroboros::Ouroboros>::python())
        }
        _ => panic!("unsupported type"),
    }
}

fn cpp(ty: &Type) -> proc_macro2::TokenStream {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let ident = &type_path.path.segments.last().unwrap().ident;
            let inner_types = &type_path.path.segments.last().unwrap().arguments;
            quote!(<#ident #inner_types as ::ouroboros::Ouroboros>::cpp())
        }
        Type::Tuple(tuple) => {
            let inner_types = tuple.elems.iter().collect::<Vec<_>>();
            quote!(<(#(#inner_types,)*) as ::ouroboros::Ouroboros>::cpp())
        }
        _ => panic!("unsupported type"),
    }
}
