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

#[proc_macro_derive(ToProtobuf, attributes(ProtoType, field, OneOf, AttachedTo, DebugThis))]
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


#[proc_macro_derive(FromProtobuf, attributes(ProtoType, field, OneOf, AttachedTo, DebugThis, Get))]
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
