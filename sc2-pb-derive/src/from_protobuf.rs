use super::{syn, quote, utils};

pub fn from_protobuf_impl(ast: &syn::DeriveInput) -> quote::Tokens {
    let debug_this = utils::get_attr(&ast.attrs, "DebugThis").is_some();

    let name = &ast.ident;
    let name_str = ast.ident.as_ref();

    // get name of protobuf type, or if missing, try to guess it from this identifier

    let _proto_type = utils::get_attr(&ast.attrs, "ProtoType").unwrap_or_else(|| name.as_ref().to_owned());

    let prototype = syn::Ident::from(format!("protos::{}", _proto_type));


    let tokens = if let &syn::Body::Struct(syn::VariantData::Struct(ref data)) = &ast.body {
        //println!("=== Implementing FromProtobuf<{}> for {}", proto_type, name);
        let mut interior_tokens = quote::Tokens::new();

        if debug_this {
            println!("{:#?}", data);
        }
        for field in data {
            let field_name = field.ident.as_ref().unwrap_or_else(
                || panic!("Can't extract ident"),
            );
            let field_name_str = field_name.as_ref();
            let field_ty_ident = utils::get_type_ident(&field.ty);
            //println!("{:?}", field.ty);

            let has_func = utils::construct_field_accessor(field_name, "has");

            let is_one_of = utils::get_attr(&field.attrs, "OneOf").is_some();
            let is_option = utils::is_option(&field.ty);
            let is_protobuf = utils::is_protobuf_type(&field.ty);
            let is_get = utils::get_attr(&field.attrs, "Get").is_some();
            let take_func = if is_get {
                utils::construct_field_accessor(field_name, "get")
            } else {
                utils::construct_field_accessor(field_name, "take")
            };


            let b = if is_protobuf {
                quote! {pb . #take_func() }
            } else {
                quote! {FromProtobuf::from_protobuf( pb . #take_func() )? }
            };

            if is_one_of {
                if is_option {
                    interior_tokens.append(quote! {
                    let #field_name = #field_ty_ident :: get_fields(&mut pb);
                });
                } else {
                    interior_tokens.append(quote! {
                    let #field_name = #field_ty_ident :: get_fields(&mut pb).unwrap_or_else(|| panic!("None of the oneof fields of {} were set", #field_name_str));
                });
                }

            } else if is_option {
                // check to see if the message has the field before setting it


                interior_tokens.append(quote! {
                    let #field_name = if pb. #has_func () {
                        Some( #b )
                    } else {
                        None
                    };
                });
            } else {
                interior_tokens.append(quote! {let #field_name = #b;});
            }
        }

        let mut v = Vec::new();

        for field in data {
            let field_name = field.ident.as_ref().unwrap_or_else(
                || panic!("Can't extract ident"),
            );
            v.push(quote! {
                #field_name
            });
        }

        interior_tokens.append(quote! {
            Ok(#name {
                    #( #v ),*
            })
        });


        quote! {
            impl FromProtobuf< #prototype > for #name {

                    #[allow(unused_mut)]
                    fn from_protobuf(mut pb: #prototype) -> Result<Self, failure::Error> {
                            #interior_tokens

                    }

            }
        }
    } else if let &syn::Body::Enum(ref variants) = &ast.body {
        let mut interior_tokens = quote::Tokens::new();

        if let Some(attached_to) = utils::get_attr(&ast.attrs, "AttachedTo") {
            let attached_to = quote::Ident::from(attached_to);

            for variant in variants {
                let var_ident = &variant.ident;
                let has_func = utils::construct_field_accessor(&variant.ident, "has");
                let is_get = utils::get_attr(&variant.attrs, "Get").is_some();
                let take_func = if is_get {
                    utils::construct_field_accessor(&variant.ident, "get")
                } else {
                    utils::construct_field_accessor(&variant.ident, "take")
                };

                if utils::is_protobuf_type(&variant.data.fields()[0].ty) {
                    // the inner type of this variant doesn't need any conversions
                    interior_tokens.append(quote! {
                        if pb . #has_func () {
                            return Some ( #name :: #var_ident ( pb . #take_func() ) )
                        }
                    });
                } else {
                    interior_tokens.append(quote! {
                        if pb . #has_func () {
                            return Some ( #name :: #var_ident ( FromProtobuf::from_protobuf( pb . #take_func() ).unwrap() )
                             )
                        }
                    });
                }
            }
            quote! {
                impl #name {
                    #[allow(unused_mut)]
                    fn get_fields(pb: &mut protos :: #attached_to) -> Option<#name> {

                        #interior_tokens

                        //panic!("Unable to construct {}", #name_str)
                        None

                    }
                }
            }
        } else {
            // a normal enum, where we can inspect the fields in `pb`
            // NB if the variant has a discriminant, this is aslkdfkla

            #[derive(Eq, PartialEq)]
            enum EnumType { Unknown, HasDiscrim, HasFields }
            let mut enum_type = EnumType::Unknown;

            for variant in variants {
                let var_ident = &variant.ident;
                let has_func = utils::construct_field_accessor(&variant.ident, "has");
                let is_get = utils::get_attr(&variant.attrs, "Get").is_some();
                let take_func = if is_get {
                    utils::construct_field_accessor(&variant.ident, "get")
                } else {
                    utils::construct_field_accessor(&variant.ident, "take")
                };

                if let &Some(ref discrim) = &variant.discriminant {
                    if enum_type == EnumType::HasFields { panic!("Unable to support an enum that has both discriminants and fields") }
                    enum_type = EnumType::HasDiscrim;
                    let pb_name = quote::Ident::new(utils::get_attr(&variant.attrs, "name").unwrap_or(variant.ident.as_ref().to_owned()));
                    interior_tokens.append(quote! {
                        #prototype :: #pb_name => #name :: #var_ident,
                    });
                } else {
                    if enum_type == EnumType::HasDiscrim { panic!("Unable to support an enum that has both discriminants and fields") }
                    enum_type = EnumType::HasFields;

                    if utils::is_protobuf_type(&variant.data.fields()[0].ty) {
                        interior_tokens.append(quote! {
                        if pb . #has_func() {
                            return Ok(#name :: #var_ident ( pb . #take_func() ))
                        }
                    });
                    } else {
                        interior_tokens.append(quote! {
                    if pb . #has_func() {
                        return Ok(#name :: #var_ident ( FromProtobuf::from_protobuf(pb . #take_func())? ))
                    }
                    });
                    }
                }
            }

            if enum_type == EnumType::HasFields {
                quote! {
             impl FromProtobuf< #prototype > for #name {

                    #[allow(unused_mut)]
                    fn from_protobuf(mut pb: #prototype) -> Result<Self, failure::Error> {
                            #interior_tokens

                            panic!("Unable to construct {}" , #name_str)
                    }

            }

                }
            } else {
                quote! {
             impl FromProtobuf< #prototype > for #name {

                    #[allow(unused_mut)]
                    fn from_protobuf(pb: #prototype) -> Result<Self, failure::Error> {
                            Ok(match pb {
                                #interior_tokens
                            })
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
