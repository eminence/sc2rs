
use super::{syn, quote, utils};

pub fn to_protobuf_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let debug_this = utils::get_attr(&ast.attrs, "DebugThis").is_some();

    let name = &ast.ident;

    // get name of protobuf type, or if missing, try to guess it from this identifier

    let proto_type = utils::get_attr(&ast.attrs, "ProtoType").unwrap_or_else(|| name.as_ref().to_owned());

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
            let field_ty_ident = utils::get_type_ident(&field.ty);

            let setter_id = utils::construct_field_accessor(field_name, "set");

            let is_one_of = utils::get_attr(&field.attrs, "OneOf").is_some();

            if is_one_of {
                interior_tokens.append(quote! {
                    self.#field_name.set_fields (&mut pb);
                });
            } else {
                if utils::is_option(&field.ty) {
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
                    if utils::is_protobuf_type(&field.ty) {
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
                let field_setter = utils::construct_field_accessor(&variant.ident, "set");
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
