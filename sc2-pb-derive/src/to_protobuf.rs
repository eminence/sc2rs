#![allow(unused_mut)]

use super::{syn, quote, utils};

pub fn to_protobuf_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let debug_this = utils::get_attr(&ast.attrs, "DebugThis").is_some();

    // Name of the struct or enum that we're generating code for
    let name = &ast.ident;

    // Normally the struct/enum name is going to match the struct name in the generated protobuf
    // code, but if not, the ProtoType attribute can be used to explicitly give the name of the
    // protobuf struct.
    let proto_type = utils::get_attr(&ast.attrs, "ProtoType").unwrap_or_else(|| name.as_ref().to_owned());

    // Full name of the protobuf type.  Note how the 'protos' module name is hard-coded
    let prototype = syn::Ident::from(format!("protos::{}", proto_type));


    let tokens = if let &syn::Body::Struct(syn::VariantData::Struct(ref data)) = &ast.body {
        // we are doing codegen for a struct

        //println!("=== Implementing ToProtobuf<{}> for {}", proto_type, name);
        let mut interior_tokens = quote::Tokens::new();

        if debug_this {
            println!("{:#?}", data);
        }
        for field in data {
            let field_name = field.ident.as_ref().unwrap_or_else(
                || panic!("Can't extract ident"),
            );
            let field_ty_ident = utils::get_type_ident(&field.ty);

            // name of the setter function in the protobuf struct.  Derived exclusively from the
            // name of the field in our struct
            let setter_id = utils::construct_field_accessor(field_name, "set");

            // If a field in our struct has the #[OneOf] annotation, then its type is an enum
            let is_one_of = utils::get_attr(&field.attrs, "OneOf").is_some();

            if is_one_of {
                // Use the set_fields() helper code (also generated) to call the right setter
                // methods in the protobuf object
                interior_tokens.append(quote! {
                    self.#field_name.set_fields (&mut pb);
                });
            } else {
                if utils::is_option(&field.ty) {
                    // if the field type in our struct looks like a raw protobuf type, then we
                    // don't need to call "into_protobuf" on the value.  otherwise we do.
                    let b = if utils::is_protobuf_type(&field.ty) {
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
                    if utils::is_protobuf_type(&field.ty) || utils::get_attr(&field.attrs, "Set").is_some() {
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

                    #[allow(unused_mut)]
                    fn into_protobuf(self) -> #prototype {
                        let mut pb = #prototype::new();
                        #interior_tokens
                        return pb;
                    }

            }
        }
    } else if let &syn::Body::Enum(ref variants) = &ast.body {
        // When doing codegen for an Enum, there are three different modes:
        // * If an enum has an #[AttachedTo(MyStruct)] attribute, then it doesn't actually
        //   implment ToProtobuf, but instead generates helper code for MyStruct to call
        // * If every
        // * If an enum doesn't have an AttachedTo attribute, then it represents a real
        //   protobuf structure that consists solely of a oneof enum

        //println!("=== Implementing helper functions for {}", name);

        let mut interior_tokens = quote::Tokens::new();

        if let Some(attached_to) = utils::get_attr(&ast.attrs, "AttachedTo") {
            let attached_to = quote::Ident::from(attached_to);
            for variant in variants {
                let var_ident = &variant.ident;
                let field_setter = utils::construct_field_accessor(&variant.ident, "set");
                if utils::is_protobuf_type(&variant.data.fields()[0].ty) {
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
                    #[allow(unused_mut)]
                    fn set_fields(self, pb: &mut protos :: #attached_to) {

                        match self {
                            #interior_tokens
                        }
                    }
                }
            }
        } else {
            // generate a conventional ToProtobuf implementation
            #[derive(Eq, PartialEq)]
            enum EnumType { Unknown, HasDiscrim, HasFields }
            let mut enum_type = EnumType::Unknown;



            for variant in variants {
                let var_ident = &variant.ident;
                let field_setter = utils::construct_field_accessor(&variant.ident, "set");

                if let &Some(ref discrim) = &variant.discriminant {
                    if enum_type == EnumType::HasFields { panic!("Unable to support an enum that has both discriminants and fields") }
                    enum_type = EnumType::HasDiscrim;

                    let pb_name = quote::Ident::new(utils::get_attr(&variant.attrs, "name").unwrap_or(variant.ident.as_ref().to_owned()));

                    interior_tokens.append(quote! {
                        #name :: #var_ident  =>  #prototype :: #pb_name ,
                    });

                } else {
                    if enum_type == EnumType::HasDiscrim { panic!("Unable to support an enum that has both discriminants and fields") }
                    enum_type = EnumType::HasFields;

                    interior_tokens.append(quote! {
                        #name :: #var_ident (v) => { pb . #field_setter ( v .into_protobuf()) ; }
                    });
                }


            }
            if enum_type == EnumType::HasFields {            quote! {
                impl ToProtobuf< #prototype > for #name {

                        #[allow(unused_mut)]
                        fn into_protobuf(self) -> #prototype {
                            let mut pb = #prototype::new();
                            match self {
                                #interior_tokens
                            }
                            pb
                        }

                }
            }
            } else {            quote! {
                impl ToProtobuf< #prototype > for #name {

                        #[allow(unused_mut)]
                        fn into_protobuf(self) -> #prototype {
                            match self {
                                #interior_tokens
                            }

                        }

                }
            }
            }
        }
    } else {
        panic!("Can only apply derive(ToProtobuf) to enums and structs");
    };


    //println!("=== {}", tokens);
    tokens
}
