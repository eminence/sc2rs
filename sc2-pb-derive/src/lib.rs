extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate synstructure;

use proc_macro::TokenStream;


#[proc_macro_derive(ToProtobuf, attributes(ProtoType, field, OneOf, AttachedTo, DebugThis))]
pub fn derive_to_protobuf(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = to_protobuf_impl(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

/// convert a field name like `FooThing` into a name like `set_foo_thing`
fn construct_field_setter_name<T: AsRef<str>>(t: T) -> quote::Ident {
    let s: &str = t.as_ref();
    let mut r = String::with_capacity(s.len());
    r.push_str("set");
    let mut first = true;
    for c in s.chars() {
        if c.is_uppercase() {
            r.push('_');
            r.push(c.to_ascii_lowercase());
        } else {
            if first {
                r.push('_')
            }
            r.push(c);
        }
        first = false;
    }
    quote::Ident::from(r)
}

fn get_type_ident(ty: &syn::Ty) -> quote::Ident {
    if let &syn::Ty::Path(_,
                          syn::Path {
                              ref global,
                              ref segments,
                          }) = ty
        {
            if segments.len() > 0 {
                return quote::Ident::from(segments[0].ident.as_ref());
            }
        }
    panic!("Can't extract ident from ty")
}

fn get_printable_ty(ty: &syn::Ty) -> String {
    use quote::ToTokens;

    let mut t = quote::Tokens::new();
    ty.to_tokens(&mut t);


    format!("{}", t)
}

fn is_option(ty: &syn::Ty) -> bool {
    if let &syn::Ty::Path(_,
                          syn::Path {
                              ref global,
                              ref segments,
                          }) = ty
        {
            // check to see if the first path segment is an Option
            segments.len() > 0 && segments[0].ident.as_ref() == "Option"
        } else {
        false
    }
}

/// Tries to determine if a given type (like Option<protos::Response>) references
/// a protobuf type somewhere.  This works by hard-coding the fact the protobuffer type
/// live in a module named "protos"
fn is_protobuf_type(ty: &syn::Ty) -> bool {
    if let &syn::Ty::Path(_, syn::Path { ref global, ref segments }) = ty {
        segments.iter().any(|segment| {
            segment.ident.as_ref() == "protos" || match &segment.parameters {
                &syn::PathParameters::AngleBracketed(ref data) => {
                    data.types.iter().any(|t| is_protobuf_type(t))
                }
                &syn::PathParameters::Parenthesized(ref data) => {
                    data.inputs.iter().any(|t| is_protobuf_type(t))
                }
            }
        })
    } else {
        false
    }
}

fn is_equal(ty: &syn::Ty, s: &str) -> bool {
    //ty == &syn::Ty::Path(None, syn::parse_path(s).unwrap())
    ty == &syn::parse_type(s).unwrap()
}

fn get_attr(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    let x = attrs
        .iter()
        .filter_map(|attr| {
            if let syn::MetaItem::NameValue(ref id, syn::Lit::Str(ref s, _)) = attr.value {
                if id.as_ref() == name {
                    return Some(s.clone());
                }
            }
            if let syn::MetaItem::List(ref id, ref vec) = attr.value {
                if id.as_ref() == name {
                    if vec.len() > 0 {
                        match &vec[0] {
                            &syn::NestedMetaItem::MetaItem(syn::MetaItem::Word(ref ident)) => {
                                return Some(ident.as_ref().to_owned());
                            }
                            &syn::NestedMetaItem::Literal(syn::Lit::Str(ref s, _)) => {
                                return Some(s.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
            }
            if let syn::MetaItem::Word(ref id) = attr.value {
                if id.as_ref() == name {
                    return Some(String::new());
                }
            }
            None
        })
        .next();
    x
}

fn to_protobuf_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let debug_this = get_attr(&ast.attrs, "DebugThis").is_some();

    let name = &ast.ident;

    // get name of protobuf type, or if missing, try to guess it from this identifier

    let proto_type = get_attr(&ast.attrs, "ProtoType").unwrap_or_else(|| name.as_ref().to_owned());

    let prototype = syn::Ident::from(format!("protos::{}", proto_type));


    let tokens = if let &syn::Body::Struct(syn::VariantData::Struct(ref data)) = &ast.body {
        println!("=== Implementing ToProtobuf<{}> for {}", proto_type, name);
        let mut interior_tokens = quote::Tokens::new();

        if debug_this {
            println!("{:#?}", data);
        }
        for field in data {
            let field_name = field.ident.as_ref().unwrap_or_else(
                || panic!("Can't extract ident"),
            );
            let field_ty_ident = get_type_ident(&field.ty);

            let setter_id = construct_field_setter_name(field_name);

            let is_one_of = get_attr(&field.attrs, "OneOf").is_some();

            if is_one_of {
                interior_tokens.append(quote! {
                    self.#field_name.set_fields (&mut pb);
                });
            } else {
                if is_option(&field.ty) {
                    let b = if is_protobuf_type(&field.ty) {
                        quote! {b}
                    } else {
                        quote! {b.into_protobuf()}
                    };
                    interior_tokens.append(quote! {
                    if let Some(b) = self.#field_name {
                        pb.#setter_id(#b);
                    }
                });
                } else {
                    if is_protobuf_type(&field.ty) {
                        interior_tokens.append(quote! {
                             pb. #setter_id ( self . #field_name );
                        });
                    } else {
                        interior_tokens.append(quote! {
                             pb. #setter_id ( self . #field_name .into_protobuf() );
                        });
                    }
                }
            }
        }

        quote! {
            impl ToProtobuf< #prototype > for #name {

                    fn into_protobuf(self) -> #prototype {
                        let mut pb = #prototype::new();
                        #interior_tokens
                        return pb;
                    }

            }
        }
    } else if let &syn::Body::Enum(ref variants) = &ast.body {
        // When doing codegen for an Enum, there are two different modes:
        // * If an enum has an #[AttachedTo(MyStruct)] attribute, then it doesn't actually
        //   implment ToProtobuf, but instead generates helper code for MyStruct to call
        // * If an enum doesn't have an AttachedTo attribute, then it represents a real
        //   protobuf structure that consists solely of a oneof enum

        println!("=== Implementing helper functions for {}", name);

        let mut interior_tokens = quote::Tokens::new();

        if let Some(attached_to) = get_attr(&ast.attrs, "AttachedTo") {
            let attached_to = quote::Ident::from(attached_to);
            for variant in variants {
                let var_ident = &variant.ident;
                let field_setter = construct_field_setter_name(&variant.ident);
                if is_protobuf_type(&variant.data.fields()[0].ty) {
                    interior_tokens.append(quote! {
                        #name :: #var_ident (v) => { pb . #field_setter ( v) ; }
                    });
                } else {
                    interior_tokens.append(quote! {
                        #name :: #var_ident (v) => { pb . #field_setter ( v . into_protobuf() ) ; }
                    });
                }

            }

            quote! {
                impl #name {
                    fn set_fields(self, pb: &mut protos :: #attached_to) {

                        match self {
                            #interior_tokens
                        }
                    }
                }
            }
        } else {
            // generate a conventional ToProtobuf implementation

            for variant in variants {
                let var_ident = &variant.ident;
                let field_setter = construct_field_setter_name(&variant.ident);
                interior_tokens.append(quote! {
                    #name :: #var_ident (v) => { pb . #field_setter ( v .into_protobuf()) ; }
                });
            }

            quote! {
                impl ToProtobuf< #prototype > for #name {

                        fn into_protobuf(self) -> #prototype {
                            let mut pb = #prototype::new();
                            match self {
                                #interior_tokens
                            }
                            unimplemented!()
                        }

                }
            }
        }
    } else {
        panic!("Can only apply derive(ToProtobuf) to enums and structs");
    };


    println!("=== {}", tokens);
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setter_name() {
        assert_eq!(construct_field_setter_name("FooBar"), "set_foo_bar");
        assert_eq!(construct_field_setter_name("Bar"), "set_bar");
        assert_eq!(construct_field_setter_name("foo"), "set_foo");
    }

    #[test]
    fn test_is_option() {
        let t = syn::parse_type("Option<bool>").unwrap();
        assert!(is_option(&t));

        let t = syn::parse_type("String").unwrap();
        assert!(!is_option(&t));
    }

    #[test]
    fn get_ty() {
        let tokens = syn::parse_path("Option<bool>").unwrap();
        println!("{:#?}", tokens);
    }

    #[test]
    fn is_protobuf() {
        assert!(is_protobuf_type(&syn::parse_type("Option<protos::ResponseCreateGame_Error>").unwrap()));
        assert!(is_protobuf_type(&syn::parse_type("Vec<protos::ResponseCreateGame_Error>").unwrap()));
        assert!(is_protobuf_type(&syn::parse_type("protos::ResponseCreateGame_Error").unwrap()));

        assert!(is_protobuf_type(&syn::parse_type("Option<sc2::protos::ResponseCreateGame_Error>").unwrap()));
        assert!(is_protobuf_type(&syn::parse_type("Vec<sc2::protos::ResponseCreateGame_Error>").unwrap()));
        assert!(is_protobuf_type(&syn::parse_type("sc2::protos::ResponseCreateGame_Error").unwrap()));


        assert!(!is_protobuf_type(&syn::parse_type("Option<String>").unwrap()));
        assert!(!is_protobuf_type(&syn::parse_type("Vec<String>").unwrap()));
        assert!(!is_protobuf_type(&syn::parse_type("String").unwrap()));
    }
}
