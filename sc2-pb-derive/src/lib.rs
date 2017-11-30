extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate synstructure;

use proc_macro::TokenStream;

mod utils;
mod to_protobuf;
mod from_protobuf;

#[proc_macro_derive(ToProtobuf, attributes(ProtoType, field, OneOf, AttachedTo, DebugThis, Set))]
pub fn derive_to_protobuf(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = to_protobuf::to_protobuf_impl(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}


#[proc_macro_derive(FromProtobuf, attributes(ProtoType, field, OneOf, AttachedTo, DebugThis, Get, name))]
pub fn derive_from_protobuf(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = from_protobuf::from_protobuf_impl(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

#[proc_macro_derive(FromU32)]
pub fn derive_enum_from_u32(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = derive_from_u32_impl(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn derive_from_u32_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let ident = &ast.ident;

    let mut interior_tokens = quote!{};

    // make sure we are attached to an enum
    if let &syn::Body::Enum(ref variants) = &ast.body {
        for var in variants {
            let var_ident = &var.ident;
            if let &Some(syn::ConstExpr::Lit(syn::Lit::Int(val, _))) = &var.discriminant {
                let val = val as u32;
                interior_tokens.append(quote!{
                    #val => Some ( #ident :: #var_ident) ,
                });
            } else {
                panic!("Enum must have a litteral integer descriminat");
            }

        }
    } else {
        panic!("Unable to apply the FromU32 autoderive to something that's not an enum")
    }

    quote! {
        impl FromU32 for #ident {
            fn from_u32(val: u32) -> Option<Self> {
                match val {
                    #interior_tokens
                    _ => None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setter_name() {
        assert_eq!(utils::construct_field_accessor("FooBar", "set"), "set_foo_bar");
        assert_eq!(utils::construct_field_accessor("Bar", "get"), "get_bar");
        assert_eq!(utils::construct_field_accessor("foo", "take"), "take_foo");
    }

    #[test]
    fn test_is_option() {
        let t = syn::parse_type("Option<bool>").unwrap();
        assert!(utils::is_option(&t));

        let t = syn::parse_type("String").unwrap();
        assert!(!utils::is_option(&t));
    }

    #[test]
    fn get_ty() {
        let tokens = syn::parse_path("Option<bool>").unwrap();
        //println!("{:#?}", tokens);
    }

    #[test]
    fn is_protobuf() {
        assert!(utils::is_protobuf_type(&syn::parse_type("Option<protos::ResponseCreateGame_Error>").unwrap()));
        assert!(utils::is_protobuf_type(&syn::parse_type("Vec<protos::ResponseCreateGame_Error>").unwrap()));
        assert!(utils::is_protobuf_type(&syn::parse_type("protos::ResponseCreateGame_Error").unwrap()));

        assert!(utils::is_protobuf_type(&syn::parse_type("Option<sc2::protos::ResponseCreateGame_Error>").unwrap()));
        assert!(utils::is_protobuf_type(&syn::parse_type("Vec<sc2::protos::ResponseCreateGame_Error>").unwrap()));
        assert!(utils::is_protobuf_type(&syn::parse_type("sc2::protos::ResponseCreateGame_Error").unwrap()));


        assert!(!utils::is_protobuf_type(&syn::parse_type("Option<String>").unwrap()));
        assert!(!utils::is_protobuf_type(&syn::parse_type("Vec<String>").unwrap()));
        assert!(!utils::is_protobuf_type(&syn::parse_type("String").unwrap()));
    }

    #[test]
    fn test_get_type_ident() {
        let ty = utils::get_type_ident(&syn::parse_type("u8").unwrap());
        assert_eq!(ty.as_ref(), "u8");

        let ty = utils::get_type_ident(&syn::parse_type("Option<u8>").unwrap());
        assert_eq!(ty.as_ref(), "u8");
    }
}
